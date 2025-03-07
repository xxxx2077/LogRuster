use crate::common::{regex_generator,token_spilter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use fancy_regex::Regex as FancyRegex;
use std::error::Error;

pub fn dictionary_builder(log_format:String, log_file_path:PathBuf, preprocess_regex:Vec<FancyRegex>)-> std::result::Result<(), Box<dyn Error>>{

    // 打开文件并创建一个 BufReader
    let file = File::open(log_file_path)?;
    let reader = BufReader::new(file);

    let regex = regex_generator(&log_format);
    //*test */
    // println!("regex = {:?}", regex);
    // println!("preprocess_regex = {:?}", preprocess_regex);

    // 逐行读取并处理每一行
    for line in reader.lines() {
        let log_line = line?;
        //*test */
        // println!("{}", log_line.trim());
        let (tokens, messages) = token_spilter(&log_line, &regex, &preprocess_regex);
        //*test */
        // println!("token = {:?}, messages = {:?}", tokens, messages);
        // print!("\n")
    }

    Ok(())
}