# =========================================================================
# Copyright (C) 2016-2023 LOGPAI (https://github.com/logpai).
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# =========================================================================


import sys
sys.path.append("../")
# from logparser.Drain import LogParser
from utils import evaluator
import os
import pandas as pd
import ctypes
from ctypes import c_char_p, c_double, c_int, POINTER

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
    ctypes.c_int,  # double_st
    ctypes.c_int,  # trible_st
]
lib.parse.restype = ctypes.c_int

input_dir = "../../data/loghub_2k/"  # The input directory of log file
output_dir = "Logram_result/"  # The output directory of parsing results

benchmark_settings = {
    "HDFS": {
        "log_file": "HDFS/HDFS_2k.log",
        "log_format": "<Date> <Time> <Pid> <Level> <Component>: <Content>",
        "regex": [
            r"blk_(|-)[0-9]+",  # block id
            r"(/|)([0-9]+\.){3}[0-9]+(:[0-9]+|)(:|)",  # IP
            r"(?<=[^A-Za-z0-9])(\-?\+?\d+)(?=[^A-Za-z0-9])|[0-9]+$",
        ],
        "doubleThreshold": 15,
        "triThreshold": 10,
    },
    "Hadoop": {
        "log_file": "Hadoop/Hadoop_2k.log",
        "log_format": "<Date> <Time> <Level> \[<Process>\] <Component>: <Content>",
        "regex": [r"(\d+\.){3}\d+"],
        "doubleThreshold": 9,
        "triThreshold": 10,
    },
    "Spark": {
        "log_file": "Spark/Spark_2k.log",
        "log_format": "<Date> <Time> <Level> <Component>: <Content>",
        "regex": [r"(\d+\.){3}\d+", r"\b[KGTM]?B\b", r"([\w-]+\.){2,}[\w-]+"],
        "doubleThreshold": 15,
        "triThreshold": 10,
    },
    "Zookeeper": {
        "log_file": "Zookeeper/Zookeeper_2k.log",
        "log_format": "<Date> <Time> - <Level>  \[<Node>:<Component>@<Id>\] - <Content>",
        "regex": [r"(/|)(\d+\.){3}\d+(:\d+)?"],
        "doubleThreshold": 15,
        "triThreshold": 10,
    },
    "BGL": {
        "log_file": "BGL/BGL_2k.log",
        "log_format": "<Label> <Timestamp> <Date> <Node> <Time> <NodeRepeat> <Type> <Component> <Level> <Content>",
        "regex": [r"core\.\d+"],
        "doubleThreshold": 92,
        "triThreshold": 4,
    },
    "HPC": {
        "log_file": "HPC/HPC_2k.log",
        "log_format": "<LogId> <Node> <Component> <State> <Time> <Flag> <Content>",
        "regex": [r"=\d+"],
        "doubleThreshold": 15,
        "triThreshold": 10,
    },
    "Thunderbird": {
        "log_file": "Thunderbird/Thunderbird_2k.log",
        "log_format": "<Label> <Timestamp> <Date> <User> <Month> <Day> <Time> <Location> <Component>(\[<PID>\])?: <Content>",
        "regex": [r"(\d+\.){3}\d+"],
        "doubleThreshold": 35,
        "triThreshold": 32,
    },
    "Windows": {
        "log_file": "Windows/Windows_2k.log",
        "log_format": "<Date> <Time>, <Level>                  <Component>    <Content>",
        "regex": [r"0x.*?\s"],
        "doubleThreshold": 15,
        "triThreshold": 10,
    },
    "Linux": {
        "log_file": "Linux/Linux_2k.log",
        "log_format": "<Month> <Date> <Time> <Level> <Component>(\[<PID>\])?: <Content>",
        "regex": [r"(\d+\.){3}\d+", r"\d{2}:\d{2}:\d{2}"],
        "doubleThreshold": 120,
        "triThreshold": 100,
    },
    "Android": {
        "log_file": "Android/Android_2k.log",
        "log_format": "<Date> <Time>  <Pid>  <Tid> <Level> <Component>: <Content>",
        "regex": [
            r"(/[\w-]+)+",
            r"([\w-]+\.){2,}[\w-]+",
            r"\b(\-?\+?\d+)\b|\b0[Xx][a-fA-F\d]+\b|\b[a-fA-F\d]{4,}\b",
        ],
        "doubleThreshold": 15,
        "triThreshold": 10,
    },
    "HealthApp": {
        "log_file": "HealthApp/HealthApp_2k.log",
        "log_format": "<Time>\|<Component>\|<Pid>\|<Content>",
        "regex": [],
        "doubleThreshold": 15,
        "triThreshold": 10,
    },
    "Apache": {
        "log_file": "Apache/Apache_2k.log",
        "log_format": "\[<Time>\] \[<Level>\] <Content>",
        "regex": [r"(\d+\.){3}\d+"],
        "doubleThreshold": 15,
        "triThreshold": 10,
    },
    "Proxifier": {
        "log_file": "Proxifier/Proxifier_2k.log",
        "log_format": "\[<Time>\] <Program> - <Content>",
        "regex": [
            r"<\d+\ssec",
            r"([\w-]+\.)+[\w-]+(:\d+)?",
            r"\d{2}:\d{2}(:\d{2})*",
            r"[KGTM]B",
        ],
        "doubleThreshold": 500,
        "triThreshold": 470,
    },
    "OpenSSH": {
        "log_file": "OpenSSH/OpenSSH_2k.log",
        "log_format": "<Date> <Day> <Time> <Component> sshd\[<Pid>\]: <Content>",
        "regex": [r"(\d+\.){3}\d+", r"([\w-]+\.){2,}[\w-]+"],
        "doubleThreshold": 88,
        "triThreshold": 81,
    },
    "OpenStack": {
        "log_file": "OpenStack/OpenStack_2k.log",
        "log_format": "<Logrecord> <Date> <Time> <Pid> <Level> <Component> \[<ADDR>\] <Content>",
        "regex": [r"((\d+\.){3}\d+,?)+", r"/.+?\s", r"\d+"],
        "doubleThreshold": 30,
        "triThreshold": 25,
    },
    "Mac": {
        "log_file": "Mac/Mac_2k.log",
        "log_format": "<Month>  <Date> <Time> <User> <Component>\[<PID>\]( \(<Address>\))?: <Content>",
        "regex": [r"([\w-]+\.){2,}[\w-]+"],
        "doubleThreshold": 2,
        "triThreshold": 2,
    },
}

