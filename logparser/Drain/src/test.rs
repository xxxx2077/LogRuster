mod parser;

use regex::Regex;
use parser::generate_logformat_regex;

fn main() {
    let log_format = "<Date> <Time> <Pid> <Level> <Component>: <Content>";
    let (headers, regex_str) = parser::generate_logformat_regex(&log_format);
    println!("{:?}", headers);
    println!("{:?}", regex_str);
}

mod parser;

use regex::Regex;
use parser::generate_logformat_regex;

fn main() {
    let log_format = "<Date> <Time> <Pid> <Level> <Component>: <Content>";
    let (headers, regex_str) = parser::generate_logformat_regex(&log_format);
    println!("{:?}", headers);
    println!("{:?}", regex_str);
}

mod parser;

use regex::Regex;
use parser::{LogParser};
// use parser::{extract_variable_names};

fn main() {
    // 日志文件路径
    let indir = "../../data/loghub_2k/HDFS/";

    let outdir = "demo_result/";

    let log_name = "HDFS_2k.log";
    
    let log_format = "<Date> <Time> <Pid> <Level> <Component>: <Content>";

    let mut log_parser = LogParser::new(
        indir.to_string(),
        outdir.to_string(),
        log_format.to_string(),
    );

    log_parser.parse(log_name.to_string());
    let headers  = log_parser.extract_variable_names(&log_format);
    println!("{:?}", headers);
    let (headers,regex)  = log_parser.generate_logformat_regex(&log_format);
    println!("{:?}", headers);
    println!("{:?}", regex);
}

mod parser;
use parser::LogParser;

fn main() {
    // 日志文件路径
    let indir = "../../data/loghub_2k/HDFS/";

    let outdir = "demo_result/";

    let log_name = "HDFS_2k.log";
    
    let log_format = "<Date> <Time> <Pid> <Level> <Component>: <Content>";

    let mut log_parser = LogParser::new(
        indir.to_string(),
        outdir.to_string(),
        log_format.to_string(),
    );

    log_parser.parse(log_name.to_string());
}