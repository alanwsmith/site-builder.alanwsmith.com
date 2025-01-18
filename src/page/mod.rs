use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Page {
    pub content: String,
}

impl Page {
    pub fn new(content: String) -> Page {
        Page { content }
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
            "include_file" => self.include_file(args),
            "include_file_lines" => self.include_file_lines(args),
            _ => Ok(Value::from("")),
        }
    }
}
