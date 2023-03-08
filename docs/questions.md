
# Questions

title: A generic format for code viewer services

TODO: talk to Hendrik van Antwerpen about Github
TODO: Peter Mosses

## Random Ideas
store in section like DebugInfo, as better debuginfo basically
Useful for debuggers? Maybe, though at least I mostly debug in my IDE which supports most of these features already. Although, spoofax + debuggers for DSLs?
LSP proxy (caching for the next run)
open code map (openstreetmap)
highlighting spans and linking

Filters / Layers

## Applications for code viewer services

* Displaying code on websites:  
  * for debugging purposes
  * for educational purposes
* Displaying code in editors:
  * "Presentation mode" in editors like intellij
  * Quicker (cached) startup for LSP-like services
  * 

## Existing code navigation and inspection tools:

Look at:
* Dynamic editor services
  * LSPs
      * Rust-analyzer
  * Jetbrains
  * Language workbenches
    * Spoofax
* Bootlin https://github.com/bootlin/elixir
  * LXR (linux cross referencer)
* GitHub code navigation
* Rustdoc
    * And other documentation viewers
* (universal) ctags https://docs.ctags.io/en/latest/output-format.html
* treesitter
* textmate
* KDevelop
* TODO: existing code viewers?
  * Highlight.js
  * Prism
  * Shiki
  * What does github use?

### How to fairly compare these existing tools
Evaluation matrix

### Do existing comparisons already exist

* Some about the usefulness of LSPs for DSLs (bunder2019decoupling, bunder2020towards)
* 

### Comparison questions:

TODO: Make an evaluation matrix out of this
#### How do they get the information necessary to perform their function?
#### What are their limitations?
* Do they generalize to multiple languages
* Can they be incrementally updated

#### How do they store their information

* Rustdoc:
  * <https://rustc-dev-guide.rust-lang.org/rustdoc.html>, <https://github.com/rust-lang/rust/tree/master/src/librustdoc>
  * Directly outputs HTML from walking the compiler's internal format
  * Internal format is queried using the same query system used to resolve types/traits/etc. Pretty much statix
* Bootlin:
  * Has an interesting problem: the linux tree is huge, has to interact with the linux git repo constantly
    * Indexing reportedly takes 15 hours
  * Indexes raw git blobs
  * Uses Berkley KV-database to store data
    * TODO: figure out their layout
  * Uses universal ctags for information
* Ctags:
  * Has parsers for every programming language imaginable it seems
  * Parsers only find identifiers, and link them. The rest of the source may be poorly parsed
  * Outputs a custom format:
    * ctags -R -o out.xref --output-format=xref src
    * 'xref' file seems most useful
    * built around vi scripts. Part of the format is basically executable code
  * Different things may be indexed for different languages depending on the parser
  * Sorts tags for stability
* Textmate
  * Has their own parsers etc. for many languages (called bundles)
  * Does the interesting thing with css-like classes for syntax categories
* KDevelop 
  * apparently does semantic highlighting pretty well

## What should the new tool do exactly?
### Where are code viewers useful


### What features are worth it (especially besides gotos! the difference seems like what I can contribute)
Think about 
* Goto file/usage/reference/implementation/etc (generalize over gotos?)
* Diagnostics?
* Semantic highlighting
  * https://zwabel.wordpress.com/2009/01/08/c-ide-evolution-from-syntax-highlighting-to-semantic-highlighting/
  * https://medium.com/@evnbr/coding-in-color-3a6db2743a1e
    * Different semantic highlightings (for example, for data flow)
  * Semantic highlighting in LSP proposal: https://github.com/microsoft/vscode-languageserver-node/pull/367
    * Some considerations: https://github.com/microsoft/vscode-languageserver-node/pull/367#issuecomment-403085124
    * Status: Dead?
  * Papers on semantic highlighting (to read):
    * benefits of semantic highlighting: https://fileadmin.cs.lth.se/cs/Education/edan70/CompilerProjects/2021/Reports/ringstrom-frisk.pdf
    * effects of code representation in general: https://dl-acm-org.tudelft.idm.oclc.org/doi/pdf/10.1145/2846680.2846685
      * "Secondary notation"
    * "C++ IDE Evolution: From Syntax High-lighting to Semantic Highlighting"
    * 

From erdweg2013state: On page 5 a diagram is given of important features of a language workbench. Some of those
features are important editor services. pelsmaeker2018portable references these and lists 13 derived from this diagram

