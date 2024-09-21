use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

enum List {
    None,
    Ordered(usize),
    Unordered(usize),
}

fn main() {
    parse("test.md").unwrap();
}

fn parse(file: &str) -> io::Result<()> {
    let file = File::open(file)?;

    let reader = BufReader::new(file);

    let mut codeblock = false;
    let mut list: Vec<List> = Vec::new();

    for line in reader.lines() {
        match line {
            Err(_) => break,
            Ok(l) => println!("{}", parse_line(l.as_str(), &mut codeblock, &mut list)),
        }
    }

    Ok(())
}

fn parse_line(line: &str, codeblock: &mut bool, list: &mut Vec<List>) -> String {
    let mut html_line: String = String::new();

    // Codeblock
    if line.starts_with("```") {
        html_line.push_str(if *codeblock { "</code>" } else { "<code>" });
        *codeblock = !*codeblock;
        return html_line;
    }

    // Headers
    let header_level = line
        .chars()
        .scan(0, |acc, c| {
            if c == '#' {
                *acc += 1;
                Some(*acc)
            } else {
                None
            }
        })
        .last()
        .unwrap_or(0);

    if header_level > 0 {
        html_line.push_str("<h");
        html_line.push_str(&header_level.to_string());
        html_line.push_str(">");
        html_line.push_str(&line[header_level + 1..]);
        html_line.push_str("</h");
        html_line.push_str(&header_level.to_string());
        html_line.push_str(">");
        return html_line;
    }

    // Lists
    let ul = Regex::new(r"^\t*[-*+]\s+.+$").unwrap().is_match(&line);
    let ol = Regex::new(r"^\t*\d+\.\s+.+$").unwrap().is_match(&line);
    let depth: usize = line.chars().take_while(|&c| c == '\t').count();

    if ul {
        match list.last().unwrap_or(&List::None) {
            List::None => {
                html_line.push_str(&format!("<ul>\n<li>{}</li>", &line.trim()[2..]));
                list.push(List::Unordered(depth));
            }
            // if last list was ordered
            List::Ordered(_) => {
                html_line.push_str("</ol>\n");
                list.pop();
                html_line.push_str(&format!("<ul>\n<li>{}</li>", &line.trim()[2..]));
                list.push(List::Unordered(depth));
            }
            // if last list was also unordered
            List::Unordered(last_depth) => {
                if last_depth > &depth {
                    html_line.push_str("</ul>\n");
                    list.pop();
                } else if last_depth < &depth {
                    html_line.push_str("<ul>\n");
                    list.push(List::Unordered(depth));
                }
                html_line.push_str(&format!("<li>{}</li>", &line.trim()[2..]));
            }
        }
    } else if ol {
        let n = *&line.trim().chars().take_while(|&c| c != ' ').count() + 1;
        match list.last().unwrap_or(&List::None) {
            List::None => {
                html_line.push_str(&format!("<ol>\n<li>{}</li>", &line.trim()[n..]));
                list.push(List::Ordered(depth));
            }
            // if last list was also ordered
            List::Ordered(last_depth) => {
                if last_depth > &depth {
                    html_line.push_str("</ol>\n");
                    list.pop();
                } else if last_depth < &depth {
                    html_line.push_str("<ol>\n");
                    list.push(List::Ordered(depth));
                }
                html_line.push_str(&format!("<li>{}</li>", &line.trim()[n..]));
            }
            // if last list was unordered
            List::Unordered(_) => {
                html_line.push_str("</ul>\n");
                list.pop();
                html_line.push_str(&format!("<ol><li>{}</li>", &line.trim()[n..]));
                list.push(List::Ordered(depth));
            }
        }
    } else {
        while !list.is_empty() {
            match list.pop().unwrap() {
                List::None => (),
                List::Ordered(_) => html_line.push_str("</ol>"),
                List::Unordered(_) => html_line.push_str("</ul>"),
            }
        }
        html_line.push_str(&format!("{}<br>", &line.trim()))
    }

    // Bold, Italics & inline Code
    let bold_re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let italics_re = Regex::new(r"\*(.*?)\*").unwrap();
    let code_re = Regex::new(r"`([^`]+)`").unwrap();
    html_line = bold_re.replace_all(&html_line, "<b>$1</b>").to_string();
    html_line = italics_re.replace_all(&html_line, "<i>$1</i>").to_string();
    html_line = code_re
        .replace_all(&html_line, "<code>$1</code>")
        .to_string();

    html_line
}
