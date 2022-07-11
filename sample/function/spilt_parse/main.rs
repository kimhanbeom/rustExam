/*
    Strig의 split 및 형변환(parse)

    1.참조 사이트
        1)구글

    2. Cargo.toml파일 버전 설정 :
        [dependencies]
        anyhow="1.0"

    3. 동작 설명
      1)string으로 들어온 특정 패턴을 split
      2)parse를 통한 형변환

*/

use anyhow::{anyhow, Result};

fn main() {
    let sample_str = "123,456,789,123";

    match process_split_parse(sample_str) {
        Ok(ret) => println!("result : {:?}", ret),
        Err(e) => panic!("Failed to split,parse : {:?}", e),
    };
}
fn process_split_parse(sample_str: &str) -> Result<(u16, u16, u16, u16)> {
    //split 결과를 collect으로 vec형태로 모음
    let split_str = sample_str.split(',').collect::<Vec<_>>();

    //vec의 요소마다 접근하여 값 확인 후 parse로 u16형 변환
    if let Some(str_a) = split_str.get(0) {
        if let Some(str_b) = split_str.get(1) {
            if let Some(str_c) = split_str.get(2) {
                if let Some(str_d) = split_str.get(3) {
                    return Ok((
                        str_a.parse::<u16>()?,
                        str_b.parse::<u16>()?,
                        str_c.parse::<u16>()?,
                        str_d.parse::<u16>()?,
                    ));
                }
            }
        }
    }
    Err(anyhow!("Failed to Split str value"))
}
