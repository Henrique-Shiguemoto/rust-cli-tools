use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Error;
use std::io::Write;

fn usage(){
	println!("USAGE: rmt <PROJECT-NAME>");
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 { 
        usage(); 
    } else {
    	let readme_contents_string: String = format!("# {project_name}\n
Short description...\n
## Features\n
- Feature 1\n
- Feature 2\n
## How to use\n
- Step 1\n
- Step 2\n
## Prerequisites\n
- Prerequisite 1\n
- Prerequisite 2\n
## Technologies Used\n
- Technology 1\n
- Technology 2\n", project_name = args[1]);
    	let readme_contents: &str = readme_contents_string.as_str();

    	let file_path: &Path = Path::new("README.md");

    	let file: Result<File, Error> = File::create(&file_path);
    	if file.is_ok() {
    		if file.unwrap().write(readme_contents.as_bytes()).unwrap() != readme_contents.len() {
    			panic!("Number of bytes written is not equal to the size of contents");
    		}
    	} else {
    		panic!("Couldn't create README.md in this directory");
    	}
    }
}