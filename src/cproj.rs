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
"#include <stdio.h>

int main(void){
	printf(\"Hello World!\\n\");	
	return 0;
}
";

		let gitignore : &str = 
"# Prerequisites
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

		let build_bat : &str = 
"@echo off

:: Set up directories
set SRC_DIR=src
set BIN_DIR=bin
set OUTPUT_EXE=main.exe

:: Create the BIN directory if it doesn't exist
if not exist %BIN_DIR% mkdir %BIN_DIR%

:: Clear the BIN directory of old object files
del /q %BIN_DIR%\\*.o > nul 2>&1

:: Compile all .c files in the src directory
for %%f in (%SRC_DIR%\\*.c) do (
    echo Compiling %%f...
    gcc -c %%f -o %BIN_DIR%\\%%~nf.o
    if errorlevel 1 goto :error
)

:: Link all .o files in the BIN directory to create the executable
cd %BIN_DIR%
echo Linking...
gcc *.o -o %OUTPUT_EXE%
if errorlevel 1 goto :error
cd ..

echo Compilation and linking successful! Executable: %BIN_DIR%\\%OUTPUT_EXE%
goto :eof

:error
echo Error occurred during compilation or linking.
exit /b 1
";

		let clean_bat: &str = 
"@echo off

:: Set the bin directory
set BIN_DIR=bin

:: Check if the bin directory exists
if exist %BIN_DIR% (
    echo Deleting *.o and *.exe files in %BIN_DIR%...
    del /q %BIN_DIR%\\*.o > nul 2>&1
    del /q %BIN_DIR%\\*.exe > nul 2>&1

    echo Removing %BIN_DIR% directory...
    rmdir /s /q %BIN_DIR%
    echo Clean-up completed.
) else (
    echo %BIN_DIR% does not exist. Nothing to clean.
)

exit /b
";

		let build_sh: &str = 
"SRC_DIR=\"src\"
BIN_DIR=\"bin\"
OUTPUT_EXE=\"main\"

# Create the bin directory if it doesn't exist
if [ ! -d \"$BIN_DIR\" ]; then
    mkdir -p \"$BIN_DIR\"
fi

# Compile all .c files in the src directory
echo \"Compiling source files...\"
for file in $SRC_DIR/*.c; do
    if [ -f \"$file\" ]; then
        gcc -c \"$file\" -o \"$BIN_DIR/$(basename \"$file\" .c).o\"
        if [ $? -ne 0 ]; then
            echo \"Error: Compilation failed for $file\"
            exit 1
        fi
    fi
done

# Link all .o files to create the executable
echo \"Linking object files...\"
gcc $BIN_DIR/*.o -o \"$BIN_DIR/$OUTPUT_EXE\"
if [ $? -ne 0 ]; then
    echo \"Error: Linking failed.\"
    exit 1
fi

echo \"Build successful. Executable: $BIN_DIR/$OUTPUT_EXE\"
";

		let clean_sh: &str = 
"BUILD_DIR=\"build\"

if [ -d \"$BUILD_DIR\" ]; then
    echo \"Cleaning build directory...\"
    rm -rf \"$BUILD_DIR\"
    echo \"Build directory removed.\"
else
    echo \"Build directory does not exist. Nothing to clean.\"
fi
";
	
		let todo_txt: &str = 
"- TODO 1
- TODO 2
- TODO 3
";

		let license: &str = 
"Copyright © 2025 Henrique Shiguemoto Felizardo

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
";

		let project_folder_path: &str = &args[1].clone();

		let src_path = &format!("./{project_folder_path}/src").to_string();
		let full_path_readme = &format!("./{project_folder_path}/README.md").to_string();
		let full_path_src_main_c = &format!("./{project_folder_path}/src/main.c").to_string();
		let full_path_gitignore = &format!("./{project_folder_path}/.gitignore").to_string();
		let full_path_todo = &format!("./{project_folder_path}/TODO.txt").to_string();
		let full_path_license = &format!("./{project_folder_path}/LICENSE").to_string();
		let full_path_build_bat = &format!("./{project_folder_path}/build.bat").to_string();
		let full_path_clean_bat = &format!("./{project_folder_path}/clean.bat").to_string();
		let full_path_build_sh = &format!("./{project_folder_path}/build.sh").to_string();
		let full_path_clean_sh = &format!("./{project_folder_path}/clean.sh").to_string();

		let vec_file_contents_files_paths: Vec<(&str, &str)> = vec![
			(readme_contents, full_path_readme),
			(src_main_c, full_path_src_main_c),
			(gitignore, full_path_gitignore),
			(todo_txt, full_path_todo),
			(license, full_path_license),
			(build_bat, full_path_build_bat),
			(clean_bat, full_path_clean_bat),
			(build_sh, full_path_build_sh),
			(clean_sh, full_path_clean_sh),
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