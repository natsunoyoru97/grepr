#![deny(missing_docs)]
//! A simple grep printing in a pretty format.

use clap::{ArgMatches, Parser};
use colored::*;
use regex::Regex;
use std::{io, env};

#[derive(Parser)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Opts {
    word_regex: Regex,
    file_regex: Option<String>, 
}

impl From<ArgMatches> for Opts {
    fn from(m: ArgMatches) -> Self {
        let file_regex = m.value_of("file_regex").unwrap();
        match file_regex.is_empty() {
            true => Opts {
                word_regex: Regex::new(m.value_of("word_regex").unwrap())
                    .expect("Parse error"),
                file_regex: None,
            },
            false => Opts {
                word_regex: Regex::new(m.value_of("word_regex").unwrap())
                    .expect("Parse Error"),
                file_regex: Some(file_regex.to_owned()),
            }
        }
    }
}

struct Line<'a> {
    row_num: usize,
    col_num: usize,
    matches: Vec<&'a str>,
    line_content: String,
}

/// Open the src_file to fetch its content
#[tokio::main]
async fn read_file<'a>(src_str: &Regex, file_regex: String) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let files: Vec<_> = glob::glob(file_regex.as_str())?.collect();

    for file in files {
        match file {
            Ok(path) => {
                let path_str = path.display().to_string();
                let content = tokio::fs::read_to_string(path).await?;
                match_word(src_str, &path_str, content.as_str());
            },
            Err(e) => println!("{:?}", e),
        }
        
    }

    Ok(())
}

/// Find the lines that match
fn match_word<'a>(regex: &Regex, path: &str, content: &'a str) {
    let lines = content
                .split("\n")
                .collect::<Vec<&str>>();
    let mut res: Vec<Line> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if regex.is_match(line) {
            let mut new_line = Line {
                row_num: i,
                col_num: regex.find(line).unwrap().start() + 1,
                matches: regex
                        .captures_iter(line)
                        .filter_map(|cap| cap.get(0).map(|m| m.into()))
                        .collect(), 
                line_content: line.to_string(),
            };
            
            render_matched_content(&mut new_line);
            res.push(new_line);
        }
    }

    if !res.is_empty() && !path.is_empty() {
        println!("- {0}", path.purple());
    }
    
    if !res.is_empty() {
        print_res(&res);
    }
}

/// Render the matched String to a certain color
fn render_matched_content(line: &mut Line) {
    for word in line.matches.clone() {
        let new_line_content = line.line_content
                                .replace(word, &format!("{}", word.to_string().red()));
        line.line_content = new_line_content;
    }
}

/// Print fetched lines
fn print_res(fetched_lines: &Vec<Line>) {
    for line in fetched_lines {
        println!("  {0}:{1}     {2}",
                line.row_num.to_string().blue(),
                line.col_num.to_string().green(),
                line.line_content
        );
    }
    println!("");
}

fn main() {
        let opts: Opts = Opts::parse();

        match opts.file_regex {
            // Read file contents
            Some(file) => {
                let result = read_file(&opts.word_regex, file);
                match result {
                    Ok(result) => result,
                    // TODO: Pretty print the error message
                    Err(error) => panic!("Cannot read file: {}", error),
                }
            },
            // Read pipe contents
            None => {
                println!("Now starting reading from pipe...");

                loop {
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("failed to read from pipe");
                    input = input.trim().to_owned();
                    if input.is_empty() {
                        break;
                    }
                    match_word(&opts.word_regex, &"", input.as_str());
                }
            },
        }
        
}
