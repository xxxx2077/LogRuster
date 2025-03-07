#!/usr/bin/env python

import sys
import ctypes
from ctypes import c_char_p, c_double, c_int, POINTER

input_dir  = '../../data/loghub_2k/HDFS/' # The input directory of log file
output_dir = 'demo_result/'  # The output directory of parsing results
log_file   = 'HDFS_2k.log'  # The input log file name
log_format = '<Date> <Time> <Pid> <Level> <Component>: <Content>'  # HDFS log format
# Regular expression list for optional preprocessing (default: [])
regex      = [
    r'blk_(|-)[0-9]+' , # block id
    r'(/|)([0-9]+\.){3}[0-9]+(:[0-9]+|)(:|)', # IP
    r'(?<=[^A-Za-z0-9])(\-?\+?\d+)(?=[^A-Za-z0-9])|[0-9]+$', # Numbers
]
doubleThreshold = 15
triThreshold = 10

# 根据平台选择合适的库文件
lib = ctypes.CDLL('../../logparser/Logram/target/release/libLogram.dylib')  # Linux/macOS
# lib = ctypes.CDLL('./target/release/your_project_name.dll')  # Windows

# 定义parse_log函数签名
lib.parse.argtypes = [
    c_char_p,  # indir
    c_char_p,  # outdir
    c_char_p,  # log_name
    c_char_p,  # log_format
    POINTER(c_char_p),  # regex_patterns
    ctypes.c_size_t,  # regex_patterns_len
    ctypes.c_int,  # doubleThreshold
    ctypes.c_int,  # triThreshold
]
lib.parse.restype = ctypes.c_int

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
    doubleThreshold,
    triThreshold,
)

if result == 0:
    print("Log parsing succeeded.")
else:
    print(f"Log parsing failed with error code {result}.")
