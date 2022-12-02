#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

mod cli;
mod consts;

use cli::Args;
use reqwest::{cookie::Jar, Url};
use std::{
    fmt::Display,
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use consts::*;

type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type NullResult = GenericResult<()>;

pub fn _debug_days_and_parts() {
    eprintln!("Allowed days: {DAY_VALUES:?}");
    eprintln!("Allowed parts: {PART_VALUES:?}");
}

pub fn prepare_or_args(bin: &str) -> GenericResult<Args> {
    let mut args = cli::args();
    if args.prepare {
        make_input_dir()?;
        make_input_files(bin)?;
        write_input(bin)?;
        eprintln!(
            "Input files prepared. Fill them with data and rerun program."
        );
        std::process::exit(0);
    } else {
        read_input(bin, &mut args)?;
        Ok(args)
    }
}

fn make_input_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(INPUT_DIR)
}

fn make_input_files(bin: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(INPUT_DIR)?;
    for f in input_files(bin) {
        OpenOptions::new().create(true).write(true).open(f)?;
    }
    Ok(())
}

fn input_files(bin: &str) -> [String; 2] {
    [
        format!("{INPUT_DIR}/{bin}.txt"),
        format!("{INPUT_DIR}/{bin}_scratchpad.txt"),
    ]
}

fn day(bin: &str) -> &str {
    bin.strip_prefix("day").expect("bin should start with day")
}

fn write_input(bin: &str) -> NullResult {
    let day = day(bin);
    let data = fetch_input(day)?;
    let file = &input_files(bin)[0][..]; // infallible, always has 3 elements
    let mut file = OpenOptions::new().write(true).truncate(true).open(file)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn fetch_input(day: &str) -> GenericResult<String> {
    let session = std::fs::read_to_string(".session")?;
    let cookie = format!("session={session}; Domain=.{AOC_DOMAIN}");
    let url = format!("https://{AOC_DOMAIN}").parse::<Url>()?;

    let cookie_store = Jar::default();
    cookie_store.add_cookie_str(&cookie, &url);

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .cookie_store(true)
        .cookie_provider(cookie_store.into())
        .build()?;

    let day_url = format!("https://{AOC_DOMAIN}/{AOC_YEAR}/day/{day}/input");
    let response = client.get(&day_url).send()?.text()?;
    Ok(response)
}

fn read_input(bin: &str, args: &mut Args) -> NullResult {
    let file = &input_files(bin)[args.example as usize][..]; // infallible, always has 3 elements
    let file = OpenOptions::new().read(true).open(file)?;
    let mut input: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    if args.example {
        let test2 = input.pop().unwrap();
        let test1 = input.pop().unwrap();
        args.expected = Some([test1, test2]);
    };
    args.input = input;
    Ok(())
}

#[inline]
pub fn part(args: &Args) -> usize {
    args.second as _
}

pub fn example_output<T: Display>(args: &Args, solution: T) {
    let expected = args
        .expected
        .as_ref()
        .and_then(|o| o.get(part(args)))
        .unwrap();
    println!("??? E {expected} == S {solution}");
}
