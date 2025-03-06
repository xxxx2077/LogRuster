mod parser;
use parser::LogParser;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{
    // 日志文件路径
    let indir = "../../data/loghub_2k/Zookeeper/";

    let outdir = "demo_result/";

    let log_name = "Zookeeper_2k.log";
    
    let log_format = r"<Date> <Time> - <Level>  \[<Node>:<Component>@<Id>\] - <Content>";

    let regex_patterns = vec![
        r"(/|)(\d+\.){3}\d+(:\d+)?".to_string(),
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

    // let (headers, regex) = log_parser.generate_logformat_regex(log_format);
    // println!("{:?}",headers);
    // println!("{:?}",regex);

    log_parser.parse(log_name.to_string())
}