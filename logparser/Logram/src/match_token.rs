use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use csv::WriterBuilder;
use md5::{Md5, Digest};

fn triple_match(tokens: &[String], tri_dictionary_list: &HashMap<String, i32>, tri_threshold: i32) -> Vec<usize> {
    let mut index_list = HashMap::new();

    for index in 0..tokens.len() {
        if index >= tokens.len() - 2 {
            break;
        }
        let triple_tmp = format!("{}^{}^{}", tokens[index], tokens[index + 1], tokens[index + 2]);

        if !tri_dictionary_list.contains_key(&triple_tmp) || tri_dictionary_list[&triple_tmp] < tri_threshold {
            index_list.insert(index, 1);
            index_list.insert(index + 1, 1);
            index_list.insert(index + 2, 1);
        }
    }

    // 将哈希表中的键转换为向量并返回
    index_list.keys().cloned().collect()
}

fn double_match(tokens: &[String], index_list: &[usize], double_dictionary_list: &HashMap<String, i32>, double_threshold: i32, length: usize) -> Vec<usize> {
    let mut dynamic_index = Vec::new();

    for &index in index_list.iter() {
        if index == 0 {
            let double_tmp = format!("{}^{}", tokens[index], tokens[index + 1]);
            if !double_dictionary_list.contains_key(&double_tmp) || double_dictionary_list[&double_tmp] <= double_threshold {
                dynamic_index.push(index);
            }
        } else if index == length - 1 {
            let double_tmp1 = format!("{}^{}", tokens[index - 1], tokens[index]);
            let double_tmp2 = format!("{}^{}", tokens[index], tokens[0]);

            if (!double_dictionary_list.contains_key(&double_tmp1) || double_dictionary_list[&double_tmp1] < double_threshold)
                && (!double_dictionary_list.contains_key(&double_tmp2) || double_dictionary_list[&double_tmp2] < double_threshold) {
                dynamic_index.push(index);
            }
        } else {
            let double_tmp1 = format!("{}^{}", tokens[index], tokens[index + 1]);
            let double_tmp2 = format!("{}^{}", tokens[index - 1], tokens[index]);

            if (!double_dictionary_list.contains_key(&double_tmp1) || double_dictionary_list[&double_tmp1] < double_threshold)
                && (!double_dictionary_list.contains_key(&double_tmp2) || double_dictionary_list[&double_tmp2] < double_threshold) {
                dynamic_index.push(index);
            }
        }
    }

    dynamic_index
}

pub fn token_match(
    all_tokens_list: &[Vec<String>],
    double_dictionary_list: &HashMap<String, i32>,
    tri_dictionary_list: &HashMap<String, i32>,
    double_threshold: i32,
    tri_threshold: i32,
    outdir: &str,
    log_file_basename: &str,
    all_message_list: &[String],
) {
    // 创建输出目录
    create_dir_all(outdir).expect("Failed to create output directory");

    let template_file = Path::new(outdir).join(format!("{}_templates.csv", log_file_basename));
    let structured_log_file = Path::new(outdir).join(format!("{}_structured.csv", log_file_basename));

    let mut template_lines = HashSet::new();
    let mut structured_log_lines = Vec::new();

    assert_eq!(all_tokens_list.len(), all_message_list.len());

    for (index, tokens) in all_tokens_list.iter().enumerate() {
        let message = &all_message_list[index];
        let index_list = triple_match(tokens, tri_dictionary_list, tri_threshold);
        let dynamic_index = double_match(tokens, &index_list, double_dictionary_list, double_threshold, tokens.len());

        let mut log_event = String::new();
        for (i, token) in tokens.iter().enumerate() {
            if dynamic_index.contains(&i) {
                log_event.push_str("<*> ");
            } else {
                log_event.push_str(&format!("{} ", token));
            }
        }

        // 移除逗号并去除首尾空格
        log_event = regex::Regex::new(r",")
            .unwrap()
            .replace_all(&log_event.trim(), "")
            .to_string();

        // 计算 MD5 哈希并截取前 8 个字符
        let hash = Md5::digest(log_event.as_bytes());
        let template_id: String = hash.iter().take(8).map(|b| format!("{:02x}", b)).collect();
        // let template_id = format!("{:x}", Md5::digest(log_event.as_bytes())[..8].iter().collect::<String>());

        if !template_lines.contains(&(template_id.clone(), log_event.clone())) {
            template_lines.insert((template_id.clone(), log_event.clone()));
        }

        structured_log_lines.push((index + 1, message.clone(), template_id, log_event));
    }

    // 写入模板文件
    let mut template_writer = WriterBuilder::new().from_path(template_file).expect("Failed to create template file writer");
    for (event_id, event_template) in template_lines.iter() {
        template_writer.serialize((event_id, event_template)).expect("Failed to write to template file");
    }

    // 写入结构化日志文件
    let mut structured_log_writer = WriterBuilder::new().from_path(structured_log_file).expect("Failed to create structured log file writer");
    for (line_id, content, event_id, event_template) in structured_log_lines.iter() {
        structured_log_writer.serialize((line_id, content, event_id, event_template)).expect("Failed to write to structured log file");
    }
}
