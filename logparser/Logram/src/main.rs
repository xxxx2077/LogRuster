mod common;
mod logram;
mod dictionary;
mod match_token;
use logram::LogParser;

use std::error::Error;

fn main()-> std::result::Result<(), Box<dyn Error>>{
    // 日志文件路径
    let indir = "../../data/loghub_2k/HDFS/";

    let outdir = "demo_result/";

    let log_name = "HDFS_2k.log";
    
    let log_format = "<Date> <Time> <Pid> <Level> <Component>: <Content>";

    let regex_patterns = vec![
        r"blk_(|-)[0-9]+".to_string(),
        r"(/|)([0-9]+\.){3}[0-9]+(:[0-9]+|)(:|)".to_string(), 
        r"(?<=[^A-Za-z0-9])(\-?\+?\d+)(?=[^A-Za-z0-9])|[0-9]+$".to_string(), 
    ];

    let double_threshold = 15;
    let tri_threshold = 10;

    let log_parser = LogParser::new(
        Some(indir.to_string()),
        Some(outdir.to_string()),
        Some(double_threshold),
        Some(tri_threshold),
        log_format.to_string(),
        regex_patterns,
    );

    log_parser.parse(log_name.to_string())
}