use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn read_grid_to_vec(path: &String) -> Vec<i32> {

    let grid_string = fs::read_to_string(path).unwrap();
    let grid = grid_string.split("\n").map(|x| x.parse::<i32>().unwrap_or_default()).collect::<Vec<_>>();

    grid

}

fn grid_to_index_value(grid_value: &String) -> i32 {

    let grid_vec = grid_value.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let out: i32 = grid_vec.iter().enumerate().map(|(i, x)| if i == 0 { 10 * (x - 1) } else { *x }).sum();

    out - 1

}

fn update_score(path: &Path, info_name: &str) {
    let score_path = path.join("SCORE");

    let mut score: i32 = if score_path.exists() {
        let score_content = fs::read_to_string(&score_path).unwrap();
        score_content.trim().parse().unwrap_or(0)
    } else {
        0
    };

    // println!("POINTS BEFORE FOR {:?} {:?}", info_name, score);

    score += 1;

    // println!("POINTS AFTER FOR {:?} {:?}", info_name, score);

    println!("{:?} WON\n", info_name);

    let mut file = fs::File::create(score_path).unwrap();
    writeln!(file, "{}", score).unwrap();

}

fn get_output(path: &Path, info: &String, prev: &String, correct_left: i32) -> String {

    let mut temp_path = path.to_str().unwrap().to_string();

    match info.as_str() {

        "RUST" | "C" | "CPP" | "C++" => {

            let _ = temp_path.push_str("/main");

            let mut cmd_out = Command::new(temp_path)
                .current_dir(path)
                .args([prev.to_string(), correct_left.to_string()])
                .output().unwrap().stdout;

            if *cmd_out.last().unwrap_or(&10) == 10 as u8 {
                cmd_out.pop();
            }

            let out = String::from_utf8(cmd_out).unwrap();

            return out;

        }

        "PYTHON" => {

            let mut cmd_out = Command::new("python")
                .current_dir(path)
                .args(["main.py".to_string(), prev.to_string(), correct_left.to_string()])
                .output().unwrap().stdout;

            if *cmd_out.last().unwrap_or(&10) == 10 as u8 {
                cmd_out.pop();
            }

            let out = String::from_utf8(cmd_out).unwrap();

            return out;

        }

        "JS" => {

            let mut cmd_out = Command::new("node")
                .current_dir(path)
                .args(["main.js".to_string(), prev.to_string(), correct_left.to_string()])
                .output().unwrap().stdout;

            if *cmd_out.last().unwrap_or(&10) == 10 as u8 {
                cmd_out.pop();
            }

            let out = String::from_utf8(cmd_out).unwrap();

            return out;

        }

        "JAVA" => {

            let mut cmd_out = Command::new("java")
                .current_dir(path)
                .args(["main.java".to_string(), prev.to_string(), correct_left.to_string()])
                .output().unwrap().stdout;

            if *cmd_out.last().unwrap_or(&10) == 10 as u8 {
                cmd_out.pop();
            }

            let out = String::from_utf8(cmd_out).unwrap();

            return out;

        }

        _ => {  }
    }

    "NONE".to_string()

}

