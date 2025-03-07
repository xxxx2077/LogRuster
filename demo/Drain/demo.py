#!/usr/bin/env python

import sys
# sys.path.append('../../')
import ctypes
from ctypes import c_char_p, c_double, c_int, POINTER
# from logparser.Drain import LogParser


# 根据平台选择合适的库文件
lib = ctypes.CDLL('../../logparser/Drain/target/release/libDrain.dylib')  # Linux/macOS
# lib = ctypes.CDLL('./target/release/your_project_name.dll')  # Windows

# 定义parse_log函数签名
lib.parse.argtypes = [
    c_char_p,  # indir
    c_char_p,  # outdir
    c_char_p,  # log_name
    c_char_p,  # log_format
    POINTER(c_char_p),  # regex_patterns
    ctypes.c_size_t,  # regex_patterns_len
    ctypes.c_double,  # st
    ctypes.c_size_t,  # depth
]
lib.parse.restype = ctypes.c_int

input_dir  = '../../data/loghub_2k/Hadoop/' # The input directory of log file
output_dir = 'demo_result/'  # The output directory of parsing results
log_file   = 'Hadoop_2k.log'  # The input log file name
log_format = '<Date> <Time> <Level> \[<Process>\] <Component>: <Content>'  # HDFS log format
# Regular expression list for optional preprocessing (default: [])
regex = [r"(\d+\.){3}\d+"]
st         = 0.5  # Similarity threshold
depth      = 4  # Depth of all leaf nodes

# 将正则表达式模式转换为字节字符串
regex_c = [s.encode('utf-8') for s in regex]
# 将regex_patterns转换为c_char_p数组
regex_c_array = (ctypes.c_char_p * len(regex_c))(*regex_c)

# 调用函数
result = lib.parse(
    input_dir.encode('utf-8'),
    output_dir.encode('utf-8'),
    log_file.encode('utf-8'),
    log_format.encode('utf-8'),
    regex_c_array,
    len(regex_c),
    st,
    depth,
)

if result == 0:
    print("Log parsing succeeded.")
else:
    print(f"Log parsing failed with error code {result}.")
