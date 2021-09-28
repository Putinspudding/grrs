use console::{style, Color};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_line(line: &str, pattern: &str) {
    let v = format!(
        "{}",
        line.split(pattern)
            .intersperse(&format!("{}", style(pattern).red().bold()))
            .collect::<String>()
    );
    print!("{}", v);
}

fn print_line_regex(line: &str, pattern: &str, re: &Regex) {
    let m: Vec<[usize; 2]> = re.find_iter(line).map(|x| [x.start(), x.end()]).collect();
    let mut modified_line: String = line.get(0..m[0][0]).unwrap().to_string();
    let length = m.len();
    let mut index = 0;
    while index < length {
        modified_line = format!(
            "{}{}{}",
            modified_line,
            format!(
                "{}",
                style(line.get(m[index][0]..m[index][1]).unwrap())
                    .red()
                    .bold()
            ),
            line.get(
                m[index][1]..if index < length - 1 {
                    m[index + 1][0]
                } else {
                    line.len()
                }
            )
            .unwrap()
        );
        index += 1;
    }
    print!("{}", modified_line);
}

fn print_line_with_numbers(line: &str, pattern: &str, line_num: u32) {
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

fn print_line_with_numbers_regex(line: &str, pattern: &str, re: &Regex, line_num: u32) {
    let m: Vec<[usize; 2]> = re.find_iter(line).map(|x| [x.start(), x.end()]).collect();
    let mut modified_line: String = format!(
        "{}{}{}",
        style(&line_num.to_string()).green(),
        style(":").blue(),
        line.get(0..m[0][0]).unwrap().to_string()
    );
    let length = m.len();
    let mut index = 0;
    while index < length {
        modified_line = format!(
            "{}{}{}",
            modified_line,
            format!(
                "{}",
                style(line.get(m[index][0]..m[index][1]).unwrap())
                    .red()
                    .bold()
            ),
            line.get(
                m[index][1]..if index < length - 1 {
                    m[index + 1][0]
                } else {
                    line.len()
                }
            )
            .unwrap()
        );
        index += 1;
    }
    print!("{}", modified_line);
}

pub fn print_output(line: &str, pattern: &str, regex_off: bool, re: &Regex) {
    if !regex_off {
        if re.is_match(line) {
            print_line_regex(line, pattern, re);
        }
    } else {
        if line.contains(pattern) {
            print_line(line, pattern);
        }
    }
}

pub fn print_output_with_numbers(
    line: &str,
    pattern: &str,
    regex_off: bool,
    re: &Regex,
    line_num: u32,
) {
    if !regex_off {
        if re.is_match(line) {
            print_line_with_numbers_regex(line, pattern, re, line_num);
        }
    } else {
        if line.contains(pattern) {
            print_line_with_numbers(line, pattern, line_num);
        }
    }
}

pub fn print_count(mut reader: BufReader<File>, pattern: &str) {
    let mut line = String::new();
    let mut count = 0;
    loop {
        if let Ok(0) = reader.read_line(&mut line) {
            break;
        }
        if line.contains(pattern) {
            count += 1;
        }
        line.clear();
    }
    println!("{}", count);
}
