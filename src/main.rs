use parsecomx::{parsers, Parser, ParserResult};

fn main() {

    let input = "               h        ello";

    let ParserResult {
        rest,
        out_result
    } = 
    parsers::parse_char(' ')
        .many()
        .then_parse(parsers::parse_char('h'))
        .then_consume(parsers::parse_char(' ').many())
        .and_then(parsers::parse_char('e'))
        .parse(input);

    match out_result {
        Ok(output) => println!("{:?}", (output, rest)),
        Err(error) => println!("{:?}", error)
    }
}
