use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(name = "json_cli", version = "1.0", author = "George Liao")]
#[command(
    about = "Query Ubuntu cloud image info",
    group(
        ArgGroup::new("mode")
            .args(&["lts", "releases", "sha256_release"])
            .required(true)
            .multiple(false) // ensures only one can be used
    )
)]
struct Cli {
    /// get current ubuntu LTS
    #[arg(short, long)]
    lts: bool,
    /// get supported ubuntu releases
    #[arg(short, long)]
    releases: bool,
    /// get the shar256 of a disk1_img of a release
    #[arg(short, long)]
    sha256_release: Option<String>,
}

fn main() {
    let cli: Cli = Cli::parse();

    if cli.lts {
        println!("Getting current LTS...");
    } else if cli.releases {
        println!("Listing all supported releases...");
    } else if let Some(release) = cli.sha256_release {
        println!("Getting sha256 for release: {release}");
    } else {
        eprintln!("No valid mode specified.");
        std::process::exit(1);
    }
}
