use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::compiler_fence;
use itertools::Itertools;
use regress::Regex;
use thiserror::Error;
use crate::{grammar, TextmateGrammar};
use crate::grammar::{Captures, Pattern};


#[derive(Debug, Error)]
pub enum ParsePatternError {
    #[error("found end key without begin in pattern: {0:?}")]
    BeginWithoutEnd(Pattern),
    #[error("found begin key without end in pattern: {0:?}")]
    EndWithoutBegin(Pattern),

    #[error("expected a 'match', 'begin' or 'end' key in this pattern: {0:?}")]
    NoMatchBeginEnd(Pattern),

    #[error("pattern has match, begin *and* end keys which is invalid: {0:?}")]
    SurroundAndMatch(Pattern),

    #[error("invalid name")]
    InvalidName,

    #[error("repository entry wasn't a single pattern: {0:?}")]
    InvalidRepositoryEntry(Pattern),

    #[error("failed to compile regex '{1}': {0}")]
    Regex(regress::Error, String)
}

#[derive(Debug)]
pub struct Span {
    start: usize,
    len: usize
}

#[derive(Debug, Clone)]
pub struct Name(Rc<Vec<String>>);

pub struct Capture {
    name: Name,
}

pub enum ParsedPattern {
    Match {
        regex: Regex,
        name: Option<Name>,
        captures: HashMap<usize, Capture>,
    },
    Surround {
        name: Option<Name>,
        content_name: Option<Name>,

        begin: Regex,
        end: Regex,
        begin_captures: HashMap<usize, Capture>,
        end_captures: HashMap<usize, Capture>,
        patterns: Vec<ParsedPattern>
    }
}

fn compile_regex(regex: &str) -> Result<Regex, ParsePatternError> {
    Regex::new(regex)
        .map_err(|e| ParsePatternError::Regex(e, regex.to_string()))
}

fn parse_name(s: String) -> Result<Name, ParsePatternError> {
    Ok(Name(Rc::new(s.split('.').map(|i| i.to_string()).collect())))
}

pub enum CacheItem {
    Unparsed(Pattern),
    Parsed(ParsedPattern)
}

fn parse_include(include: &str, cache: &mut Repository) -> Result<Vec<ParsedPattern>, ParsePatternError> {
    // match  {
    //     None => unimplemented!(), // todo: throw error
    //     Some('#') => {
    //
    //     }
    // }
    todo!()
}

/// TODO: detect cycles
fn parse_pattern(pattern: Pattern, cache: &mut Repository) -> Result<Vec<ParsedPattern>, ParsePatternError> {
    Ok(match (&pattern.begin, &pattern.end, &pattern.match_) {
        (Some(begin), Some(end), None) => {
            vec![ParsedPattern::Surround {
                name: pattern.name.map(parse_name).transpose()?,
                content_name: pattern.content_name.map(parse_name).transpose()?,
                begin: compile_regex(begin)?,
                end: compile_regex(end)?,
                begin_captures: Default::default(),
                end_captures: Default::default(),
                patterns: pattern.patterns
                    .into_iter()
                    .map(|i| parse_pattern(i, cache))
                    .flatten_ok()
                    .collect::<Result<_, _>>()?
                ,
            }]
        }
        (None, None, Some(match_)) => {
            vec![ParsedPattern::Match {
                regex: compile_regex(match_)?,
                name: pattern.name.map(parse_name).transpose()?,
                captures: Default::default(),
            }]
        }
        (None, None, None) => {
            if let Some(i) = pattern.include {
                parse_include(&i, cache)?
            } else if !pattern.patterns.is_empty() {
                pattern.patterns.into_iter().map(|i| parse_pattern(i, cache)).flatten_ok().collect::<Result<_, _>>()?
            } else {
                return Err(ParsePatternError::NoMatchBeginEnd(pattern))
            }
        },
        (None, Some(_), _) => return Err(ParsePatternError::EndWithoutBegin(pattern)),
        (Some(_), None, _) => return Err(ParsePatternError::BeginWithoutEnd(pattern)),
        (Some(_), Some(_), Some(_)) => return Err(ParsePatternError::SurroundAndMatch(pattern))
    })

}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("failed to parse pattern: {0}")]
    Pattern(#[from] ParsePatternError)
}

impl TextmateGrammar {
    pub fn parse(&self, source: &str) -> Result<Tokens, ParseError> {
        let mut parser = Parser::new(source, self)?;
        parser.parse()
    }
}

