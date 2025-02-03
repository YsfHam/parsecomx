use parsecomx::{parsers, Parser};

fn main() {

    let input = "-1234";

    let result = 
        parsers::int_parser::<i32>(10)
        .parse(input)
    ;


    println!("{result:?}");
}