## Datasets

- [Datasets](#datasets)
  - [Loghub\_2k](#loghub_2k)
  - [Loghub\_2k\_corrected](#loghub_2k_corrected)
  - [Loghub](#loghub)
  - [LogPub](#logpub)

### Loghub_2k

Loghub_2k数据集是从[loghub logs](https://github.com/logpai/loghub)日志中采样得到的，每条日志包含2000行日志消息。消息模板基于正则表达式提取，并经过人工验证和标注。Loghub_2k数据集最初用于在2019年ICSE会议上发表的论文《[Tools and Benchmarks for Automated Log Parsing](https://arxiv.org/pdf/1811.03509.pdf)"》中，作为日志解析器的基准测试数据集。

### Loghub_2k_corrected

Loghub_2k_corrected数据集由2022年ICSE会议上发表的论文《[Guidelines for Assessing the Accuracy of Log Message Template Identification Techniques](https://dl.acm.org/doi/abs/10.1145/3510003.3510101)》开发，该数据集进一步优化和修正了原始Loghub_2k数据集中一些不正确的真实事件模板。

### Loghub

Loghub提供了大量的系统日志数据集，这些数据集可免费用于AI驱动的日志分析研究。原始日志可以在https://github.com/logpai/loghub 上访问。

### LogPub

Loghub提供了大规模的原始日志，但缺乏相应规模的标注事件模板。为了在更严格和实际的场景中评估日志解析器，LogPub为Loghub中的原始日志提供了大规模的人工标注。LogPub数据集可以在https://github.com/logpai/LogPub 上访问。


