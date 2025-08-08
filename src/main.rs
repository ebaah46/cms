mod parsing;

use std::fs;
use std::error::Error;

use regex::Regex;
// use crate::parsing::content_parser::parse_content;

fn main() -> Result<(), Box<dyn Error>> {
    let json_string = fs::read_to_string("src/data.json")?;
    println!("{}", json_string);
    // let parsed_node = parse_content(json_string);

    // let text = "listGames(genre=action|adventure)";
    let text = "favoriteGames";
    let re = Regex::new(r"^([^\(]+)(?:\(([^)]*)\))?$")?;
    let caps = re.captures(text);
    if let Some(caps) = caps {
        println!("{:?}", &caps[1]);
    }
    Ok(())
}
