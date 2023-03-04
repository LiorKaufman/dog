use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::str;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    display_d: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut cnt = 0;

                if config.display_d {
                    let reader = Box::new(BufReader::new(File::open(filename).unwrap()));

                    let mut line_of_bytes: Vec<u8> = vec![];
                    for byte_result in reader.bytes() {
                        let byte = byte_result.unwrap();
                        // println!("{}$", byte.is_ascii());
                        if byte.is_ascii() {
                            line_of_bytes.push(byte);
                        } else {
                            if byte == 127 {
                                line_of_bytes.push(b'^');
                                line_of_bytes.push(b'?');
                            } else {
                                line_of_bytes.push(b'M');
                                line_of_bytes.push(b'-');
                                if byte >= 128 + 32 {
                                    if byte < 128 + 127 {
                                        line_of_bytes.push(byte - 128);
                                    } else {
                                        line_of_bytes.push(b'^');
                                        line_of_bytes.push(b'?');
                                    }
                                } else {
                                    line_of_bytes.push(b'^');
                                    line_of_bytes.push(byte - 128 + 64);
                                }
                            }
                        }
                    }
                    let s = match String::from_utf8(line_of_bytes) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };

                    for line in s.lines() {
                        if config.number_lines {
                            cnt += 1;
                            println!("{:>6}\t{}$", cnt, line);
                        } else {
                            println!("{}$", line);
                        }
                    }
                } else {
                    for line_result in file.lines() {
                        // check if line contains valid utf-8
                        match line_result {
                            Ok(line) => {
                                cnt += 1;
                                if config.number_lines {
                                    println!("{:>6}\t{}", cnt, line);
                                } else if config.number_nonblank_lines {
                                    if line.is_empty() {
                                        println!("{}", line);
                                        cnt -= 1;
                                    } else {
                                        println!("{:>6}\t{}", cnt, line);
                                    }
                                } else {
                                    println!("{}", line);
                                }
                            }
                            // line contains invalid utf-8
                            Err(err) => {
                                println!("Error: {}", err);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("dog")
        .version("0.1.0")
        .author("Lior Kaufman")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("display_d")
                .short("e")
                .help(
                    "Display non-print characters, 
                 and display a dollar ($) sign at the end of each line",
                )
                .takes_value(false),
        )
        .about("Terminal application like cat but it's made in rust and called dog")
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
        display_d: matches.is_present("display_d"),
    })
}
// Hey everyone!
// I am currently following a book named Command Line Rust, I am currently trying to build a rust version of cat.
// I am c  urrently trying to add a "new feature" to the book provided solution.
