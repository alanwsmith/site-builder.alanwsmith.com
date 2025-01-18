use alans_site_builder::page::*;
use minijinja::syntax::SyntaxConfig;
use minijinja::{context, Environment, Value};
use std::fs;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn is_non_hidden_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    let source_files = get_source_files();
    process_source_files(source_files);
}

fn get_source_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = vec![];
    let walker = WalkDir::new("content").into_iter();
    for entry in walker.filter_entry(|e| !is_non_hidden_file(e)) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            let entry_path = entry.path().to_path_buf();
            let path_parts: Vec<String> = entry_path
                .iter()
                .skip(1)
                .map(|p| p.to_string_lossy().to_string())
                .collect();
            let chopped_path = PathBuf::from(path_parts.join("/"));
            files.push(chopped_path)
        }
    }
    files
}

fn process_source_files(files: Vec<PathBuf>) {
    let mut env = Environment::new();
    env.set_syntax(
        SyntaxConfig::builder()
            .block_delimiters("[!", "!]")
            .variable_delimiters("[@", "@]")
            .comment_delimiters("[#", "#]")
            .build()
            .unwrap(),
    );
    let wrapper_path = "templates/wrappers/main.html";
    let wrapper_string = fs::read_to_string(&wrapper_path).unwrap();
    env.add_template_owned(wrapper_path, wrapper_string)
        .unwrap();

    let content_dir = PathBuf::from("content");
    let output_dir = PathBuf::from("_site");
    files.iter().for_each(|file| {
        let input_path = content_dir.join(file);
        let output_path = output_dir.join(file);
        let page = Page::new(fs::read_to_string(input_path).unwrap());
        // let content = fs::read_to_string(input_path).unwrap();
        env.add_template_owned("working-file", page.clone().content)
            .unwrap();
        match env.get_template("working-file") {
            Ok(template) => match template.render(context!(page => Value::from_object(page))) {
                Ok(output) => {
                    write_file_with_mkdir(&output_path, &output).unwrap();
                }
                Err(e) => {
                    dbg!(e);
                    ()
                }
            },
            Err(e) => {
                dbg!(e);
                ()
            }
        }
    })
}

fn write_file_with_mkdir(path: &PathBuf, content: &str) -> Result<(), String> {
    match path.parent() {
        Some(parent_dir) => match fs::create_dir_all(parent_dir) {
            Ok(_) => match fs::write(path, content) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        None => Err("Could not make directory".to_string()),
    }
}
