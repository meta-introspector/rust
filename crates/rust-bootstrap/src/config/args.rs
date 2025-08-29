use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    

    #[arg(long)]
    pub config: Option<String>,

    #[arg(long)]
    pub build_dir: Option<String>,

    #[arg(long)]
    pub build: Option<String>,

    #[arg(long, default_value = "auto")]
    pub color: String,

    #[arg(long)]
    pub clean: bool,

    #[arg(long)]
    pub json_output: bool,

    #[arg(long, default_value = "default")]
    pub warnings: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
