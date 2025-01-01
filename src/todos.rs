use std::path::PathBuf;
use walkdir::WalkDir;
use walkdir::DirEntry;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let source_code_extensions: Vec<&str> = vec![
    "c", "rs", "cpp", "h", "hpp", "py", "dart", "java", "cs", "sql", 
    "yaml", "json", "R", "css", "js", "bat", "sh", "cmake", "Makefile", 
    "mk", "glsl", "hlsl", "txt", "cfg", "config", "ini", "xml", "frag", 
    "vert", "md"];

	let entries: Vec<DirEntry> = WalkDir::new(".").into_iter().filter_map(|e| e.ok()).collect();

	let mut entries_to_analyze : Vec<PathBuf> = vec![];
    for entry in entries {
        let entry_extension = entry.path().extension();
        if (!entry_extension.is_none()) && (source_code_extensions.contains(&entry_extension.unwrap().to_str().unwrap())) {
            entries_to_analyze.push(entry.into_path());
        }
    }

    let todo_str = "// TODO";
    for entry in entries_to_analyze {
    	let file = File::open(&entry)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let buffer_str: &str = std::str::from_utf8(&buffer).unwrap();
        let str_split_into_newlines = buffer_str.split('\n');

        let mut line_number: usize = 0;
        for line in str_split_into_newlines {
        	if line.contains(todo_str) {
        		println!("File: {filepath} ({line_number}): {line}", filepath = entry.to_str().unwrap());
        	}
        	line_number += 1;
        }
    }

    Ok(())
}