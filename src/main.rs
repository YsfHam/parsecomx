use parsecomx::traits::{Parser, FloatParser};

fn main() {
    let result= 
        '['
        .then_parse(f32::str_parser())
        .then_consume(&']')
        .parse("[1.0]")
    ;
    match result {
        Ok((rest, output)) => println!("rest \"{rest}\", output={output}"),
        Err((_, error)) => println!("error {error:?}"),
    }
}