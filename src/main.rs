extern crate clap;

use std::collections::HashMap;
use clap::{App, load_yaml};
use regex::Regex;

fn main() {
    println!(" ");

    let yml = load_yaml!("yaml.yml");
    let matches = App::from_yaml(yml).get_matches();

    // 合并服务列表逻辑
    if let Some(values) = matches.values_of("merge-service-list") {
        println!("本次需要处理服务{}个，正在合并服务列表... \n", matches.occurrences_of("merge-service-list"));

        merge_service_list(values.collect::<Vec<_>>());
    } else {
        println!("merge-service-list not found");
    }

    // 测试命令逻辑
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("list") {
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }
}

fn merge_service_list(list: Vec<&str>) {
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

    println!("合并完毕... 相同的服务只保留时间戳最新的 \n");
    for (k, v) in &map {
        println!("{}:{}", k, v);
    }
}

fn remove_brackets_and_content(pattern: &Regex, input: &str) -> String {
    pattern.replace_all(input, "").into_owned()
}