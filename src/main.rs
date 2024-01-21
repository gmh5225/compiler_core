use clap::Parser;
use compiler_core::compiler;

/// Builder Language Compiler and Runner
#[derive(Parser, Debug)] // this is not the parser we made
#[clap(author="Union College", version="0.1.0", about="Builder Language Compiler and Runner")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Compile {
        file: String,
    },
    Run {
        file: String,
    },
}

fn main() {
    let builder: Cli = Cli::parse();

    match &builder.command {
        Commands::Compile { file } => {
            let _ = compiler::compile(file);
        },
        Commands::Run { file } => {
            unimplemented!("Running unimplemented")
        }
    }
}
