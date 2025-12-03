use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

type Coordinate = (i32, i32);

fn read_grid_to_hashmap(path: &str) -> HashMap<i32, HashSet<Coordinate>> {
    let mut ship_data: HashMap<i32, HashSet<Coordinate>> = HashMap::new();
    let grid_string = fs::read_to_string(path).unwrap();

    for line in grid_string.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let ship_type: i32 = parts[0].parse().expect("Invalid ship type");
            let coords_str = &parts[1][1..parts[1].len() - 1]; // Remove the outer brackets
            let coords: HashSet<Coordinate> = coords_str
                .split("), (")
                .map(|s| {
                    let coord_parts: Vec<&str> = s.trim_matches(&['(', ')'][..]).split(", ").collect();
                    (
                        coord_parts[0].parse().expect("Invalid coordinate"),
                        coord_parts[1].parse().expect("Invalid coordinate"),
                    )
                })
                .collect();

            ship_data.insert(ship_type, coords);
        }
    }

    ship_data

}

fn find_and_remove_coordinate(ship_data: &mut HashMap<i32, HashSet<Coordinate>>, coord: Coordinate) -> Option<i32> {
    for (&ship_type, coords) in ship_data.iter_mut() {
        if coords.remove(&coord) {
            return Some(ship_type);
        }
    }
    None
}


fn update_score(path: &Path, info_name: &str) {
    let score_path = path.join("SCORE");

    let mut score: i32 = if score_path.exists() {
        let score_content = fs::read_to_string(&score_path).unwrap();
        score_content.trim().parse().unwrap_or(0)
    } else {
        0
    };

    score += 1;

    println!("{:?} WON\n", info_name);

    let mut file = fs::File::create(score_path).unwrap();
    writeln!(file, "{}", score).unwrap();

}

fn get_output(path: &Path, info: &String, prev: &String, ship_destroyed: &String) -> String {

    let mut temp_path = path.to_str().unwrap().to_string();

    match info.as_str() {

        "RUST" | "C" | "CPP" | "C++" => {

            let _ = temp_path.push_str("/main");

            let mut cmd_out = Command::new(temp_path)
                .current_dir(path)
                .args([prev.to_string(), ship_destroyed.to_string()])
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
                .args(["main.py".to_string(), prev.to_string(), ship_destroyed.to_string()])
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
                .args(["main.js".to_string(), prev.to_string(), ship_destroyed.to_string()])
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
                .args(["main.java".to_string(), prev.to_string(), ship_destroyed.to_string()])
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

    let mut a_guess_list: HashMap<Coordinate, i32> = HashMap::new();
    let mut b_guess_list: HashMap<Coordinate, i32> = HashMap::new();

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

            let mut a_ship_dest: String = "NONE".to_string();
            let mut b_ship_dest: String = "NONE".to_string();

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

                let mut a_hash = read_grid_to_hashmap(&a_vec_path);
                let mut b_hash = read_grid_to_hashmap(&b_vec_path);

                while a_correct_guess != 0 || b_correct_guess != 0 {

                    a_out = get_output(&path_a, &info_a.to_uppercase(), &a_out_prev, &a_ship_dest);
                    b_out = get_output(&path_b, &info_b.to_uppercase(), &b_out_prev, &b_ship_dest);

                    let a_out_vec = a_out.split(" ").collect::<Vec<&str>>();
                    let b_out_vec = b_out.split(" ").collect::<Vec<&str>>();

                    if !a_guess_list.contains_key(&(a_out_vec[0].parse().unwrap(), a_out_vec[1].parse().unwrap())) {
                        a_guess_list.insert((a_out_vec[0].parse().unwrap(), a_out_vec[1].parse().unwrap()), 1);
                    } else {
                        a_dis = true;
                    }

                    if !b_guess_list.contains_key(&(b_out_vec[0].parse().unwrap(), b_out_vec[1].parse().unwrap())) {
                        b_guess_list.insert((b_out_vec[0].parse().unwrap(), b_out_vec[1].parse().unwrap()), 1);
                    } else {
                        b_dis = true;
                    }

                    if a_dis == true && b_dis == true {
                        println!("{:?} & {:?} WERE DQ FOR REPEATING CO-ORDINATES {:?} {:?}", info_name_a, info_name_b, a_out, b_out);
                        break;
                    } else if a_dis == true {
                        println!("{:?} WAS DQ FOR REPEATING CO-ORDINATES {:?}", info_name_a, a_out);
                        update_score(&path_b, info_name_b);
                        break;
                    } else if b_dis == true {
                        println!("{:?} WAS DQ FOR REPEATING CO-ORDINATES {:?}", info_name_b, b_out);
                        update_score(&path_a, info_name_a);
                        break;
                    }

                    if let Some(_) = find_and_remove_coordinate(&mut a_hash, 
                                                               (b_out_vec[0].parse().unwrap(), 
                                                                b_out_vec[1].parse().unwrap()))  {
                        b_correct_guess -= 1;

                        if b_correct_guess <= 0 {
                            break;
                        }

                        b_out_prev = "HIT".to_string();

                        println!("{:?} GUESSED CORRECTLY AT {:?} LEFT {:?}\n", info_name_b, b_out, b_correct_guess);

                        let a_hash_filter_vec: Vec<i32> = a_hash.iter().filter_map(|(&key, set)| {
                            if set.len() == 0 {
                                Some(key)
                            } else {
                                None
                            }
                        }).collect();

                        if let Some(t) = a_hash_filter_vec.get(0) {
                            a_hash.remove(&t);
                            a_ship_dest = t.to_string();
                            println!("{:?} DESTROYED SHIP {:?}\n", info_name_b, t);
                        } else {
                            a_ship_dest = "NONE".to_string();
                        }

                    } else {

                        b_out_prev = "MISS".to_string();
                        a_ship_dest = "NONE".to_string();

                        println!("{:?} GUESSED AND MISSED AT {:?}", info_name_b, b_out);

                    }

                    if let Some(_) = find_and_remove_coordinate(&mut b_hash, 
                                                               (a_out_vec[0].parse().unwrap()
                                                               ,a_out_vec[1].parse().unwrap()))  {
                        a_correct_guess -= 1;

                        if a_correct_guess <= 0 {
                            break;
                        }

                        a_out_prev = "HIT".to_string();

                        println!("{:?} GUESSED AT {:?}", info_name_a, a_out);
                        println!("{:?} GUESSED CORRECTLY AT {:?} LEFT {:?}\n", info_name_a, a_out, a_correct_guess);

                        let b_hash_filter_vec: Vec<i32> = b_hash.iter().filter_map(|(&key, set)| {
                            if set.len() == 0 {
                                Some(key)
                            } else {
                                None
                            }
                        }).collect();

                        if let Some(t) = b_hash_filter_vec.get(0) {
                            b_hash.remove(&t);
                            b_ship_dest = t.to_string();
                            println!("{:?} DESTROYED SHIP {:?}\n", info_name_a, t);
                        } else {
                            b_ship_dest = "NONE".to_string();
                        }
                        
                    } else {

                        a_out_prev = "MISS".to_string();
                        b_ship_dest = "NONE".to_string();

                        println!("{:?} GUESSED AND MISSED AT {:?}", info_name_a, a_out);

                    }

                }

                if a_correct_guess <= 0 {
                    update_score(&path_a, info_name_a);
                } else if b_correct_guess <= 0 {
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
