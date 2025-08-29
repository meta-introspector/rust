use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Path to the build configuration file
    #[arg(short, long)]
    pub config: Option<String>,
    /// Path to the Git repository to analyze
    pub repo_path: String,
    /// Clean the build directory
    #[arg(long)]
    pub clean: bool,
}