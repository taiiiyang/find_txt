### find_txt
``cargo run [absolute_path] [dist_path] [target_file_name]``

``eg: cargo run "C:/cargo" "dist.txt" "README.md"``

一个命令行脚本，用于将指定文件夹`[absolute_path]`中对应的文件内容`[target_file_name]`收集到

`[dist_path]`(同时支持相对路径或绝对路径)。

系统需要支持 cargo

在条件不满足时将无法运行

目前支持在文件夹中寻找对应文件