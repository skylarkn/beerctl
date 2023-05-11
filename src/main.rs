extern crate clap;

use clap::{App, load_yaml};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use tokio;

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
    if let Some(values) = subcommand.values_of("image-version-list") {
        println!("本次需要处理服务{}个，正在合并服务列表... \n", values.len());
        let merged_list = merge_service_list(values.collect::<Vec<_>>());
        print_merged_list(&merged_list);
    }
}

async fn handle_deploy_subcommand(subcommand: &clap::ArgMatches<'_>) {
    if let Some(values) = subcommand.values_of("image-version-list") {
        println!("本次需要处理服务{}个，正在合并服务列表... \n", values.len());
        let merged_list = merge_service_list(values.collect::<Vec<_>>());
        let ser_list_param = merged_list.join("\n");

        let url = subcommand.value_of("url").unwrap().to_string();
        let token = subcommand.value_of("jtoken").unwrap().to_string();
        let request_url = build_request_url(&url, &token, &ser_list_param);
        println!("触发Jenkins更新...");
        let data = make_http_request(&request_url).await.unwrap();
        println!("本次请求响应:{:?}", data);
    }
}

fn handle_test_subcommand(matches: &clap::ArgMatches) {
    if matches.is_present("list") {
        println!("Printing testing lists...");
    } else {
        println!("Not printing testing lists...");
    }
}

fn merge_service_list(list: Vec<&str>) -> Vec<String> {
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

fn build_request_url(base_url: &str, token: &str, ser_list_param: &str) -> String {
    let mut request_url = "http://admin:11529c11a09a8df06bf943397a8ae5ea25@".to_string();
    request_url.push_str(base_url);
    request_url.push_str("buildWithParameters");
    request_url.push_str("?token=");
    request_url.push_str(token);
    request_url.push_str("&ser_list=");
    request_url.push_str(ser_list_param);
    request_url
}

async fn make_http_request(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .build()
        .unwrap();

    let res = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}
