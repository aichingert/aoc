extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, Ident, Span};

const BASE_DIR: &'static str = "advent/src/";

#[proc_macro_attribute]
pub fn aoc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("=======");
    println!("  AOC ");
    println!("=======");
    let mut tokens = Vec::<TokenTree>::from([
        TokenTree::Ident(Ident::new("fn", Span::call_site())),
        TokenTree::Ident(Ident::new("main", Span::call_site()))
    ]);

    for i in item.clone() {
        match i {
            TokenTree::Group(ref g) => {
                for j in g.stream() {
                    println!("{j:?}");
                }
            }
            ref r => println!("{r:?}"),
        }
    }

    for path in std::fs::read_dir(BASE_DIR).unwrap() {
        let path = path.unwrap().path();

        match path.is_dir() {
            _ => (),
        }
    }

    TokenStream::from_iter(tokens.into_iter().chain(item.into_iter().skip(2)))
}

#[proc_macro_attribute]
pub fn parse(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("=======");
    println!("PARSING");
    println!("=======");
    println!("{:?}", _attr);

    item
}

#[proc_macro]
pub fn aoc_main(_item: TokenStream) -> TokenStream {
    "pub fn solve() { println!(\"ADVENT OF CODE\"); }".parse().unwrap()
}
