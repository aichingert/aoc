extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

use std::time::SystemTime;

mod gen;
use gen::{imports, functions};

const BASE_DIR: &'static str = "advent/src/";
const SOLUTION: &'static str = "solve";

#[proc_macro]
pub fn add_modules(item: TokenStream) -> TokenStream {
    let tokens = item.into_iter().collect::<Vec<TokenTree>>();
    imports::add_days_of_year(&tokens[..]).join("\n").parse().unwrap()
}

#[proc_macro]
pub fn add_years(_item: TokenStream) -> TokenStream {
    imports::add_years().join("\n").parse().unwrap()
}

#[proc_macro]
pub fn add_fn_pointers(_item: TokenStream) -> TokenStream {
    let cur_secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let cur_year = 1970 + cur_secs / 3600 / 24 / 365;
    let mut lines = Vec::new();

    lines.push(format!("const CUR_YEAR: usize = {};", cur_year));
    lines.push(gen::functions::find_last_edited());

    lines.push(String::from("fn no_solution() { println!(\"Not solved yet\"); }"));
    lines.push(format!("const FN_POINTER: [[fn() -> (); 25]; {}] = [", cur_year - 2015 + 1));

    lines.push((2015..=cur_year).map(|year| functions::add_fns_of_year(year)).collect::<Vec<_>>().join(","));
    lines.push(String::from("];"));

    lines.join("\n").parse().unwrap()
}

#[proc_macro_attribute]
pub fn parse(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("=======");
    println!("PARSING");
    println!("=======");
    //println!("{:?}", _attr);

    item
}

#[proc_macro]
pub fn aoc_main(_item: TokenStream) -> TokenStream {
    "pub fn solve() { println!(\"ADVENT OF CODE\"); }".parse().unwrap()
}
