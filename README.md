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

### 构建编辑

```shell
(base) ➜  beerctl git:(master) ✗ cargo build --release
   Compiling libc v0.2.144
   Compiling autocfg v1.1.0
   Compiling cfg-if v1.0.0
   Compiling proc-macro2 v1.0.56
   Compiling unicode-ident v1.0.8
   Compiling quote v1.0.27
   Compiling log v0.4.17
   Compiling itoa v1.0.6
   Compiling pin-project-lite v0.2.9
   Compiling parking_lot_core v0.9.7
   Compiling bytes v1.4.0
   Compiling futures-core v0.3.28
   Compiling scopeguard v1.1.0
   Compiling smallvec v1.10.0
   Compiling tinyvec_macros v0.1.1
   Compiling io-lifetimes v1.0.10
   Compiling core-foundation-sys v0.8.4
   Compiling tinyvec v1.6.0
   Compiling once_cell v1.17.1
   Compiling rustix v0.37.19
   Compiling lock_api v0.4.9
   Compiling tokio v1.28.1
   Compiling futures-task v0.3.28
   Compiling unicode-normalization v0.1.22
   Compiling tracing-core v0.1.30
   Compiling indexmap v1.9.3
   Compiling slab v0.4.8
   Compiling futures-util v0.3.28
   Compiling percent-encoding v2.2.0
   Compiling unicode-bidi v0.3.13
   Compiling fnv v1.0.7
   Compiling serde v1.0.163
   Compiling http v0.2.9
   Compiling syn v2.0.15
   Compiling tracing v0.1.37
   Compiling hashbrown v0.12.3
   Compiling time-core v0.1.0
   Compiling num_cpus v1.15.0
   Compiling mio v0.8.6
   Compiling socket2 v0.4.9
   Compiling parking_lot v0.12.1
   Compiling signal-hook-registry v1.4.1
   Compiling errno v0.3.1
   Compiling core-foundation v0.9.3
   Compiling security-framework-sys v2.8.0
   Compiling native-tls v0.2.11
   Compiling futures-sink v0.3.28
   Compiling version_check v0.9.4
   Compiling fastrand v1.9.0
   Compiling futures-channel v0.3.28
   Compiling httparse v1.8.0
   Compiling pin-utils v0.1.0
   Compiling cookie v0.16.2
   Compiling security-framework v2.8.2
   Compiling time-macros v0.2.8
   Compiling idna v0.3.0
   Compiling form_urlencoded v1.1.0
   Compiling ryu v1.0.13
   Compiling try-lock v0.2.4
   Compiling lazy_static v1.4.0
   Compiling serde_json v1.0.96
   Compiling want v0.3.0
   Compiling tempfile v3.5.0
   Compiling http-body v0.4.5
   Compiling matches v0.1.10
   Compiling psl-types v2.0.11
   Compiling httpdate v1.0.2
   Compiling tower-service v0.3.2
   Compiling publicsuffix v2.2.3
   Compiling idna v0.2.3
   Compiling url v2.3.1
   Compiling atty v0.2.14
   Compiling encoding_rs v0.8.32
   Compiling ipnet v2.7.2
   Compiling base64 v0.21.0
   Compiling mime v0.3.17
   Compiling clap v2.34.0
   Compiling time v0.3.20
   Compiling tokio-macros v2.1.0
   Compiling serde_derive v1.0.163
   Compiling tokio-util v0.7.8
   Compiling tokio-native-tls v0.3.1
   Compiling h2 v0.3.18
   Compiling serde_urlencoded v0.7.1
   Compiling cookie_store v0.16.1
   Compiling hyper v0.14.26
   Compiling hyper-tls v0.5.0
   Compiling reqwest v0.11.17
   Compiling beerctl v0.1.0 (/Users/skylarko/IdeaProjects/beerctl)
    Finished release [optimized] target(s) in 50.81s
```

### 使用

#### 帮助信息

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
(base) ➜  ~ beerctl deploy --help
 
beerctl-deploy 
合并镜像列表，并且触发Jenkins的更新

USAGE:
    beerctl deploy --image-version-list <image-version-list>... --jtoken <jtoken> --url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -v, --image-version-list <image-version-list>...
            提供多个镜像版本定义 格式为【开标】fas-biz-open-paas:1.0.0-3f6672c-20230511092336

        --jtoken <jtoken>                               定义Jenkins的Token值，例如11529c11a09a8df06bf943397a8ae5ea25
        --url <url>                                     定义Jenkins的Job地址
(base) ➜  ~ 
```

#### 合并

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

#### 部署

```shell
(base) ➜  ~ beerctl deploy \
-v 【前端-运营】fas-web-platform:1.0.0-8c41f4e-20230511163417 \
-v 【前端-交易】fas-web-trade:1.0.0-ced335f-20230511163625 \
--jtoken beerctl --url 192.168.4.79:8083/view/Fas发布/job/fas-test-build/
 
本次需要处理服务2个，正在合并服务列表... 

合并完毕... 相同的服务只保留时间戳最新的 

fas-web-platform:1.0.0-8c41f4e-20230511163417
fas-web-trade:1.0.0-ced335f-20230511163625
 
触发Jenkins更新...
本次请求响应:""
```

