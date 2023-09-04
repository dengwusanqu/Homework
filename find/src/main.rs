// ⽀持同时搜索多个 path
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查参数数量是否足够
    if args.len() < 3 {
        eprintln!("使用方式：{} <目标目录1> <目标目录2> ... <要搜索的正则表达式>", args[0]);
        process::exit(1);
    }

    // 获取要搜索的正则表达式
    let pattern = &args.last().expect("缺少正则表达式参数");
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效的正则表达式 '{}':{}", pattern, err);
            process::exit(1);
        }
    };

    // 获取目标目录参数
    let target_directories: Vec<&str> = args[1..args.len() - 1].iter().map(|s| s.as_str()).collect();

    // 遍历目标目录并执行搜索
    for dir in &target_directories {
        match find(dir, &regex) {
            Ok(matches) => {
                if matches.is_empty() {
                    println!("在目录 '{}' 中未找到匹配项。", dir);
                } else {
                    println!("在目录 '{}' 中找到以下匹配项：", dir);
                    for file in &matches {
                        println!("{}", file);
                    }
                }
            }
            Err(error) => {
                eprintln!("在目录 '{}' 中发生错误：{}", dir, error);
                process::exit(1);
            }
        }
    }
}

// 递归搜索文件匹配正则表达式的函数
fn find<P: AsRef<Path>>(root: P, regex: &Regex) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, &mut matches)?;
    Ok(matches)
}

// 递归遍历目录树，查找匹配的文件名
fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}
