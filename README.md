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

### 安装

下载 ```beerctl``` unix可执行程序，并复制到 ```/usl/local/bin```目录

### 使用

```shell
(base) ➜  ~ beerctl --help
 
beerctl 1.0
guzb
beer 命令行工具

USAGE:
    beerctl [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m <merge-service-list>...        合并镜像列表提供给Jenkins使用

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    does testing things
```

```shell
(base) ➜  ~ beerctl \
-m【交易】fas-biz-trade-paas:1.0.0-345fc84-20230510160210 \
-m【商城】fas-biz-mall-paas:1.0.0-4978278-20230510171837 \
-m【前端-运营】fas-web-platform:1.0.0-24627ce-20230505180901 \
-m【前端-交易】fas-web-trade:1.0.0-779f89a-20230510160229 \
-m【前端-开评】fas-web-kaiping:1.0.0-0897b3a-20230510154552 \
-m【前端-交易】fas-web-trade:1.0.0-c231840-20230510162135 \
-m【sse服务】fas-biz-sse-paas:1.0.0-356627c-20230510144858 \
-m【数据上报】fas-data-report:1.0.0-62fcd0c-20230508102316 \
-m【品库】fas-biz-category-pass:1.0.0-98fa49d-20230509170421 \
-m【前端-交易】fas-web-trade:1.0.0-c4cd55a-20230510185459 \
-m【交易】fas-biz-trade-paas:1.0.0-8ed993d-20230510185917 \
-m【前端-开评】fas-web-kaiping:1.0.0-b8c0427-20230510201107
 
本次需要处理服务12个，正在合并服务列表... 

合并完毕... 相同的服务只保留时间戳最新的 

fas-biz-category-pass:1.0.0-98fa49d-20230509170421
fas-biz-trade-paas:1.0.0-8ed993d-20230510185917
fas-biz-mall-paas:1.0.0-4978278-20230510171837
fas-web-platform:1.0.0-24627ce-20230505180901
fas-biz-sse-paas:1.0.0-356627c-20230510144858
fas-web-trade:1.0.0-c4cd55a-20230510185459
fas-web-kaiping:1.0.0-b8c0427-20230510201107
fas-data-report:1.0.0-62fcd0c-20230508102316
(base) ➜  ~ 
```