#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

mod cli;
mod consts;

use cli::Args;
use fs_err::OpenOptions;
use include_dir::{include_dir, Dir};
use reqwest::{cookie::Jar, Url};
use std::{
    fmt::Display,
    io::{BufRead, Cursor, Write},
    time::Duration,
};

use cli::prepare_args;
use consts::*;

pub use std::time::Instant;

type GenericResult<T> = color_eyre::Result<T>;
pub type NullResult = GenericResult<()>;

pub static INPUTS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/inputs");

pub fn args(bin: &str) -> GenericResult<Args> {
    color_eyre::install()?;

    let mut args = cli::args();
    args.day = day(bin).parse().unwrap();
    read_input(bin, &mut args)?;
    Ok(args)
}

pub fn prepare_cli() -> NullResult {
    color_eyre::install()?;

    let args = prepare_args();
    let day = format!("day{}", args.day);

    make_input_dir()?;
    make_input_files(&day)?;
    write_input(&day)?;

    println!("Input files prepared. Fill them with data and rerun program.");
    Ok(())
}

fn make_input_dir() -> NullResult {
    fs_err::create_dir_all(INPUT_DIR).map_err(From::from)
}

fn make_input_files(bin: &str) -> NullResult {
    fs_err::create_dir_all(INPUT_DIR)?;
    for f in input_files(bin) {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{INPUT_DIR}/{f}"))?;
    }
    Ok(())
}

fn input_files(bin: &str) -> [String; 2] {
    [format!("{bin}.txt"), format!("{bin}_scratchpad.txt")]
}

fn day(bin: &str) -> &str {
    bin.strip_prefix("day").expect("bin should start with day")
}

fn write_input(bin: &str) -> NullResult {
    let day = day(bin);
    let data = fetch_input(day)?;
    let file = format!("{INPUT_DIR}/{}", &input_files(bin)[0][..]);
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
    let name = &input_files(bin)[args.example as usize][..];
    let file = INPUTS.get_file(name).unwrap().contents_utf8().unwrap();

    let mut input: Vec<String> =
        Cursor::new(file).lines().filter_map(Result::ok).collect();
    if args.example {
        let test2 = input.pop().unwrap();
        let test1 = input.pop().unwrap();
        args.expected = Some([test1, test2]);
        args.input = input.join("\n").into();
    } else {
        args.input = file.into();
    }
    Ok(())
}

#[inline]
pub fn part(args: &Args) -> usize {
    args.second as _
}

#[inline]
fn example_output<T: Display>(args: &Args, solution: T) {
    let expected = args
        .expected
        .as_ref()
        .and_then(|o| o.get(part(args)))
        .unwrap();
    println!("??? E {expected} == S {solution}");
}

#[inline]
pub fn result<T: Display>(
    solution: T,
    elapsed: Duration,
    args: &Args,
) -> NullResult {
    let day = args.day;
    let part = if args.second { 2 } else { 1 };
    if args.example {
        example_output(args, solution);
    } else {
        // [D##.#] solution:  ABCD
        //         solved in: 123.4µs
        println!(
            "[D{day:02}.{part}] solution: {solution}\n        solved in {elapsed:?}"
        );
    };
    Ok(())
}
