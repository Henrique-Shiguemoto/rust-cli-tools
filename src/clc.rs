use regex::Regex;
use std::path::PathBuf;
use walkdir::WalkDir;
use walkdir::DirEntry;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::env;

fn usage(){
    println!("USAGE: clc (optional)<SUBCOMMANDS>");
    println!("SUBCOMMANDS:");
    println!("\t-pathexclude=[PATHS]   : excludes files which are in the PATHS specified. For multiple paths to exclude, just separate them with :");
    println!("\t-extexclude=[EXTS]     : excludes files which contain the extensions listed in EXTS. For multiple extensions to exclude, just separate them with :");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut exclude_paths: Vec<String> = vec![];
    let mut exclude_exts: Vec<String> = vec![];
    if args.len() >= 2 {
        let pattern_exclude_paths = Regex::new(r"^-pathexclude=.*").unwrap();
        let pattern_exclude_exts = Regex::new(r"^-extexclude=.*").unwrap();
        if args[1] == "-help" {
            usage();
        } else {
            let pathexclude_element: Option<&String> = args.iter().find(|s| pattern_exclude_paths.is_match(s));
            let extexclude_element: Option<&String> = args.iter().find(|s| pattern_exclude_exts.is_match(s));
            if pathexclude_element.is_some() {
                // -pathexclude=[PATH1:PATH2:PATH3:...] => [PATH1, PATH2, PATH3, ...]
                exclude_paths = pathexclude_element
                        .unwrap()
                        .strip_prefix("-pathexclude=")
                        .unwrap()
                        .trim_matches(|c| c == '[' || c == ']')
                        .split(":")
                        .map(|s| s.to_string())
                        .collect();
            }

            if extexclude_element.is_some() {
                // -extexclude=[EXT1:EXT2:EXT3:...] => [EXT1, EXT2, EXT3, ...]
                exclude_exts = extexclude_element
                        .unwrap()
                        .strip_prefix("-extexclude=")
                        .unwrap()
                        .trim_matches(|c| c == '[' || c == ']')
                        .split(":")
                        .map(|s| s.to_string())
                        .collect();
            }
        }
    }

    for i in 0..exclude_paths.len() {
        if !exclude_paths[i].starts_with(".\\") {
            exclude_paths[i] = format!("{}{}", ".\\", exclude_paths[i]);
        }
        exclude_paths[i] = exclude_paths[i].replace('/', "\\");
    }

    let source_code_extensions: Vec<&str> = vec![
    "c", "rs", "cpp", "h", "hpp", "py", "dart", "java", "cs", "sql", 
    "yaml", "json", "R", "css", "js", "bat", "sh", "cmake", "Makefile", 
    "mk", "glsl", "hlsl", "txt", "cfg", "config", "ini", "xml", "frag", 
    "vert", "md"];

    let entries: Vec<DirEntry> = WalkDir::new(".").into_iter().filter_map(|e| e.ok() ).collect();
    let mut entries_to_analyze : Vec<PathBuf> = vec![];
    for entry in entries {
        let entry_extension = entry.path().extension();
        let entry_str = entry.path().to_str().unwrap();
        let mut entry_is_in_exclude_paths: bool = false;

        for path in &exclude_paths {
            if entry_str.starts_with(&*path) {
                entry_is_in_exclude_paths = true;
                break;
            }
        }

        if (!entry_extension.is_none()) && 
            (source_code_extensions.contains(&entry_extension.unwrap().to_str().unwrap())) &&
            !entry_is_in_exclude_paths && 
            !exclude_exts.contains(&entry_extension.unwrap().to_str().unwrap().to_string()) {
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
    println!("You can use clc -help for more info.");

    Ok(())
}