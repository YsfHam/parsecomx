use parsecomx::{parsers::{self, AnyChar}, Parser, ParserResult};

fn main() {

    let input = "                   hello";

    let ParserResult {
        rest,
        out_result
    } = 
    parsers::parse_char(' ')
        .many()
        .and_then(parsers::parse_char('h'))
        .parse(input);

    match out_result {
        Ok(output) => println!("{:?}", (output, rest)),
        Err(error) => println!("{:?}", error)
    }
}
