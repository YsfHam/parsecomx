use std::str::FromStr;

use parsecomx::{parsers, traits::Parser};


fn main() {
    let result = 
        parsers::float_parser::<f32>()
        .sep_by(parsers::whitespaces_parser())
        .parse("-256.0 1.0 0.5 -2. 4.")
    ;

    println!("{result:?}");
}