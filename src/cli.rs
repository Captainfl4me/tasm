use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Assemble(AssembleArgs),
}

#[derive(Args)]
pub struct AssembleArgs {
    pub source: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long)]
    pub coe: bool,
}
