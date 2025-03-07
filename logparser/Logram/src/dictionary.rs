use crate::common::{regex_generator,token_spilter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use fancy_regex::Regex as FancyRegex;
use std::error::Error;
use std::collections::HashMap;

// pub fn dictionary_builder(log_format:String, log_file_path:PathBuf, preprocess_regex:Vec<FancyRegex>)-> std::result::Result<(), Box<dyn Error>>{

//     // 打开文件并创建一个 BufReader
//     let file = File::open(log_file_path)?;
//     let reader = BufReader::new(file);

//     let regex = regex_generator(&log_format);
//     //*test */
//     // println!("regex = {:?}", regex);
//     // println!("preprocess_regex = {:?}", preprocess_regex);

//     // 逐行读取并处理每一行
//     for line in reader.lines() {
//         let log_line = line?;
//         //*test */
//         // println!("{}", log_line.trim());
//         let (tokens, messages) = token_spilter(&log_line, &regex, &preprocess_regex);
//         //*test */
//         // println!("token = {:?}, messages = {:?}", tokens, messages);
//         // print!("\n")
//     }
// 
//     Ok(())
// }

pub fn dictionary_builder(log_format:String, log_file_path:PathBuf, preprocess_regex:Vec<FancyRegex>) -> (HashMap<String, i32>, HashMap<String, i32>, Vec<Vec<String>>, Vec<String>) {
    let mut double_dictionary_list = HashMap::new();
    double_dictionary_list.insert("dictionary^DHT".to_string(), -1);

    let mut tri_dictionary_list = HashMap::new();
    tri_dictionary_list.insert("dictionary^DHT^triple".to_string(), -1);

    let mut all_token_list = Vec::new();
    let mut all_message_list = Vec::new();

    let regex = regex_generator(&log_format);

    let file = File::open(log_file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let log_line = line.expect("Unable to read line");
        let (tokens, messages) = token_spilter(&log_line, &regex, &preprocess_regex);

        let messages = match messages{
            Some(messages)=>messages,
            None=> String::new(),
        };
        all_message_list.push(messages);

        let tokens = match tokens{
            Some(tokens) => tokens,
            None => {
                continue;
            }
        };

        all_token_list.push(tokens.clone());

        for index in 0..tokens.len() {
            if index >= tokens.len() - 2 {
                break;
            }
            let triple_tmp = format!("{}^{}^{}", tokens[index], tokens[index + 1], tokens[index + 2]);
            *tri_dictionary_list.entry(triple_tmp).or_insert(0) += 1;
        }

        for index in 0..tokens.len() {
            if index == tokens.len() - 1 {
                let double_tmp = format!("{}^{}", tokens[index], tokens[0]);
                *double_dictionary_list.entry(double_tmp).or_insert(0) += 1;
                break;
            }
            let double_tmp = format!("{}^{}", tokens[index], tokens[index + 1]);
            *double_dictionary_list.entry(double_tmp).or_insert(0) += 1;
        }
    }

    (double_dictionary_list, tri_dictionary_list, all_token_list, all_message_list)
}
