use parsecomx::{parsers, Parser};

#[derive(Debug)]
enum Item {
    Integer(u32),
    Literal(String)
}

fn main() {

    let parse_literal = 
        parsers::char_parser('"')
        .then_parse(
            parsers::any_char()
            .map(|c| c.to_string())
            .sep_by(parsers::string_parser("\\\"").optional())
            .map(|chars| chars.join("\\\""))
        )
        .then_consume(parsers::char_parser('"'))
        .map_err(|_| "error while parsing")
    ;

    // let parse_item = 
    //     parsers::uint_parser::<u32>(10)
    //     .map(|n| Item::Integer(n))
    //     .map_err(|_| "cant parse int")
    //     .or_else(parse_literal.map(|s| Item::Literal(s)))
    // ;

    // let items_with_space = 
    //     parsers::whitespaces_parser().optional()
    //     .then_continue_with(parse_item)
    //     .then_consume_optional(parsers::whitespaces_parser().optional())
    // ;

    // let comma_sep_items = 
    //     items_with_space
    //     .sep_by(parsers::char_parser(',').map_err(|_| "expects comma"))
    // ;

    // "hello \n herer\""

    let input = "\"h \\\" x\"";

    let result = 
        // parsers::char_parser('[')
        // .then_parse(comma_sep_items)
        // .then_consume(parsers::char_parser(']'))
        parsers::char_parser('"')
        .then_parse(
            parsers::any_char()
            .parse_if(|c| {
                if *c != '\\' {
                    Ok(())
                }
                else {
                    Err(None)
                }
            })
            .parse_if(|c| {
                if *c != '"' {
                    Ok(())
                }
                else {
                    Err(None)
                }
            })
            .many()
            .parse_if(|result| {
                if !result.is_empty() {
                    Ok(())
                }
                else {
                    Err(None)
                }
            })
            .map(|x| {println!("parsed string {x:?}"); x})
            .and_then(
                parsers::char_parser('\\')
                .then_parse(parsers::any_char())
                .optional()
            )
            .map(|(mut result, escaped_char)| {
                if let Some(escaped_char) = escaped_char {
                    result.push(escaped_char);
                }

                String::from_iter(result)
            })
            .many()
        )
        .map(|x| x.concat())
        .then_consume(parsers::char_parser('"'))

        .parse(input)
    ;


    println!("{result:?}");
}