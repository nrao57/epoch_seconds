use chrono::{DateTime};
use chrono::format::ParseError;
use std::env;


fn main() -> Result<(), ParseError> {
    let given_args: Vec<String> = env::args().collect();
    let date_given = &given_args[1];
    let rfc3339 = DateTime::parse_from_rfc3339(&date_given).expect("Unable to Parse Date");
    println!("{}", rfc3339.timestamp());
    Ok(())
}