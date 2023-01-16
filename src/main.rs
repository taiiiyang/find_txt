use find_txt::FileInfo;
use std::env;

fn main() {
    // cargo run [absolute file] [dist_file] [target_file]
    let input: Vec<String> = env::args().collect();

    FileInfo::find_txt(&input);
}