1. syntax coloring 
2. code folding
3. code completion
4. structure outline
5. symbol navigation
6. hover documentation (called reference resolution in erdweg2013state)
7. signature help (displaying function signatures while typing that functions name)
8. automatic formatting
9. rename refactoring
10. code actions
11. diagnostic messages
12. debugging
13. testing

So which of these are useful for just code viewing? (so not interactive)

1. syntax coloring: Yes
2. code folding: Yes
   * In relation to teaching maybe even to hide away portions of code by default that are not related to the learning goal
3. code completion: No
4. structure outline: Yes
5. symbol navigation: Yes
6. hover documentation: Yes
7. signature help: Maybe. Seeing a function signature when seeing a usage of that function may be useful even when you're not typing
8. automatic formatting: Likely not
   * Only thing I can think of is getting warning when code is not formatted properly, maybe
9. rename refactoring: No
10. code actions: Some
  * Generate/rewrite code: No
  * Show expansions (macros, types, etc): Yes
11. diagnostic messages: Maybe
12. debugging: No
13. testing: No

Though to this list I would like to add a feature I personally think is very useful while reading code is inlay hints (code lens?). Especially in languages which support inference. It's useful to see what a language has inferred.

Separate but similar features:
* inlay hints in code
* hits between code (committer, etc)
* previews of code somewhere else

That leaves the following list of 8 useful code reader features, excluding formatting for now since its usecase seems very small:
1. syntax coloring and semantic highlighting
2. code folding
3. structure outline
4. symbol navigation
5. hover documentation
6. signature help
7. Show expansions (macros, types, etc)
8. diagnostic messages
9. Inlay hints (etc.)

### What information is necessary to provide these features

1. syntax coloring and semantic highlighting
   * token categorization
   * CSS class-like system
2. code folding
   * scoping/block information, basically the AST, and which nodes can and cannot be folded away
   * spans of foldable blocks
3. structure outline
   * List of "top level" symbols, scoping information
4. symbol navigation
   * some sort of symbol graph like, a bit like scope graphs 
5. hover documentation
   * Documentation on *every* item visible 
6. signature help
   * type signature at definition site
   * scope graph to find definition site
7. Show expansions (macros, types, etc)
   * pre-expanded versions of every expandable item
8. diagnostic messages
   * List of diagnostic messages with their span

#### What information sources can be combined?

1. syntax coloring and semantic highlighting: class for each token
2. code folding: list of foldable spans
3. structure outline: might be useful to be a separate list, but could be derived from syntax coloring information
4. symbol navigation: "list of references at span" (so also an annotation)
5. hover documentation: "annotation at span"
6. signature help: "annotation at span"
7. Show expansions (macros, types, etc): "annotation at span"
8. diagnostic messages: "annotation at span"

Everything is in the format of span: metadata, where metadata is often just text that should be interpreted in a certain way. Sometimes its a reference, or list of references,
and for syntax highlighting its usually a classification of a token.

```rust
struct Ref;
struct Span;
struct TokenClass;

enum Class {
    Documentation,
    Type,
    // etc
}

struct Entry {
    span: Span,
    data: Data,
}

enum Data {
    Refs(Vec<Ref>),
    Syntax(TokenClass),
    Info {
        classes: Vec<Class>,
        data: String,
    },
}
```

### How is this tool different from all the analysed existing tools
### Are there any features not worth pursuing
for example: Takes too much storage space for example

## How should this tool store information (see 'Existing code navigation' results)
### What information should be stored
### How can diffs be minimized on updates?

## What exactly is the difference between what I'm doing (statically precomputing) and what an LSP is doing
### What are the advantages and limitations of that?


## At what granularity should analyses be done?
* Analyse single functions
* Analyse single files
* Analyse whole projects
* What's useful for whom?

## Integration

### How can this integrate with LSPs
### How can this integrate with existing code highlighters (semantic highlighting)
### With CTAGS/XREFs/etc
### How can this integrate with websites

## Incremental

What's a "live-marker"?? LSPs seem to use it to keep things in sync across client and server
https://github.com/microsoft/vscode-languageserver-node/pull/367#issuecomment-403085124
Maybe not actually called live-marker

### Are there applications for which this makes sense?
### If so, how can it be done?

## How can I evaluate whether I answered/solved each of these questions?
### What questions have not been answered yet, why?

## What name should this tool have, so I don't have to call it 'this tool' all the time

papers:
about editor services / LSP
different storage formats, are there speed advantages to some?

twee evaluatiepunten
2 maanden
green light (6 weken voor eind)


aannames om te versimpelen
lijst van wat we willen
wat doen bestaande tools hier al van



