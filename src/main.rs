use parsecomx::{parsers, Parser};

fn main() {

    let input = "[1, 2, 3, 4, 5]";

    let int_parser = 
        parsers::whitespaces_parser().optional()
        .then_continue_with(parsers::uint_parser::<u32>(10))
        .then_consume_optional(parsers::whitespaces_parser().optional())
    ;

    let comma_sep_ints = 
        int_parser
        .sep_by(parsers::char_parser(','))
    ;

    let result = 
        parsers::char_parser('[')
        .then_parse(comma_sep_ints)
        .then_consume(parsers::char_parser(']'))
        .parse(input)
    ;


    println!("{result:?}");
}