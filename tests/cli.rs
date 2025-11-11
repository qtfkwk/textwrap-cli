use assert_cmd::{Command, cargo};

const ABC: &str = include_str!("abc.txt");
const ABC_WANT: &str = "abcdefghijklm\\\nnopqrstuvwxyz";
const ABC_EOL_WANT: &str = "abcdefghijklm \\\nnopqrstuvwxyz";

const ABC_SPACED: &str = include_str!("abc-spaced.txt");
const ABC_SPACED_WANT: &str = "abc def ghi\\\njkl mno pqr\\\nstu vwx yz";
const ABC_SPACED_EOL_WANT: &str = "abc def ghi \\\njkl mno pqr \\\nstu vwx yz";

const LOREM: &str = include_str!("lorem.txt");
const LOREM_WANT: &str = "\
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor\\
incididunt ut labore et dolore magna aliqua. Ut enim ad minima veniam, quis\\
nostrud exercitation ullamco laboris nisi ut aliquid ex ea commodi consequatur?\\
quis autem vel eum iure reprehenderit, qui in ea voluptate velit esse, quam\\
nihil.";

// Helper functions

/// Retrieve the binary to test
pub fn cmd() -> Command {
    Command::new(cargo::cargo_bin!("tw"))
}

/// Print the command
fn p(args: &[&str]) {
    println!(
        "tw {}",
        args.iter()
            .map(|x| {
                if x.contains(' ') {
                    format!("\"{}\"", x)
                } else {
                    x.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    );
}

/*
/// Run command that fails
fn fail(bin: &str, args: &[&str], code: i32, msg: &str) {
    p(bin, args);
    cmd(bin)
        .args(args)
        .assert()
        .failure()
        .code(code)
        .stderr(format!("Error: \"{}\"\n", msg.replace("\"", "\\\"")));
}
*/

/// Run command that succeeds
fn pass(args: &[&str], want: &str) {
    p(args);
    cmd()
        .args(args)
        .assert()
        .success()
        .stdout(format!("{}\n", want));
}

// Tests

#[test]
fn version() {
    for i in ["-V", "--version"].iter() {
        pass(&[i], &format!("tw {}", env!("CARGO_PKG_VERSION")));
    }
}

#[test]
fn file_lorem() {
    pass(&["tests/lorem.txt"], LOREM_WANT);
}

#[test]
fn stdin_lorem() {
    let args = &["-"];
    p(args);
    cmd()
        .args(args)
        .write_stdin(LOREM)
        .assert()
        .success()
        .stdout(format!("{}\n", LOREM_WANT));
}

#[test]
fn file_abc() {
    pass(&["-w", "13", "tests/abc.txt"], ABC_WANT);
}

#[test]
fn stdin_abc() {
    let args = &["-w", "13", "-"];
    p(args);
    cmd()
        .args(args)
        .write_stdin(ABC)
        .assert()
        .success()
        .stdout(format!("{}\n", ABC_WANT));
}

#[test]
fn file_abc_eol() {
    pass(&["-w", "13", "-e", " \\", "tests/abc.txt"], ABC_EOL_WANT);
}

#[test]
fn stdin_abc_eol() {
    let args = &["-w", "13", "-e", " \\", "-"];
    p(args);
    cmd()
        .args(args)
        .write_stdin(ABC)
        .assert()
        .success()
        .stdout(format!("{}\n", ABC_EOL_WANT));
}

#[test]
fn file_abc_spaced() {
    pass(&["-w", "13", "tests/abc-spaced.txt"], ABC_SPACED_WANT);
}

#[test]
fn stdin_abc_spaced() {
    let args = &["-w", "13", "-"];
    p(args);
    cmd()
        .args(args)
        .write_stdin(ABC_SPACED)
        .assert()
        .success()
        .stdout(format!("{}\n", ABC_SPACED_WANT));
}

#[test]
fn file_abc_spaced_eol() {
    pass(
        &["-w", "13", "-e", " \\", "tests/abc-spaced.txt"],
        ABC_SPACED_EOL_WANT,
    );
}

#[test]
fn stdin_abc_spaced_eol() {
    let args = &["-w", "13", "-e", " \\", "-"];
    p(args);
    cmd()
        .args(args)
        .write_stdin(ABC_SPACED)
        .assert()
        .success()
        .stdout(format!("{}\n", ABC_SPACED_EOL_WANT));
}