enum TryPatternStatus<'p> {
    Matched(Vec<Token>),
    MatchedSurround(Vec<Token>, Scope<'p>),
    MatchedEnd(Vec<Token>),
    NoneMatched
}

pub struct Scope<'p> {
    end: &'p Regex,
    original_pattern: &'p ParsedPattern,
    scope_patterns: &'p [ParsedPattern],
}


type Repository<'g> = HashMap<&'g str, CacheItem>;
type Token = (Span, Name);
type Tokens = Vec<Token>;
type ScopeStack<'p> = Vec<Scope<'p>>;

pub struct Parser<'a, 'g> {
    source: &'a str,
    grammar: &'g TextmateGrammar,
    repository: Repository<'g>
}

impl<'a, 'g> Parser<'a, 'g> {
    pub fn new(source: &'a str, grammar: &'g TextmateGrammar) -> Result<Self, ParseError> {
        let mut repository = Repository::new();
        for (name, pattern) in &grammar.grammar.repository {
            repository.insert(name.as_str(), CacheItem::Unparsed(pattern.clone()));
        }

        for (name, pattern) in &grammar.grammar.repository {
            let mut pat =  parse_pattern(pattern.clone(), &mut repository)?;
            if pat.len() != 1 {
                return Err(ParseError::Pattern(ParsePatternError::InvalidRepositoryEntry(pattern.clone())))
            }

            repository.insert(name.as_str(), CacheItem::Parsed(pat.swap_remove(0)));
        }

        Ok(Self {
            source,
            grammar,
            repository,
        })
    }

    pub fn parse(&mut self) -> Result<Tokens, ParseError> {
        let patterns = self.grammar.grammar.patterns
            .iter()
            .map(|i| parse_pattern(i.clone(), &mut self.repository))
            .flatten_ok()
            .collect::<Result<Vec<_>, _>>()?;

        self.parse_with_patterns(patterns.as_slice())
    }

    fn patterns_in_scope<'x, 'p>(&self, scope_stack: &'x ScopeStack<'p>) -> impl Iterator<Item=&'p ParsedPattern> + 'x
    {
        scope_stack
            .iter()
            .rev()
            .map(|i| i.scope_patterns)
            .flatten()
    }

    fn try_patterns<'p>(&self, patterns: &'p [ParsedPattern], token: &str, span: Span, scope_stack: &ScopeStack<'p>) -> TryPatternStatus<'p> {
        let patterns = patterns.iter()
            .chain(self.patterns_in_scope(scope_stack));

        for i in patterns {
            match i {
                ParsedPattern::Match { regex, name, captures } => {
                    if let Some(i) = regex.find(token) {
                        let mut tokens = Tokens::new();

                        if let Some(name) = name {
                            tokens.push((span, name.clone()))
                        }

                        return TryPatternStatus::Matched(tokens)
                    }
                }
                ParsedPattern::Surround {
                    name, content_name,
                    begin, end,
                    begin_captures, end_captures,
                    patterns
                } => {
                    let mut tokens = Tokens::new();
                    let scope = Scope {
                        end,
                        original_pattern: &i,
                        scope_patterns: &patterns,
                    };

                    return TryPatternStatus::MatchedSurround(tokens, scope)
                }
            }
        }

        TryPatternStatus::NoneMatched
    }

    fn parse_line<'p>(&mut self, line: &str, patterns: &'p [ParsedPattern], tokens: &mut Tokens, scope_stack: &mut ScopeStack<'p>) {
        let mut token_start = 0;

        for (c, _) in line.char_indices() {
            let span = Span {
                start: token_start,
                len: c - token_start,
            };
            let token = &line[token_start..c];

            match self.try_patterns(patterns, token, span, scope_stack) {
                TryPatternStatus::Matched(new_tokens) => {
                    tokens.extend(new_tokens);
                    token_start = c;
                }
                TryPatternStatus::MatchedSurround(new_tokens, end) => {
                    tokens.extend(new_tokens);
                    token_start = c;
                }
                TryPatternStatus::MatchedEnd(_) => {}
                TryPatternStatus::NoneMatched => {}
            }
        }
    }

    fn parse_with_patterns(&mut self, patterns: &[ParsedPattern]) -> Result<Tokens, ParseError> {
        let mut scope_stack = Vec::<Scope>::new();
        let mut tokens = Tokens::new();

        for line in self.source.lines() {
            self.parse_line(line, patterns, &mut tokens, &mut scope_stack)
        }

        Ok(tokens)
    }
}
