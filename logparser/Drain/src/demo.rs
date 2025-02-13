use std::path::Path;
// 假设 LogParser 是在 parser 模块中的一个公开结构体
use parser::generate_logformat_regex;

fn main() {
    let input_dir = "../../data/loghub_2k/HDFS/"; // 输入日志文件目录
    let output_dir = "demo_result/"; // 解析结果输出目录
    let log_file = "HDFS_2k.log"; // 输入日志文件名
    let log_format = "<Date> <Time> <Pid> <Level> <Component>: <Content>"; // HDFS 日志格式
    let regex = vec![
        r"blk_(|-)[0-9]+", // block id
        r"(/|)([0-9]+\.{3}[0-9]+(:[0-9]+|))(:|)", // IP
        r"(?<=[^A-Za-z0-9])(\-?\+?\d+)(?=[^A-Za-z0-9])|[0-9]+$", // Numbers
    ];
    let st = 0.5; // 相似度阈值
    let depth = 4; // 所有叶子节点的深度

    let parser = LogParser::new(
        log_format.to_string(),
        Path::new(input_dir).to_path_buf(),
        Path::new(output_dir).to_path_buf(),
        depth,
        st,
        regex,
    );

    parser.parse(log_file);
}