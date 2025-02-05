use parsecomx::{parsers, traits::Parser};


fn main() {
    let result = 
        parsers::string_literal_parser()
        .parse("\"hello\\n\\\"J\\\"  \\t this is with tabs 'rr\"")
    ;

    match result {
        Ok((rest, output)) => println!("rest \"{rest}\", output={output}"),
        Err((_, error)) => println!("error {error:?}"),
    }
}