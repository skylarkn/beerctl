# Beer命令行工具快速入门

[TOC]

## 安装

下载 ```beerctl``` unix可执行程序，并复制到 ```/usl/local/bin```目录

## 使用

### 帮助信息

```shell
(base) ➜  ~ beerctl --help
 
beerctl 1.0
guzb
beer 命令行工具

USAGE:
    beerctl [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    deploy    合并镜像列表，并且触发Jenkins的更新
    help      Prints this message or the help of the given subcommand(s)
    merge     合并镜像列表提供给Jenkins使用

交易组
```

> 可以通过 beerctl SUBCOMMANDS --help查看用法

#### 合并

```shell
(base) ➜  ~ beerctl merge -f /Users/skylarko/Desktop/version.txt
 
本次需要处理服务2个，正在合并服务列表... 

合并完毕... 相同的服务只保留时间戳最新的 

fas-web-trade:1.0.0-196d008-20230512114228
```

### 部署

```shell
(base) ➜  beerctl deploy -f /Users/skylarko/Desktop/version.txt -n fas-test
 
本次需要处理服务2个，正在合并服务列表... 

合并完毕... 相同的服务只保留时间戳最新的 

fas-web-trade:1.0.0-196d008-20230512114228
 
触发Jenkins更新...
本次请求响应: ""
```


