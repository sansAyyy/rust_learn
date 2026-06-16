use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "mini-cli")]
#[command(about = "A small CLI example built with clap")]
struct Cli {
    /// Print extra information.
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Greet a person.
    Greet {
        name: String,

        /// Convert the greeting to uppercase.
        #[arg(short, long)]
        shout: bool,
    },
    /// Add two numbers.
    Add { left: i32, right: i32 },
}

fn main() {
    // 示例：
    // cargo run --bin 01_cli_with_clap -- greet Alice --shout
    // cargo run --bin 01_cli_with_clap -- --verbose add 2 3
    let cli = Cli::parse();

    if cli.verbose {
        eprintln!("parsed args: {cli:?}");
    }

    match cli.command {
        Commands::Greet { name, shout } => {
            let greeting = format!("Hello, {name}!");
            if shout {
                println!("{}", greeting.to_uppercase());
            } else {
                println!("{greeting}");
            }
        }
        Commands::Add { left, right } => {
            println!("{left} + {right} = {}", left + right);
        }
    }
}
