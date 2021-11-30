use clap::{Parser};
use colored::*;
use tokio::fs;
use regex::Regex;

#[derive(Parser)]
#[clap(version = "1.0", author = "natsunoyoru97 <natsunoyoru97@outlook.com>")]
struct Opts {
    regex: Regex,
    src_file: String,
}

struct Line<'a> {
    row_num: usize,
    col_num: usize,
    matches: Vec<&'a str>,
    line_content: String,
}

/// Open the src_file to fetch its content
#[tokio::main]
async fn read_file<'a>(src_str: Regex, file_name: &'a str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let content = fs::read_to_string(file_name).await?;
    match_word(src_str, content.as_str());

    Ok(())
}

/// Find the lines that match
/// TODO: Highlight the src_str
fn match_word<'a>(regex: Regex, content: &'a str) {
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

            // Highlight the matched words
            // TODO: Move the code snippet, making them a single function
            for word in new_line.matches.clone() {
                let new_line_content = new_line.line_content
                                        .replace(word, &format!("{}", word.to_string().red()));
                new_line.line_content = new_line_content;
            }

            res.push(new_line);
        }
    }

    highlight_match(&res);
    print_res(&res);

}

fn highlight_match(fetched_line: &Vec<Line>) {
    // TODO: The function to fill
}

fn print_res(fetched_lines: &Vec<Line>) {
    for line in fetched_lines {
        println!("{}:{} {}",
                line.row_num.to_string().blue(),
                line.col_num.to_string().green(),
                line.line_content
        );
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    // Read file
    let result = read_file(opts.regex, opts.src_file.as_str());
    match result {
        Ok(result) => result,
        // TODO: Pretty print the error message
        Err(error) => panic!("Cannot read file: {}", error),
    }
}
