use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};
use std::path::PathBuf;

/// Command line interface for textwrap; https://crates.io/crates/textwrap-cli
#[derive(Parser)]
#[clap(about, version, name = "tw")]
struct Args {
    /// Width
    #[clap(short, default_value = "80")]
    width: usize,

    /// End of line string
    #[clap(short, default_value = "\\")]
    eol: String,

    /// Input file(s); use `-` to read from standard input
    #[clap(required = true)]
    input_files: Vec<PathBuf>,
}

/// Command line interface
fn main() -> Result<(), String> {
    let args = Args::parse();

    for input_file in args.input_files {
        if input_file.as_os_str() == "-" {
            let f = stdin();
            let mut r = BufReader::new(f);
            process_reader(&mut r, args.width, &args.eol);
        } else {
            let f = File::open(&input_file).unwrap();
            let mut r = BufReader::new(f);
            process_reader(&mut r, args.width, &args.eol);
        }
    }

    Ok(())
}

fn chomp(s: &str) -> String {
    let mut s = s.to_string();
    while s.ends_with('\n') {
        s.pop();
        while s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

fn process_line(line: &str, width: usize, eol: &str) {
    let line = chomp(line);
    let lines = textwrap::wrap(&line, width);
    let last = lines.len() - 1;
    for (i, line) in lines.iter().enumerate() {
        if i == last {
            println!("{line}");
        } else {
            println!("{line}{eol}");
        }
    }
}

fn process_reader<R>(r: &mut BufReader<R>, width: usize, eol: &str)
where
    R: Read,
{
    let mut line = String::new();
    loop {
        match r.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    process_line(&line, width, &eol);
                    line = String::new();
                }
            }
            Err(_e) => break,
        }
    }
}
