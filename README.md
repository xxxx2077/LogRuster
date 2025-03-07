# logparser_rust

毕业设计：rust高性能日志框架（Drain & Logram）

## Quick Start

### Environment

```bash
pip install -r requirement.txt
```

### Algorithm1 : Drain

#### demo

```bash
python demo/Drain/demo.py
```

#### benchmark

```bash
python benchmark/Drain/benchmark.py
```

### Algorithm2 : Logram

#### demo

```bash
python demo/Logram/demo.py
```

#### benchmark

```bash
python benchmark/Logram/benchmark.py
```


## Application

如何使用本框架

使用以下命令，编译rust源代码

```bash
cd logparser/Drain
cargo build --release
```

编译后代码在`logparser/Drain/target/release`目录

## Reference 

[logparser](https://github.com/logpai/logparser)

数据集、论文来自以上链接
