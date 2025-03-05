use std::ffi::CStr;
use std::os::raw::c_char;

// 假设LogParser和相关模块已经正确导入
mod parser;
use parser::LogParser;

// #[no_mangle]
// pub extern "C" fn print_string(c_string_ptr: *const c_char) {
//     let bytes = unsafe { CStr::from_ptr(c_string_ptr).to_bytes() };
//     let str_slice = std::str::from_utf8(bytes).unwrap();
//     println!("{}", str_slice);
// }

#[no_mangle]
pub extern "C" fn parse(
    indir_ptr: *const c_char,
    outdir_ptr: *const c_char,
    log_name_ptr: *const c_char,
    log_format_ptr: *const c_char,
    regex_patterns_ptr: *const *const c_char,
    regex_patterns_len: usize,
    st: f64,
    depth: usize,
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
        let mut log_parser = LogParser::new(
            Some(indir),
            Some(outdir),
            Some(depth),
            Some(st),
            None,
            log_format,
            regex_patterns,
            None,
        );

        // 执行解析
        if let Err(e) = log_parser.parse(log_name) {
            eprintln!("fail to parse, error is {}",e);
            return -3; // 返回错误码表示解析失败
        }

        0 // 成功
    }
}