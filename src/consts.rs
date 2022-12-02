pub(crate) const INPUT_DIR: &str = "inputs";

pub(crate) const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (rust; 2022; gh: @asaaki)"
);

pub(crate) const AOC_DOMAIN: &str = "adventofcode.com";

pub(crate) const AOC_YEAR: &str = "2022";

pub(crate) const DAY_VALUES: &[&str; 26] = aom::day_str_values!();

pub(crate) const PART_VALUES: &[&str; 2] = &["1", "2"];
