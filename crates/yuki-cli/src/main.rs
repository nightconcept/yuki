use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use yuki_core::commands;

#[derive(Parser)]
#[command(
    name = "yuki",
    about = "Declarative system configuration manager for Windows",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(global = true, long, env = "YUKI_LOG", default_value = "warn")]
    log_level: String,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Apply system configuration from a manifest")]
    Apply {
        #[arg(help = "Path to the YAML manifest file")]
        manifest: PathBuf,

        #[arg(long, help = "Perform a dry run without making changes")]
        dry_run: bool,

        #[arg(long, help = "Continue on errors")]
        force: bool,
    },

    #[command(about = "List installed packages across all package managers")]
    List {
        #[arg(long, help = "Filter by package manager (scoop, chocolatey, winget)")]
        pm: Option<String>,

        #[arg(long, help = "Output format (table, json, yaml)", default_value = "table")]
        format: String,
    },

    #[command(about = "Update packages")]
    Update {
        #[arg(long, help = "Update all packages across all package managers")]
        all: bool,

        #[arg(long, help = "Specific package manager to update")]
        pm: Option<String>,

        #[arg(long, help = "Perform a dry run without making changes")]
        dry_run: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(&cli.log_level)
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    debug!("yuki v{}", env!("CARGO_PKG_VERSION"));
    debug!("Command: {:?}", std::env::args().collect::<Vec<_>>());

    match cli.command {
        Commands::Apply { manifest, dry_run, force } => {
            commands::apply::run(manifest, dry_run, force).await?;
        }
        Commands::List { pm, format } => {
            commands::list::run(pm, format).await?;
        }
        Commands::Update { all, pm, dry_run } => {
            commands::update::run(all, pm, dry_run).await?;
        }
    }

    Ok(())
}