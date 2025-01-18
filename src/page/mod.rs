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
            _ => Ok(Value::from("")),
        }
    }
}
