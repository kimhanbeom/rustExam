/*
    기존 std를 이용한 파일 처리

    1. Cargo.toml파일 버전 설정 :
        [dependencies]
        anyhow="1.0"

    2.동작 설명
      1).lines/buffed를 이용한 file read
      2).결과를 통해 write file 수행
*/

use anyhow::{anyhow, Result};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Read, Write};

fn lines_file_read(path: &str) -> Result<Vec<String>> {
    let mut file_vec = vec![];
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let lines = contents.lines();
            for line in lines {
                file_vec.push(line.to_string());
            }
            Ok(file_vec)
        } else {
            return Err(anyhow!("Fail to read to string"));
        }
    } else {
        return Err(anyhow!("Fail to File Open"));
    }
}
fn buffed_file_read(path: &str) -> Result<Vec<String>> {
    let mut file_vec = vec![];
    if let Ok(file) = File::open(path) {
        let buffered = BufReader::new(file);
        buffered
            .lines()
            .filter_map(std::result::Result::ok)
            .for_each(|x| file_vec.push(x));
        Ok(file_vec)
    } else {
        return Err(anyhow!("Fail to File Open"));
    }
}
fn file_write(path: &str, items: Vec<String>) -> Result<()> {
    if let Ok(mut writer) = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
    {
        for item in items {
            if writeln!(writer, "{}", item).is_err() {
                return Err(anyhow!("Faild to file write"));
            }
        }
        Ok(())
    } else {
        Err(anyhow!("Faild to file Open(for write"))
    }
}
fn main() {
    let read_file_path = "somethig_read_file.txt";
    let write_file_path = "somthing_write_file.txt";

    if let Ok(buffered_result) = buffed_file_read(read_file_path) {
        println!("{:?}", buffered_result);
        if file_write(write_file_path, buffered_result).is_ok() {
            println!("buffed read /write done");
        };
    }

    if let Ok(lines_result) = lines_file_read(read_file_path) {
        println!("{:?}", lines_result);
        if file_write(write_file_path, lines_result).is_ok() {
            println!("lines read /write done");
        }
    }
}
