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
    let input_dir = PathBuf::from("content");
    let output_dir = PathBuf::from("_site");
    files.iter().for_each(|file| {
        let input_path = input_dir.join(file);
        let output_path = output_dir.join(file);
        let content = fs::read_to_string(input_path).unwrap();
        fs::write(output_path, content).unwrap();
    })
}