benchmark_result = []
for dataset, setting in benchmark_settings.items():
    print("\n=== Evaluation on %s ===" % dataset)
    indir = os.path.join(input_dir, os.path.dirname(setting["log_file"]))
    log_file = os.path.basename(setting["log_file"])

        # 将正则表达式模式转换为字节字符串
    regex_c = [s.encode('utf-8') for s in setting["regex"]]
    # 将regex_patterns转换为c_char_p数组
    regex_c_array = (ctypes.c_char_p * len(regex_c))(*regex_c)

    # 确保路径末尾有目录分隔符
    if not indir.endswith(os.sep):
        indir += os.sep

    # 调用函数
    result = lib.parse(
        indir.encode('utf-8'),
        output_dir.encode('utf-8'),
        log_file.encode('utf-8'),
        setting["log_format"].encode('utf-8'),
        regex_c_array,
        len(regex_c),
        setting["doubleThreshold"],
        setting["triThreshold"],
    )

    if result == 0:
        print("Log parsing succeeded.")
    else:
        print(f"Log parsing failed with error code {result}.")

    F1_measure, accuracy = evaluator.evaluate(
        groundtruth=os.path.join(indir, log_file + "_structured.csv"),
        parsedresult=os.path.join(output_dir, log_file + "_structured.csv"),
    )
    benchmark_result.append([dataset, F1_measure, accuracy])

print("\n=== Overall evaluation results ===")
df_result = pd.DataFrame(benchmark_result, columns=["Dataset", "F1_measure", "Accuracy"])
df_result.set_index("Dataset", inplace=True)
print(df_result)
df_result.to_csv("Logram_benchmark_result.csv", float_format="%.6f")
