use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
#[command(arg_required_else_help=true)]
#[command(infer_subcommands=true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Analyse {

    },
    Generate {

    },
}

fn main() {
    let c: Cli = Cli::parse();
}
