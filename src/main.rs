use clap::Parser;
use clap::Subcommand;
use skel::SKEL_DIR;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let skel_dir = home::home_dir().unwrap().join(SKEL_DIR);
    let skel_dir = skel_dir.to_str().unwrap().to_string();
    match args.command {
        Commands::Create {
            skel_name,
            target_dir
        } => {
            skel::cmd::create::run(skel_dir, skel_name, target_dir).await?;
        }
        Commands::Sync => {
            skel::cmd::sync::run(skel_dir).await?;
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[command(about, long_about = None, version)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create project from skeleton
    Create {
        /// A skeleton name
        skel_name: String,
        /// A name of target directory
        target_dir: String
    },
    /// Sync skeletons from GitHub
    Sync
}
