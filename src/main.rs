use backend::JsonProcessor;
use backend::UbuntuImageJsonProcessor;
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
    /// get the shar256 of a disk1_img of the latest version of a release
    #[arg(short, long)]
    sha256_release: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();
    // TODO, mock the class so we can test the logic in main
    let json_processor: Box<dyn JsonProcessor> = Box::new(UbuntuImageJsonProcessor::new()?);

    if cli.lts {
        println!(
            "Getting current LTS \n{}",
            json_processor.get_current_ubuntu_lts().ok_or("Could not get current LTS version")?
        );
    } else if cli.releases {
        let supported_releases = json_processor
            .get_supported_ubuntu_releases()
            .ok_or("Could not get the supported releases")?;
        println!("Listing all supported releases...");
        for item in &supported_releases {
            println!("{}", item);
        }
    } else if let Some(release_version) = cli.sha256_release {
        println!(
            "Getting sha256 for release: {release_version}\n\n{}",
            json_processor
                .get_disk1_img_sha256_of_release(&release_version)
                .ok_or("Could not get the sha256 for the given release")?
        );
    } else {
        eprintln!("No valid mode specified.");
    }

    Ok(())
}
