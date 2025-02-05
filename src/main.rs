use parsecomx::{parsers, traits::Parser};


fn main() {
    let result = 
        parsers::uint_parser::<u32>(10)
        .parse("-256")
    ;
    println!("{result:?}");
}