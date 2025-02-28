use std::fs;
use std::path::Path;
use std::io;
use std::env;
use std::ffi::OsString;
use std::fs::FileType;
use colored::{Colorize,ColoredString};
use mime_guess::from_path;

fn get_files(dir_path: &str, flags: &Flags) -> io::Result<Vec<(OsString,FileType)>> {
    //convert to Path obj
    let path = Path::new(dir_path);

    //check if directory
    if !path.is_dir(){
        return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Provided path is not a directory"
        ));
    }

    //contents of directory
    let mut file_list = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();

        if flags.ignore_hidden {
            if !is_hidden_folder(entry.path().as_path()) {
                if let Ok(file_type) = entry.file_type() {
                    file_list.push((file_name,file_type));
                }
            }
        } else {
            if let Ok(file_type) = entry.file_type() {
                file_list.push((file_name,file_type));
            }
        }

    }
    Ok(file_list)
}

fn is_hidden_folder(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn get_file_mime(filename: &str) -> String {
    let file_mime = from_path(filename).first_or_octet_stream();
    let file_mime_type = file_mime.type_();
    file_mime_type.to_string()
}

fn set_file_color(filename: &str) -> ColoredString {
   match get_file_mime(filename).as_str() {
       "image" => {
           filename.to_string().blue()
       }
       "text" => {
           filename.to_string().yellow()
       }
       "application" => {
           filename.to_string().green()
       }
       "video" => {
           filename.to_string().cyan()
       }
       &_ => filename.to_string().magenta()
   }
}

fn strify_files(files: &Vec<(OsString,FileType)>) -> Vec<ColoredString> {
    let mut file_strs = Vec::new();
    for file in files {
        let (file_name,file_type) = file;
        if let Some(file_str) = file_name.to_str() {
            let mut file_str_colored = file_str.to_string().red();

            if file_type.is_dir() {
                file_str_colored = file_str_colored.bold();
            }
            else {
                file_str_colored = set_file_color(file_str);
                //file_str_colored = file_str_colored.blue();
                //println!("{:?}",get_file_mime(file_str));
            }
            file_strs.push(file_str_colored);
            //file_strs.push(file_str.to_string().red());
        }
    }
    file_strs
}

struct Flags {
    ignore_hidden: bool,
    show_size: bool,
}

fn parse_flags(user_input: &String) -> Flags {
    let mut program_flags = Flags {
        ignore_hidden: true,
        show_size: false,
    };

    if user_input.contains("a") {
        program_flags.ignore_hidden = false;
    }
    if user_input.contains("s") {
        program_flags.show_size = true;
    }

    program_flags
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut flags = parse_flags(&String::from(""));
    if args.len() >= 3 {
        flags = parse_flags(&args[2]);
    }
    match get_files(&args[1],&flags) {
        Ok(files) => {
            let file_strs = strify_files(&files);
            for file in file_strs{
                println!("{}", file);
            }
        }
        Err(e) => eprintln!("Error listing files: {}",e),
    }
}
