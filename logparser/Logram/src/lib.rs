mod logram;
mod dictionary;
mod common;
mod match_token;

use logram::LogParser;

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn parse(
    indir_ptr: *const c_char,
    outdir_ptr: *const c_char,
    log_name_ptr: *const c_char,
    log_format_ptr: *const c_char,
    regex_patterns_ptr: *const *const c_char,
    regex_patterns_len: usize,
    double_threshold: i32,
    tri_threshold: i32,
) -> i32 {
    unsafe {
        // 检查输入指针是否为空
        if indir_ptr.is_null() || outdir_ptr.is_null() || log_name_ptr.is_null() || log_format_ptr.is_null() || regex_patterns_ptr.is_null() {
            return -1; // 返回错误码表示输入无效
        }

        // 将C字符串转换为Rust字符串
        let indir = CStr::from_ptr(indir_ptr).to_string_lossy().into_owned();
        let outdir = CStr::from_ptr(outdir_ptr).to_string_lossy().into_owned();
        let log_name = CStr::from_ptr(log_name_ptr).to_string_lossy().into_owned();
        let log_format = CStr::from_ptr(log_format_ptr).to_string_lossy().into_owned();

        // 转换正则表达式模式
        let regex_patterns = std::slice::from_raw_parts(regex_patterns_ptr, regex_patterns_len)
            .iter()
            .map(|&p| CStr::from_ptr(p).to_string_lossy().into_owned())
            .collect::<Vec<String>>();

        // 创建并配置LogParser
        let log_parser = LogParser::new(
            Some(indir.to_string()),
            Some(outdir.to_string()),
            Some(double_threshold),
            Some(tri_threshold),
            log_format.to_string(),
            regex_patterns,
        );

        // 执行解析
        if let Err(e) = log_parser.parse(log_name) {
            eprintln!("fail to parse, error is {}",e);
            return -3; // 返回错误码表示解析失败
        }

        0 // 成功
    }
}
