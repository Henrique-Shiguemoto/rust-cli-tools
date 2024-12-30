use std::path::PathBuf;
use walkdir::WalkDir;
use walkdir::DirEntry;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_code_extensions: Vec<&str> = vec!["c", "rs", "cpp", "h", "hpp", "py", "dart", "java", "cs", "sql", "yaml", "json", "R", "css", "js"]; // can add more of course

    let entries: Vec<DirEntry> = WalkDir::new(".").into_iter().filter_map(|e| e.ok()).collect();
    let mut entries_to_analyze : Vec<PathBuf> = vec![];
    for entry in entries {
        let entry_extension = entry.path().extension();
        if (!entry_extension.is_none()) && (source_code_extensions.contains(&entry_extension.unwrap().to_str().unwrap())) {
            entries_to_analyze.push(entry.into_path());
        }
    }

    let mut vec_file_extension_line_count: Vec<(String, usize)> = Vec::new();
    for entry in entries_to_analyze {
        let file = File::open(&entry)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let mut line_count: usize = 0;
        for c in buffer {
            if c == b'\n' { line_count += 1; }
        }

        let extensions_from_vec: Vec<String> = vec_file_extension_line_count.iter().map(|(ext, _)| ext.clone()).collect();
        let entry_extension = entry.as_path().extension().unwrap().to_str().unwrap();
        if let Some(i) = extensions_from_vec.iter().position(|r| r == entry_extension) {
            if let Some((_str, num)) = vec_file_extension_line_count.get_mut(i) { 
                *num += line_count; 
            }
        } else { 
            vec_file_extension_line_count.push((entry_extension.to_string(), line_count));
        }
    }

    vec_file_extension_line_count.sort_by_key(|x| x.1); // sorting by the line_count column (index 1)
    println!("Results:\n");
    println!("----------------------------------------------------");
    let mut total_line_count: usize = 0;
    for (extension, line_count) in vec_file_extension_line_count {
        println!("|  {ext}\t\t\t| {line_count} lines", ext = extension.to_uppercase());
        total_line_count += line_count;
    }
    println!("----------------------------------------------------");
    println!("|  TOTAL\t\t| {total_line_count} lines");
    println!("----------------------------------------------------");

    Ok(())
}