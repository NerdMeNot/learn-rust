use std::num::ParseIntError;

// "3,4" -> (3, 4). split_once finds the comma, parse does the numbers,
// and ? hands the first parse error to the caller.
fn parse_pair(text: &str) -> Result<(i32, i32), ParseIntError> {
    let (left, right) = text.split_once(',').unwrap_or((text, ""));
    Ok((left.trim().parse()?, right.trim().parse()?))
}

fn main() {
    println!("{:?}", parse_pair("3,4"));
    println!("{:?}", parse_pair(" 3 , -4 "));
    println!("{:?}", parse_pair("3"));
    match parse_pair("3,x") {
        Ok(pair) => println!("pair {pair:?}"),
        Err(e) => println!("error: {e}"),
    }
}
