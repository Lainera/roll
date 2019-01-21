use std::env;

mod arguments;
mod rolls;

fn main() {
    let result: Vec<rolls::Roll> = arguments::parse(env::args()).iter()
        .map(|roll_config| roll_config.roll())
        .collect();

    println!("{:?}", result);
}
