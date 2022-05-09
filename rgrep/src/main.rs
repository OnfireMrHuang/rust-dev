/// 实现rgrep需求
/// 1、命令行支持【文本匹配规则：正则表达式】,【正则表达式匹配文件名】
/// 2、文件读取，使用std::fs
extern crate clap;

use anyhow::Result;
use clap::{Arg, Command};
use regex::Regex;
use std::fs;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("rgrep")
        .version("0.1")
        .author("MrHuang")
        .arg(Arg::new("text").index(1))
        .arg(Arg::new("file").index(2))
        .get_matches();

    let txt_reg = matches.value_of("text").unwrap();
    let file_reg = matches.value_of("file").unwrap();
    let file_re = Regex::new(file_reg).unwrap();
    let txt_re = Regex::new(txt_reg).unwrap();

    // println!("text: {}", txtReg);
    // println!("file: {}", fileReg);

    // 使用正则表达式模糊匹配文件
    let dir = fs::read_dir(".").unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();

        if !file_re.is_match(file_name.as_str()) {
            continue;
        }
        // println!("{:?}", file_name);

        // 开始读取文件内容
        let mut file = fs::File::open(file_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        // println!("{}", contents);

        // 开始使用正则表达式匹配文本
        // let caps = txt_re.captures_iter(contents.as_str());
        // for cap in caps {
        //     println!("{:?}", cap);
        // }
        let mat = txt_re.find(contents.as_str()).unwrap();
        println!("{}:{}:{}", mat.start(), mat.end(), mat.as_str());
    }
    Ok(())
}

// 对比实现
// use anyhow::Result;
// use clap::Parser;
// use rgrep::*;

// fn main() -> Result<()> {
//     let config: GrepConfig = GrepConfig::parse();
//     config.match_with_default_strategy()?;

//     Ok(())
// }
