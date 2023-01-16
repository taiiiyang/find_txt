use std::path::Path;
use std::{fs, process};

pub struct FileInfo<'a> {
    root_path: &'a Path,
    output_file_name: &'a Path,
    target_file_name: &'a Path,
}

impl<'a> FileInfo<'a> {
    fn new(args: &[String]) -> Result<FileInfo, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let root_path = Path::new(&args[1]);
        let output_file_name = Path::new(&args[2]);
        let target_file_name = Path::new(&args[3]);

        Ok(FileInfo {
            root_path,
            output_file_name,
            target_file_name,
        })
    }

    // 找出对应的文件
    pub fn find_txt(input: &[String]) {
        let file_info = FileInfo::new(&input).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });

        let mut contents = String::new();

        find_readme_core(
            file_info.root_path,
            &mut contents,
            file_info.target_file_name,
        );

        match fs::File::create(file_info.output_file_name) {
            Ok(_) => println!("create dist file done"),
            Err(_) => eprintln!("create dist file error"),
        };

        match fs::write(file_info.output_file_name, &mut contents) {
            Ok(()) => println!("find README.md done"),
            Err(_) => eprintln!("find README error"),
        };
    }
}

// change contents
// 核心操作
fn find_readme_core(dir: &Path, contents: &mut String, target_file_name: &Path) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let buf = dir.join(path);
                find_readme_core(buf.as_path(), contents, target_file_name);
            } else {
                if path.file_name() == target_file_name.file_name() {
                    let buf = dir.join(path);
                    match fs::read_to_string(buf) {
                        Ok(content) => contents.push_str(&content),
                        Err(_) => {}
                    };
                }
            }
        }
    } else {
        eprintln!("{:?} not exist", dir);
        process::exit(1);
    }
}
