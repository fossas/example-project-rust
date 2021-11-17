use clap::{App, Arg};
use serde_derive::Serialize;

#[derive(Serialize)]
struct Result {
    greeting: String,
    name: String,
}

fn main() {
    let matches = App::new("FOSSA Rust Example")
        .version("1.0")
        .arg(
            Arg::with_name("greeting")
                .long("greeting")
                .short("g")
                .value_name("GREETING")
                .help("The greeting to use")
                .default_value("Hello")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .long("name")
                .short("n")
                .value_name("NAME")
                .help("The entity to greet")
                .default_value("World")
                .takes_value(true),
        )
        .get_matches();

    let output = Result {
        greeting: matches
            .value_of("greeting")
            .expect("Greeting must be provided")
            .to_owned(),
        name: matches
            .value_of("name")
            .expect("Name must be provided")
            .to_owned(),
    };

    let serialized = serde_json::to_string(&output).expect("Must serialize");
    println!("{}", serialized);
}
