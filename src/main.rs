use parsecomx::{parsers, Parser};

fn main() {

    let input = "hello world";

    let result = 
        parsers::any_char()
        .and_then(parsers::parse_char('h'))
        .parse(input)
    ;

    println!("{result:?}");
}
