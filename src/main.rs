use anyhow::{Context, Result};
use console::style;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

#[allow(unused)]
#[derive(StructOpt)]
struct Cli {
    //pattern
    pattern: String,
    //path
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let f = File::open(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    loop {
        if let Ok(0) = reader.read_line(&mut line) {
            break;
        }
        if line.contains(&args.pattern) {
            // itertools version
            let v = line
                .split(&args.pattern)
                .intersperse(&format!("{}", style(&args.pattern).green().bold()))
                .collect::<String>();
            // Non-itertools version
            /*
            let v = line
                .split(&args.pattern)
                .collect::<Vec<&str>>()
                .join(&format!("{}", style(&args.pattern).green().bold()));
            */
            print!("{}", v);
        }
        line.clear();
    }
    Ok(())
}
