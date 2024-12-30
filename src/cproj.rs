use std::fs;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Error;
use std::io::Write;

fn usage(){
	println!("USAGE: cproj <PROJECT-NAME>");
}

fn main() -> std::io::Result<()> {
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

    let src_main_c: &str = 
"
#include <stdio.h>

int main(void){
	printf(\"Hello World!\n\");
	return 0;
}
";

	let gitignore : &str = 
"
# Prerequisites
*.d

# Object files
/obj
*.o
*.ko
*.obj
*.elf

# Linker output
*.ilk
*.map
*.exp

# Precompiled Headers
*.gch
*.pch

# Libraries
*.lib
*.a
*.la
*.lo

# Shared objects (inc. Windows DLLs)
*.dll
*.so
*.so.*
*.dylib

# Executables
*.exe
*.out
*.app
*.i*86
*.x86_64
*.hex

# Debug files
*.dSYM/
*.su
*.idb
*.pdb

# Kernel Module Compile Results
*.mod*
*.cmd
.tmp_versions/
modules.order
Module.symvers
Mkfile.old
dkms.conf
";

	let makefile : &str = 
"
CC = gcc
CFLAGS = -Wall -Wextra -pedantic -g -std=c11 -Wno-unused-parameter -Wno-unused-variable
TARGET = main
SRCDIR = src
OBJDIR = obj

# Find all .c files in the project
SRC = $(shell find $(SRCDIR) -name '*.c')

# Convert .c file paths to .o file paths
OBJ = $(patsubst $(SRCDIR)/%.c, $(OBJDIR)/%.o, $(SRC))

all: $(TARGET)

# Link all object files into the executable
$(TARGET): $(OBJ)
	$(CC) $(OBJ) -o $@

# Compile each .c file into an object file
$(OBJDIR)/%.o: $(SRCDIR)/%.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(OBJDIR) $(TARGET)

.PHONY: all clean
";

		let project_folder_path: &str = &args[1].clone();

		let src_path = &format!("./{project_folder_path}/src").to_string();
		let full_path_readme = &format!("./{project_folder_path}/README.md").to_string();
		let full_path_src_main_c = &format!("./{project_folder_path}/src/main.c").to_string();
		let full_path_gitignore = &format!("./{project_folder_path}/.gitignore").to_string();
		let full_path_makefile = &format!("./{project_folder_path}/Makefile").to_string();

		let vec_file_contents_files_paths: Vec<(&str, &str)> = vec![
			(readme_contents, full_path_readme),
			(src_main_c, full_path_src_main_c),
			(gitignore, full_path_gitignore),
			(makefile, full_path_makefile),
		];

		// Create directories needed first
		fs::create_dir_all(args[1].clone()).unwrap();
		fs::create_dir_all(src_path).unwrap();

		for (contents, path) in vec_file_contents_files_paths {
			let file_path: &Path = Path::new(path);

	    	let file: Result<File, Error> = File::create(&file_path);
	    	if file.is_ok() {
	    		let _ = file.unwrap().write_all(contents.as_bytes());
	    	} else {
	    		println!("FILE_NOT_OK {f}: ", f = file_path.to_str().unwrap());
	    		panic!("Couldn't create file in this directory");
	    	}	
		}
    }

    Ok(())
}