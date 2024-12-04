use std::{env, fs};

pub fn read_file(file: String) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("src").join(file);

    fs::read_to_string(filepath).unwrap()
}


pub fn get_file_path(day: i32, is_sample: bool) -> String {
    format!(
        "day_{}/{}.txt",
        day,
        if is_sample { "sample" } else { "input" }
    )
    .to_string()
}
