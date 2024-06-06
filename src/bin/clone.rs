use std::fs;
use std::path::Path;
use std::process::Command;

fn git_clone(path: &Path) {

    let string_repos = fs::read_to_string("/home/arhant/Sandbox/rusty/jit_bot/src/git_repo_list").unwrap();
    let mut list_of_repos = string_repos.split("\n").collect::<Vec<&str>>();
    list_of_repos.pop();

    for (num, url) in list_of_repos.iter().enumerate() {

        let mut parent_path = String::from(path.to_str().unwrap());
        parent_path.push_str(&num.to_string());

        let temp_path = Path::new(&parent_path);
        fs::create_dir(temp_path).unwrap();

        Command::new("git")
            .args(["clone", url, "."])
            .current_dir(temp_path)
            .spawn()
            .unwrap();

    }

}

fn main() {
    git_clone(Path::new("/home/arhant/JIT_CUP/playground/"));
}
