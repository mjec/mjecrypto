extern crate clap;

use clap::{App, Arg, SubCommand};
use mjecrypto::encoding::pgp;
use std::io::{self, stdin, stdout, Read, Write};

fn main() {
    let matches = App::new("PGP Wordlist Encoder/Decoder")
        .version("1.0")
        .author("Michael Cordover <mjecrypto@mjec.net>")
        .about("Encodes and decodes text based on the PGP wordlist")
        .subcommand(
            SubCommand::with_name("encode")
                .about("Encode STDIN to PGP wordlist text")
                .arg(
                    Arg::with_name("width")
                        .short("w")
                        .long("width")
                        .help("Wrap lines at this many words wide, or don't wrap lines if 0")
                        .takes_value(true)
                        .empty_values(false)
                        .default_value("16")
                        .conflicts_with("encode")
                        .validator(validate_width),
                ),
        )
        .subcommand(
            SubCommand::with_name("decode").about("Decode STDIN from PGP wordlist text to bytes"),
        )
        .get_matches();

    let mut buffer = String::new();
    if let Err(e) = stdin().read_to_string(&mut buffer) {
        println!("Unable to read from stdin: {:?}", e);
        ::std::process::exit(1);
    }

    match matches.subcommand() {
        ("encode", m) => match usize::from_str_radix(m.unwrap().value_of("width").unwrap(), 10) {
            Ok(0) => println!("{}", pgp::encode(buffer.as_bytes()).join(" ")),
            Ok(w) => {
                for sixteen_words in pgp::encode(buffer.as_bytes()).chunks(w) {
                    println!("{}", sixteen_words.join(" "));
                }
            }
            Err(_) => unreachable!(),
        },
        ("decode", m) => decode(buffer.as_str()),
        _ => unreachable!(),
    }
}

fn decode(buffer: &str) {
    match pgp::decode(buffer) {
        Ok(result) => stdout().write_all(&result).unwrap(),
        Err(e) => {
            eprintln!("{}", e);
            ::std::process::exit(2)
        }
    };
}

fn validate_width(width_str: String) -> Result<(), String> {
    match usize::from_str_radix(&width_str, 10) {
        Err(_) => Err("width must be an integer".to_string()),
        Ok(n) => Ok(()),
    }
}
