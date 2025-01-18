use minijinja::value::{Object, Value};
use minijinja::Error;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Clone, Debug)]
pub struct Page {
    pub content: String,
}

impl Page {
    pub fn new(content: String) -> Page {
        Page { content }
    }

    pub fn highlight_file(&self, args: &[Value]) -> Result<Value, Error> {
        // TODO: Fix so you can't do `.../` etc...
        if args.len() == 1 {
            let content_dir = PathBuf::from("content");
            let file_path = content_dir.join(args[0].to_string());
            let content = fs::read_to_string(file_path).unwrap();
            let highlighted_content = highlight_code(&content, "css");
            Ok(Value::from(highlighted_content))
        } else {
            Ok(Value::from("could not file file"))
        }
    }

    pub fn ping(&self) -> Result<Value, Error> {
        Ok(Value::from("this is stuff"))
    }

    pub fn include_file(&self, args: &[Value]) -> Result<Value, Error> {
        // TODO: Fix so you can't do `.../` etc...
        if args.len() == 1 {
            let content_dir = PathBuf::from("content");
            let file_path = content_dir.join(args[0].to_string());
            let content = fs::read_to_string(file_path).unwrap();
            Ok(Value::from(content))
        } else {
            Ok(Value::from("could not file file"))
        }
    }

    pub fn include_file_lines(&self, args: &[Value]) -> Result<Value, Error> {
        // TODO: Fix so you can't do `.../` etc...
        if args.len() == 3 {
            let content_dir = PathBuf::from("content");
            let file_path = content_dir.join(args[0].to_string());
            let content = fs::read_to_string(file_path).unwrap();
            let lines = content.lines();
            let start_line: usize = args[1].to_string().parse().unwrap();
            let end_line: usize = args[2].to_string().parse().unwrap();
            let output = lines
                .into_iter()
                .skip(start_line)
                .take(end_line - start_line + 1)
                .map(|l| l.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            Ok(Value::from(output))
        } else {
            Ok(Value::from(
                "could not file file, or args problem with not enough args",
            ))
        }
    }
}

impl Object for Page {
    fn call_method(
        self: &Arc<Page>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "ping" => self.ping(),
            "highlight_file" => self.highlight_file(args),
            "include_file" => self.include_file(args),
            "include_file_lines" => self.include_file_lines(args),
            _ => Ok(Value::from("")),
        }
    }
}

fn highlight_code(code: &str, lang: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set
        .find_syntax_by_token(&lang)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in LinesWithEndings::from(&trim_empty_lines(code)) {
        let _ = html_generator.parse_html_for_line_which_includes_newline(line);
    }
    let initial_html = html_generator.finalize();
    let output_html: Vec<_> = initial_html
        .lines()
        .map(|line| format!(r#"<span class="line-marker"></span>{}"#, line))
        .collect();
    output_html.join("\n")
}

fn trim_empty_lines(source: &str) -> String {
    let re = Regex::new(r"\S").unwrap();
    let trimmed_front = source.split("\n").fold("".to_string(), |acc, l| {
        if !acc.is_empty() {
            acc + l + "\n"
        } else {
            if re.is_match(l) {
                l.to_string() + "\n"
            } else {
                acc
            }
        }
    });
    trimmed_front.trim_end().to_string()
}
