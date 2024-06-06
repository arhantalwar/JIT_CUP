use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn get_output(path: &Path, info: &String, round: i32, prev: &String) -> String {

    let mut temp_path = path.to_str().unwrap().to_string();

    match info.as_str() {

        "RUST" | "C" | "CPP" | "C++" => {

            let _ = temp_path.push_str("/main");

            let mut cmd_out = Command::new(temp_path)
                .args([round.to_string(), prev.to_string()])
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
                .args(["main.py".to_string(), round.to_string(), prev.to_string()])
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
                .args(["main.js".to_string(), round.to_string(), prev.to_string()])
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
                .args(["main.java".to_string(), round.to_string(), prev.to_string()])
                .output().unwrap().stdout;

            if *cmd_out.last().unwrap_or(&10) == 10 as u8 {
                cmd_out.pop();
            }

            let out = String::from_utf8(cmd_out).unwrap();

            return out;

        }

        "ELIXIR" => {

            let mut cmd_out = Command::new("elixir")
                .current_dir(path)
                .args(["main.exs".to_string(), round.to_string(), prev.to_string()])
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

fn update_score(path: &Path, state: &str, info_name: &str) {
    let score_path = path.join("score.txt");

    let mut score: i32 = if score_path.exists() {
        let score_content = fs::read_to_string(&score_path).unwrap();
        score_content.trim().parse().unwrap_or(0)
    } else {
        0
    };

    println!("POINTS BEFORE FOR {:?} {:?}", info_name, score);

    match state {
        "+3" => {
            score += 3;
        }

        "+2" => {
            score += 2;
        }

        "+0" => {
            score += 0;
        }

        "-1" => {
            score -= 1;
        }

        _ => {}
    }

    println!("POINTS AFTER FOR {:?} {:?}", info_name, score);

    let mut file = fs::File::create(score_path).unwrap();
    writeln!(file, "{}", score).unwrap();

}

fn compete(path: &Path) {

    let mut a_out: String;
    let mut b_out: String;


    for (index_a, entry_a) in fs::read_dir(path).unwrap().enumerate() {

        let path_a = entry_a.unwrap().path();

        let mut info_a_path = path_a.as_path().to_str().unwrap().to_string();
        info_a_path.push_str("/INFO");

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

                let mut info_b_read = fs::read_to_string(&info_b_path).unwrap();
                info_b_read.pop();
                let info_b = info_b_read.split("\n").last().unwrap().to_string();

                let info_name_b_vec = info_b_read.split("\n").collect::<Vec<_>>();
                let info_name_b = info_name_b_vec.get(0).unwrap();

                for round in 1..=200 {

                    a_out = get_output(&path_a, &info_a.to_uppercase(), round, &b_out_prev);
                    b_out = get_output(&path_b, &info_b.to_uppercase(), round, &a_out_prev);

                    println!("ROUND BETWEEN {:?} {:?} AND OUTPUTS {:?} {:?} ROUND NO {:?} PREV RES {:?} {:?}",
                             info_name_a, 
                             info_name_b,
                             a_out,
                             b_out,
                             round,
                             a_out_prev,
                             b_out_prev);

                    if a_out.to_uppercase() == "YES" && b_out.to_uppercase() == "YES" {
                        update_score(&path_a, "+2", info_name_a);
                        update_score(&path_b, "+2", info_name_b);
                    } else if a_out.to_uppercase() == "NO" && b_out.to_uppercase() == "NO" {
                        update_score(&path_a, "+0", info_name_a);
                        update_score(&path_b, "+0", info_name_b);
                    } else if a_out.to_uppercase() == "YES" && b_out.to_uppercase() == "NO" {
                        update_score(&path_a, "-1", info_name_a);
                        update_score(&path_b, "+3", info_name_b);
                    } else if a_out.to_uppercase() == "NO" && b_out.to_uppercase() == "YES" {
                        update_score(&path_a, "+3", info_name_a);
                        update_score(&path_b, "-1", info_name_b);
                    }

                    a_out_prev = a_out.to_uppercase();
                    b_out_prev = b_out.to_uppercase();

                }

            }

        }

        println!("\n");

    }

}

fn main() {

    compete(Path::new("/home/arhant/JIT_CUP/playground/"));


}


/* [C, CPP, Rust, JS, Java Python] */
