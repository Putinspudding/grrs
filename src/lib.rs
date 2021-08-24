use console::{style, Color};
use itertools::Itertools;

pub fn print_output(line: &str, pattern: &str) {
    if line.contains(pattern) {
        let v = format!(
            "{}",
            line.split(pattern)
                .intersperse(&format!("{}", style(pattern).red().bold()))
                .collect::<String>()
        );
        print!("{}", v);
    }
}

pub fn print_output_with_numbers(line: &str, pattern: &str, line_num: u32) {
    if line.contains(pattern) {
        let v = format!(
            "{}{}{}",
            style(&line_num.to_string()).green(),
            style(":").blue(),
            line.split(pattern)
                .intersperse(&format!("{}", style(pattern).red().bold()))
                .collect::<String>()
        );
        print!("{}", v);
    }
}