fn compete(path: &Path) {

    let mut a_correct_guess = 17;
    let mut b_correct_guess = 17;

    let mut a_guess_list: HashMap<i32, i32> = HashMap::new();
    let mut b_guess_list: HashMap<i32, i32> = HashMap::new();

    let mut a_dis: bool = false;
    let mut b_dis: bool = false;

    let mut a_out: String;
    let mut b_out: String;

    for (index_a, entry_a) in fs::read_dir(path).unwrap().enumerate() {

        let path_a = entry_a.unwrap().path();

        let mut info_a_path = path_a.as_path().to_str().unwrap().to_string();
        info_a_path.push_str("/INFO");

        let mut a_vec_path = path_a.as_path().to_str().unwrap().to_string();
        a_vec_path.push_str("/GRID");

        let mut info_a_read = fs::read_to_string(&info_a_path).unwrap();

        info_a_read.pop();

        let info_a = info_a_read.split("\n").last().unwrap().to_string();
        let info_name_a_vec = info_a_read.split("\n").collect::<Vec<_>>();
        let info_name_a = info_name_a_vec.get(0).unwrap();

        for (index_b, entry_b) in fs::read_dir(path).unwrap().enumerate() {

            let mut a_out_prev: String = "NONE".to_string();
            let mut b_out_prev: String = "NONE".to_string();

            if index_a != index_b {

                let path_b = entry_b.unwrap().path();

                let mut info_b_path = path_b.as_path().to_str().unwrap().to_string();
                info_b_path.push_str("/INFO");

                let mut b_vec_path = path_b.as_path().to_str().unwrap().to_string();
                b_vec_path.push_str("/GRID");

                let mut info_b_read = fs::read_to_string(&info_b_path).unwrap();
                info_b_read.pop();
                let info_b = info_b_read.split("\n").last().unwrap().to_string();

                let info_name_b_vec = info_b_read.split("\n").collect::<Vec<_>>();
                let info_name_b = info_name_b_vec.get(0).unwrap();

                let a_vec = read_grid_to_vec(&a_vec_path);
                let b_vec = read_grid_to_vec(&b_vec_path);

                while a_correct_guess != 0 || b_correct_guess != 0 {

                    a_out = get_output(&path_a, &info_a.to_uppercase(), &a_out_prev, a_correct_guess);
                    b_out = get_output(&path_b, &info_b.to_uppercase(), &b_out_prev, b_correct_guess);

                    let a_index_value = grid_to_index_value(&a_out);
                    let b_index_value = grid_to_index_value(&b_out);

                    if a_guess_list.get(&a_index_value).is_none() {
                        a_guess_list.insert(a_index_value, 1);
                    } else {
                        a_dis = true;
                    }

                    if b_guess_list.get(&b_index_value).is_none() {
                        b_guess_list.insert(b_index_value, 1);
                    } else {
                        b_dis = true;
                    }

                    if a_dis == true && b_dis == true {
                        println!("A & B WERE DQ");
                        break;
                    } else if a_dis == true {
                        println!("A WAS DQ");
                        update_score(&path_b, info_name_b);
                        break;
                    } else if b_dis == true {
                        println!("B WAS DQ");
                        update_score(&path_a, info_name_a);
                        break;
                    }

                    if *b_vec.get(a_index_value as usize).unwrap() == 1 {
                        a_correct_guess -= 1;
                        if a_correct_guess <= 0 {
                            break;
                        }
                        a_out_prev = "HIT".to_string();
                        println!("A GUESSED AT {:?}", a_out);
                        println!("A GUESSED CORRECTLY AT {:?} LEFT {:?}\n", a_out, a_correct_guess);
                    } else {
                        println!("A GUESSED AT {:?}", a_out);
                        println!("A MISSED AT {:?}\n", a_out);
                        a_out_prev = "MISS".to_string();
                    }

                    if *a_vec.get(b_index_value as usize).unwrap() == 1 {
                        b_correct_guess -= 1;
                        if b_correct_guess <= 0 {
                            break;
                        }
                        b_out_prev = "HIT".to_string();
                        println!("B GUESSED AT {:?}", b_out);
                        println!("B GUESSED CORRECTLY AT {:?} LEFT {:?}\n", b_out, b_correct_guess);
                    } else {
                        println!("B GUESSED AT {:?}", b_out);
                        println!("B MISSED AT {:?}\n", b_out);
                        b_out_prev = "MISS".to_string();
                    }

                }

                if a_correct_guess == 0 || a_correct_guess < 0 {
                    update_score(&path_a, info_name_a);
                } else if b_correct_guess == 0 || b_correct_guess < 0 {
                    update_score(&path_b, info_name_b);
                }

                a_correct_guess = 17;
                b_correct_guess = 17;

                a_guess_list.clear();
                b_guess_list.clear();

                a_dis = false;
                b_dis = false;

            }

        }

    }

}

fn main() {

    compete(Path::new("/home/arhant/JIT_CUP/playground/"));

}
