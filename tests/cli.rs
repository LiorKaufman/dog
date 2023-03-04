use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "dog";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

// --------------------------------------------------
#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
fn run_invalid_utf_8(args: &[&str], expected_file: &str) -> TestResult {
    let expected = get_invalid_file_to_string(expected_file);
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
// --------------------------------------------------
fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn get_invalid_file_to_string(file: &str) -> String {
    println!("{}", file);
    let file = File::open(file).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = vec![];
    let mut str_buffer = String::new();

    while let Ok(_) = reader.read_until(b'\n', &mut buf) {
        if buf.is_empty() {
            break;
        }
        let line = String::from_utf8_lossy(&buf);
        str_buffer.push_str(&line);

        println!("{}", line);
        buf.clear();
    }
    str_buffer
}
// --------------------------------------------------
fn run_stdin_no_utf8(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = get_invalid_file_to_string(expected_file);
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
// --------------------------------------------------
#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")
}

// --------------------------------------------------
#[test]
fn bustle_stdin_e() -> TestResult {
    run_stdin_no_utf8(
        BUSTLE,
        &["-e", "-"],
        "tests/expected/the-bustle.txt.e.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_stdin_n() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected/the-bustle.txt.n.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_stdin_b() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-b", "-"],
        "tests/expected/the-bustle.txt.b.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n() -> TestResult {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

// --------------------------------------------------
#[test]
fn empty_b() -> TestResult {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

// --------------------------------------------------
#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_n() -> TestResult {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

// --------------------------------------------------
#[test]
fn fox_b() -> TestResult {
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

// --------------------------------------------------
#[test]
fn fox_e() -> TestResult {
    run_invalid_utf_8(&["-e", FOX], "tests/expected/fox.txt.e.out")
}

// --------------------------------------------------
#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

// --------------------------------------------------
#[test]
fn spiders_n() -> TestResult {
    run(&["--number", SPIDERS], "tests/expected/spiders.txt.n.out")
}

// --------------------------------------------------
#[test]
fn spiders_b() -> TestResult {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
fn spiders_e() -> TestResult {
    run(&["-e", SPIDERS], "tests/expected/spiders.txt.e.out")
}
// --------------------------------------------------
#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

// --------------------------------------------------
#[test]
fn bustle_n() -> TestResult {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

// --------------------------------------------------
#[test]
fn bustle_b() -> TestResult {
    run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}
// --------------------------------------------------
#[test]
fn bustle_e() -> TestResult {
    run_invalid_utf_8(&["-b", BUSTLE], "tests/expected/the-bustle.txt.e.out")
}
// --------------------------------------------------
#[test]
fn all() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

// --------------------------------------------------
#[test]
fn all_n() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

// --------------------------------------------------
#[test]
fn all_b() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
// --------------------------------------------------
#[test]
fn all_e() -> TestResult {
    run_invalid_utf_8(&[FOX, SPIDERS, BUSTLE, "-e"], "tests/expected/all.e.out")
}
