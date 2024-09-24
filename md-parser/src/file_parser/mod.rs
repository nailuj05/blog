use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum List {
    None,
    Ordered(usize),
    Unordered(usize),
}

pub fn parse(file: &str) -> String {
    let file = File::open(file).unwrap();

    let reader = BufReader::new(file);

    let mut codeblock = false;
    let mut list: Vec<List> = Vec::new();

    let mut html: String = String::new();

    for line in reader.lines() {
        match line {
            Err(_) => break,
            Ok(l) => html.push_str(parse_line(l.as_str(), &mut codeblock, &mut list).as_str()),
        }
    }

    html
}

fn convert_links(input: &str) -> String {
    let mut output = input.to_string();

    // Convert images with optional size specification
    let image_regex = Regex::new(r"!\[\[([^]]+)\]\]").unwrap();
    output = image_regex
        .replace_all(&output, r#"<img src="_images/$1">"#)
        .to_string();

    // Convert web links
    let web_link_regex = Regex::new(r"\[([^]]+)\]\(([^)]+)\)").unwrap();
    output = web_link_regex
        .replace_all(&output, r#"<a href="$2">$1</a>"#)
        .to_string();

    // Convert file links
    let file_link_regex = Regex::new(r"\[\[([^|\]]+)\]\]").unwrap();
    output = file_link_regex
        .replace_all(&output, r#"<a href="$1.html">$1</a>"#)
        .to_string();

    // Convert named file links
    let named_file_link_regex = Regex::new(r"\[\[([^|]+)\|([^]]+)\]\]").unwrap();
    output = named_file_link_regex
        .replace_all(&output, r#"<a href="$1.html">$2</a>"#)
        .to_string();

    output
}

fn parse_line(line: &str, codeblock: &mut bool, list: &mut Vec<List>) -> String {
    let mut html_line: String = String::new();

    // Codeblock
    if line.starts_with("```") {
        html_line.push_str(if *codeblock { "</code>" } else { "<code>" });
        *codeblock = !*codeblock;
        return html_line;
    }

    // Quote
    if line.starts_with(">") {
        html_line.push_str("<blockquote");
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
        let n = line.trim().chars().take_while(|&c| c != ' ').count() + 1;
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
    html_line = code_re.replace_all(&html_line, "<code>$1</code>").to_string();

    // Quote
    if line.starts_with(">") {
        html_line.push_str("</blockquote>");
    }

    html_line = convert_links(html_line.as_str());

    html_line
}
