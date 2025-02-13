mod parser;
use parser::LogParser;

fn main() {
    // 日志文件路径
    let indir = "../../data/loghub_2k/HDFS/";

    let outdir = "demo_result/";

    let log_name = "HDFS_2k.log";
    
    let log_format = "<Date> <Time> <Pid> <Level> <Component>: <Content>";

    let regex_patterns = vec![
        r"blk_(|-)[0-9]+", // block id
        r"(/|)([0-9]+\.){3}[0-9]+(:[0-9]+|)(:|)", // IP Address
        r"(?<=[^A-Za-z0-9])(\-?\+?\d+)(?=[^A-Za-z0-9])|[0-9]+$", // Numbers
    ];

    let st = 0.5;

    let depth = 4;

    let mut log_parser = LogParser::new(
        Some(indir.to_string()),
        Some(outdir.to_string()),
        Some(depth),
        Some(st),
        None,
        log_format.to_string(),
        regex_patterns,
        None,
    );

    log_parser.parse(log_name.to_string());
}