use regex::Regex;
use fancy_regex::Regex as FancyRegex;

pub fn preprocess(line: &str, preprocess_regex: &Vec<FancyRegex>) -> String {
    let mut result = line.to_string();
    for current_rex in preprocess_regex {
        // replace_all 返回 Cow<str>，可以直接转换为 String
        let replacement = current_rex.replace_all(result.as_str(), "<*>").to_string();
        result = replacement;
    }
    result
}

pub fn token_spilter(log_line:&str, regex: &Regex, preprocess_regex: &Vec<FancyRegex>)->(Option<Vec<String>>, Option<String>){
    // 去掉首尾空白并尝试匹配
    let log_line_trimmed = log_line.trim();
    let captures = regex.captures(log_line_trimmed);
    // *test
    println!("match : {:?}", captures);
    match captures {
        None => {
            // 如果没有找到匹配项
            (None, None)
        }
        Some(caps) => {
            // 提取名为 "Content" 的捕获组
            if let Some(message) = caps.name("Content").map(|m| m.as_str()) {
                //*test */
                println!("message = {:?}", message);
                // 预处理消息
                let processed_message = preprocess(message, preprocess_regex);
                //*test */
                println!("process_message = {:?}", processed_message);
                
                // 按空格分割消息为标记
                let tokens: Vec<String> = processed_message
                    .trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();

                (Some(tokens), Some(message.to_string()))
            } else {
                // 如果没有 "Content" 捕获组
                (None, None)
            }
        }
    }
}

pub fn regex_generator(log_format: &str)->Regex{
    let mut regex = String::new();
    let mut captures = Vec::new();
    let mut spliters = Vec::new();
    let re = Regex::new(r"(<[^<>]+>)").unwrap();
    for cap in re.captures_iter(log_format){
        let c = cap.get(0).unwrap();
        let mut s = c.as_str().to_string();
        s = s.trim_start_matches('<').trim_end_matches('>').to_string();
        s = format!(r"(?P<{}>.*?)", s);
        captures.push(s);
    }
    let re_splitter = Regex::new(" +").unwrap();
    for s in re.split(log_format){
        let new_s = re_splitter.replace_all(s, r"\s+");
        spliters.push(new_s.to_string());
    }
    for (idx, value) in captures.iter().enumerate(){
        regex += &spliters[idx];
        regex += value;
    }
    let regex = Regex::new(&format!("^{}$",&regex)).unwrap();
    regex
}