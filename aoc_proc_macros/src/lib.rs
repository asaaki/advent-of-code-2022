use proc_macro::TokenStream;
use std::str::FromStr;

const DAYS: std::ops::RangeInclusive<usize> = 0..=25;

#[proc_macro]
pub fn day_str_values(_: TokenStream) -> TokenStream {
    let days = DAYS
        .map(|d| format!("\"{}\"", d))
        .collect::<Vec<String>>()
        .join(", ");
    let code = format!("&[{}]", days);
    TokenStream::from_str(&code).unwrap()
}
