use regex::Regex;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::process;

mod parser;
use parser::log_to_dataframe;

fn main() {
    // 日志文件路径
    let log_file = "../../data/loghub_2k/HDFS/HDFS_2k.log";
    
    // 正则表达式来匹配日志
    let log_format_regex = Regex::new(r"(?P<Date>.*?) (?P<Time>.*?) (?P<Pid>.*?) (?P<Level>.*?) (?P<Component>.*?)\: (?P<Content>.*?)").unwrap();
    
    // 定义表头
    let headers = ["LineId", "Date", "Time", "Pid", "Level", "Component", "Content"];
    
    // 调用 log_to_dataframe 函数
    if let Err(e) = log_to_dataframe(log_file, &log_format_regex, &headers) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}