extern crate clap;

use clap::{App, load_yaml};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use tokio;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[tokio::main]
async fn main() {
    println!(" ");

    let yml = load_yaml!("yaml.yml");
    let matches = App::from_yaml(yml).get_matches();

    if let Some(subcommand) = matches.subcommand_matches("merge") {
        handle_merge_subcommand(subcommand);
    }

    if let Some(subcommand) = matches.subcommand_matches("deploy") {
        handle_deploy_subcommand(subcommand).await;
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        handle_test_subcommand(matches);
    }
}

fn handle_merge_subcommand(subcommand: &clap::ArgMatches) {

    // 检查 file 和 version 参数是否至少传递一个
    if !subcommand.is_present("file-path") && !subcommand.is_present("image-version-list") {
        println!("-v、-f，至少传递一个参数用于获取镜像列表");
        // 在此可以执行相应的错误处理逻辑
        return;
    }

    if let Some(values) = subcommand.values_of("image-version-list") {
        println!("本次需要处理服务{}个，正在合并服务列表... \n", values.len());
        merge_service_list(values.map(|s| s.to_string()).collect());
    }

    if let Some(file_path) = subcommand.value_of("file-path") {
        // 打开文件并读取内容
        if let Ok(lines) = read_lines(file_path) {
            // 将文件内容转换为字符串列表
            let content: Vec<String> = lines
                .filter_map(|line| line.ok())
                .collect();

            println!("本次需要处理服务{}个，正在合并服务列表... \n", content.len());
            merge_service_list(content);
        }
    }
}

// 读取文件并返回一个行迭代器
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn handle_deploy_subcommand(subcommand: &clap::ArgMatches<'_>) {

    // 检查 file 和 version 参数是否至少传递一个
    if !subcommand.is_present("file-path") && !subcommand.is_present("image-version-list") {
        println!("-v、-f，至少传递一个参数用于获取镜像列表");
        // 在此可以执行相应的错误处理逻辑
        return;
    }

    if let Some(values) = subcommand.values_of("image-version-list") {
        println!("本次需要处理服务{}个，正在合并服务列表... \n", values.len());
        let merged_list = merge_service_list(values.map(|s| s.to_string()).collect());
        let ser_list_param = merged_list.join("\n");

        let namespace = subcommand.value_of("namespace").unwrap().to_string();
        let request_url = build_request_url(&namespace, &ser_list_param);
        println!("触发Jenkins更新...");
        make_http_request(&request_url).await.unwrap();
    }

    if let Some(file_path) = subcommand.value_of("file-path") {
        // 打开文件并读取内容
        if let Ok(lines) = read_lines(file_path) {
            // 将文件内容转换为字符串列表
            let content: Vec<String> = lines
                .filter_map(|line| line.ok())
                .collect();

            println!("本次需要处理服务{}个，正在合并服务列表... \n", content.len());
            let merged_list = merge_service_list(content);
            let ser_list_param = merged_list.join("\n");

            let namespace = subcommand.value_of("namespace").unwrap().to_string();
            let request_url = build_request_url(&namespace, &ser_list_param);
            println!("触发Jenkins更新...");
            make_http_request(&request_url).await.unwrap();
        }
    }
}

fn handle_test_subcommand(matches: &clap::ArgMatches) {
    if matches.is_present("list") {
        println!("Printing testing lists...");
    } else {
        println!("Not printing testing lists...");
    }
}

fn merge_service_list(list: Vec<String>) -> Vec<String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let pattern = Regex::new(r"\[[^\]]*\]").unwrap();

    for item in list {
        let s = remove_brackets_and_content(&pattern, &item.replace("【", "[").replace("】", "]"));
        let split: Vec<&str> = s.split(":").collect();

        if split.len() >= 2 {
            map.entry(split[0].to_string()).and_modify(|existing_value| {
                if split[1].to_string() > *existing_value {
                    *existing_value = split[1].to_string();
                }
            }).or_insert(split[1].to_string());
        }
    }

    let merged_list: Vec<String> = map
        .iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect();

    println!("合并完毕... 相同的服务只保留时间戳最新的 \n");
    print_merged_list(&merged_list);
    merged_list
}

fn print_merged_list(merged_list: &[String]) {
    for item in merged_list {
        println!("{}", item);
    }
    println!(" ");
}

fn remove_brackets_and_content(pattern: &Regex, input: &str) -> String {
    pattern.replace_all(input, "").into_owned()
}

fn build_request_url(namespace: &str, ser_list_param: &str) -> String {
    let url = match namespace {
        "fas-test" => "192.168.4.79:8083/view/Fas发布/job/fas-test-build/",
        _ => panic!("暂不支持命名空间:{}", namespace),
    };

    // url是match之后的结果
    let template = format!("http://admin:11529c11a09a8df06bf943397a8ae5ea25@{}buildWithParameters?token=beerctl&ser_list={}", url, ser_list_param);
    template
}

async fn make_http_request(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .build()
        .unwrap();

    let res = client
        .get(url)
        .send()
        .await?;

    let status_code = res.status();
    let headers = res.headers();

    // 打印HTTP状态码
    println!("HTTP Status Code: {}", status_code);

    // 打印特定的响应头，例如"Content-Type"
    if let Some(location) = headers.get("Location") {
        println!("查看Jenkins任务执行详情: {}", "http://192.168.4.79:8083/view/Fas%E5%8F%91%E5%B8%83/job/fas-test-build/");
    }
    let res = res.text().await?;
    Ok(res)
}
