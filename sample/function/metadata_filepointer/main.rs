/*
    std를 이용한 파일/폴더의 메타데이터 , 파일의 파일 포인터 처리

    1.참조 사이트
        1)std library

    2. Cargo.toml파일 버전 설정 :
        [dependencies]

    3. 동작설명
      1) 폴더/파일의 open 후 metadata 정보 보기
      2) 파일 포인터 처리에 대한 std lib 기능 사용

*/

use std::io::{Seek, SeekFrom};
use std::{fs, fs::File};

fn main() {
    let file_path = "something_file_path";
    let dir_path = "something_dir_path";

    //파일을 열고 , 해당 파일의 메타데이터 확인
    if let Ok(mut file) = File::open(file_path) {
        if let Ok(meta_data) = file.metadata() {
            println!("{:?}\n", meta_data);
        }
        //파일 포인터 처리 방법
        //1.현재 파일의 시작지점부터 특정 값만큼 파일 포인터 이동
        if let Ok(ret) = file.seek(SeekFrom::Start(10)) {
            println!("seek start :{}", ret);
        }
        //2.현재 파일의 끝지점부터 특정 값만큼 파일 포인터 이동
        if let Ok(ret) = file.seek(SeekFrom::End(10)) {
            println!("seek end:{}", ret);
        }
        //3.현재 파일의 파일포인터 위치부터 특정 값만큼 파일 포인터 이동
        if let Ok(ret) = file.seek(SeekFrom::Current(10)) {
            println!("seek current:{}", ret);
        }
        //4.현재 파일의 파일포인터를 초기화시킴(0으로 이동시킴)
        if let Ok(_) = file.rewind() {
            println!("rewind done");
        }
        //5.현재 파일의 파일포인터 위치를 리턴함
        if let Ok(ret) = file.stream_position() {
            println!("file pointer position :{} ", ret);
        }
    }

    //폴더를 열고 ,해당 폴더의 메타데이터 확인(내부 파일/서브 폴더 정보의 메타데이터 확인)
    if let Ok(dirs) = fs::read_dir(dir_path) {
        for dir in dirs.flatten() {
            if let Ok(meta_data) = dir.metadata() {
                println!("{:?}\n", meta_data);
            }
        }
    }
}
