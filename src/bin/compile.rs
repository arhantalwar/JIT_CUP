use std::fs;
use std::path::Path;
use std::process::Command;

fn compile(path: &Path) {

    if path.is_dir() {

        for child_dir in fs::read_dir(path).unwrap() {
            
            for file in fs::read_dir(child_dir.unwrap().path()).unwrap() {

                let file_path = file.unwrap().path();

                let mut path_index = file_path.to_str().unwrap().split("/").collect::<Vec<&str>>();
                path_index.pop().unwrap();

                let cur_dir = path_index.join("/");

                if file_path.to_str().unwrap().contains("INFO") {

                    let info_cxt = fs::read_to_string(file_path).unwrap();
                    let lang = info_cxt.trim().split("\n").into_iter().last().unwrap();

                    match lang.to_uppercase().as_str() {

                        "RUST" => {
                            Command::new("rustc").
                                current_dir(cur_dir).
                                arg("main.rs").
                                spawn().
                                unwrap();
                        }
                        
                        "C" => {
                            Command::new("gcc").
                                current_dir(cur_dir).
                                args(["main.c", "-o", "main"]).
                                spawn().
                                unwrap();
                        }

                        "CPP" => {
                            Command::new("g++").
                                current_dir(cur_dir).
                                args(["main.cpp", "-o", "main"]).
                                spawn().
                                unwrap();
                        }

                        _ => {   }

                    }

                }

            }

        }

    }

}

fn main() {
    compile(Path::new("/home/arhant/JIT_CUP/playground/"));
}
