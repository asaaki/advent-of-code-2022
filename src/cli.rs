use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(skip)]
    pub day: u8,

    /// Should the second part be solved? (default is to solve first)
    #[arg(short, long)]
    pub second: bool,

    /// Run in example mode (uses scratchpad input data)
    #[arg(short, long)]
    pub example: bool,

    #[cfg(target_pointer_width = "64")]
    #[arg(skip)]
    pub input: beef::lean::Cow<'static, str>,
    #[cfg(not(target_pointer_width = "64"))]
    #[arg(skip)]
    pub input: beef::Cow<'static, str>,

    #[arg(skip)]
    pub expected: Option<[String; 2]>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
pub struct Prepare {
    /// Which day to prepare (1–25)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    pub day: u8,
}

pub fn args() -> Args {
    Args::parse()
}

pub fn prepare_args() -> Prepare {
    Prepare::parse()
}
