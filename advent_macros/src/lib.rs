extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn aoc_main(_item: TokenStream) -> TokenStream {
    "fn main() { println!(\"ADVENT OF CODE {}\", add(10, 10)); }".parse().unwrap()
}
