use aoc_lib::*;

const BIN: &str = env!("CARGO_BIN_NAME");

fn main() -> NullResult {
    let args = prepare_or_args(BIN)?;
    dbg!(&args);
    if !args.second {
        todo!("no part 1")
    } else {
        todo!("no part 2")
    }
    Ok(())
}
