use std::collections::HashMap;
use clap::{Parser, Subcommand, ValueEnum};
use code_exploration_services_lib::output::simple_html::SimpleHtml;
use code_exploration_services_lib::{Analysis, Annotater, SourceCode};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about)]
#[command(arg_required_else_help = true)]
#[command(infer_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Output {
    // convert to html
    SimpleHtml,
}

#[derive(Subcommand)]
enum Commands {
    Analyse {
        /// source file to analyse
        #[arg(long, short)]
        file: PathBuf,
        /// where to output the analysis (`-` means stdout)
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
    Generate {
        #[arg(value_enum)]
        output_type: Output,

        #[arg(long, short)]
        file: PathBuf,
        #[arg(long, short)]
        analysis: Option<PathBuf>,
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let c: Cli = Cli::parse();
    match c.command {
        Commands::Analyse { file, output } => {
            let source = SourceCode::from_path(file)?;
            let result = code_exploration_services_lib::input::analyse(&source)?;
            let serialized = result.serialize();

            if output.is_none() || output.as_ref().unwrap() == Path::new("-") {
                stdout().write_all(&serialized)?;
            } else if let Some(output) = output {
                File::create(output)?.write_all(&serialized)?;
            }
        }
        Commands::Generate {
            file,
            analysis,
            output,
            output_type,
        } => {
            let source = SourceCode::from_path(file)?;

            let analysis = if let Some(analysis) = analysis {
                let string_analysis = if analysis == Path::new("-") {
                    let mut buf = Vec::new();
                    stdin().read_to_end(&mut buf)?;
                    buf
                } else {
                    std::fs::read(analysis)?
                };

                Analysis::deserialize(&string_analysis)
            } else {
                code_exploration_services_lib::input::analyse(&source)?
            };

            match output_type {
                Output::SimpleHtml => {
                    let res = SimpleHtml.annotate(&source, analysis)?;

                    if let Some(output) = output {
                        std::fs::write(output, res)?;
                    } else {
                        stdout().write_all(res.as_bytes())?;
                    }
                }
            };
        }
    }

    Ok(())
}
