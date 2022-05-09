/// 实现rgrep需求
/// 1、命令行支持【文本匹配规则：正则表达式】,【正则表达式匹配文件名】
/// 2、文件读取，使用std::fs
extern crate clap;

use anyhow::Result;
use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("rgrep")
        .version("0.1")
        .author("MrHuang")
        .arg(Arg::new("text").index(1))
        .arg(Arg::new("file").index(2))
        .get_matches();

    if let Some(t) = matches.value_of("text") {
        println!("text: {}", t);
    }
    if let Some(f) = matches.value_of("file") {
        println!("file: {}", f);
    }
    Ok(())
}
