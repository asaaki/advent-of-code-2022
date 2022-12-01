use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
            ArgGroup::new("part or prepare")
                //.required(true)
                .args(["second", "prepare"]),
        ))]
pub struct Args {
    /// Should the second part be solved? (default is to solve first)
    #[arg(short, long)]
    pub second: bool,

    /// Run in example mode (uses scratchpad input data)
    #[arg(short, long)]
    pub example: bool,

    /// Prepare the day (generates files and fetches input data)
    #[arg(short = 'P', long)]
    pub prepare: bool,

    #[arg(skip)]
    pub input: Vec<String>,

    #[arg(skip)]
    pub expected: Option<[String; 2]>,
}

pub fn args() -> Args {
    Args::parse()
}
