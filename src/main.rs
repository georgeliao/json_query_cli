use clap::{Parser};

#[derive(Parser)]
#[command(name = "json_cli", version = "1.0", author = "George Liao")]
struct Cli {
    /// Activate verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Input arg
    #[arg(short, long)]
    input: String,
}

fn main() {
    // let cli = Cli::parse();
    let cli: Cli = Cli::parse();
    if cli.verbose {
        println!("Verbose mode is on");
    }
    println!("Input arg: {}", cli.input);
}