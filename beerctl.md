## Beer命令行工具

```yaml
name: beerctl
version: "1.0"
about: beer 命令行工具
author: guzb

args:
  - merge-service-list:
      help: 合并镜像列表提供给Jenkins使用
      short: msl
      takes_value: true
      multiple: true
subcommands:
  - test:
      about: does testing things
      args:
        - list:
            help: lists test values
            short: l
```

> 可用命令参考上述文件