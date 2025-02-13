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