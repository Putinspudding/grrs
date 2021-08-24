use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

#[allow(unused)]
#[derive(StructOpt)]
struct Cli {
    // pattern
    pattern: String,
    // path
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    // print line number with output lines
    #[structopt(short = "n", long = "line-number")]
    display_line_number: bool,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let f = File::open(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut line_num: u32 = 1;
    loop {
        if let Ok(0) = reader.read_line(&mut line) {
            break;
        }
        if args.display_line_number {
            grrs::print_output_with_numbers(&line, &args.pattern, line_num);
        } else {
            grrs::print_output(&line, &args.pattern);
        }
        line.clear();
        line_num += 1;
    }
    Ok(())
}
