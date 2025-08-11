mod parsing;
mod processing;

use std::fs;
use std::error::Error;


use crate::parsing::content_parser::parse_content;


fn main() -> Result<(), Box<dyn Error>> {
    let file = "src/data.json";
    let json_string = fs::read_to_string(&file)?;
    // println!("{}", json_string);
    let mut parsed = parse_content(json_string)?;
    parsed.add_name(&file);
    println!("{:#?}", parsed);
    
    Ok(())
}
