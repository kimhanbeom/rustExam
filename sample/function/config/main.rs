/*
    config crate 를 이용한 파일에서 설정 읽기
    serde crate를 이용한 설정파일을 custom type으로  deserialize

    1.참조 사이트
        1)psutil crate : https://crates.io/crates/config
        2)serde crate : https://crates.io/crates/serde

    2. Cargo.toml파일 버전 설정 :
        [dependencies]
        anyhow="1.0"
        serde = { version = "1", features = ["derive"] }
        config = { version = "0.13", features = ["toml"], default-features = false }

    3. 동작설명
      1)config crate를 이용하여 설정파일(.toml확장자)의 값을 읽어옴 (설정파일의 위치에 맞게 path값은 다시 설정 필요)
      2)읽어온 값을 serde crate를 이용하여 custom type인 Configure로 매칭하여 설정값 저장
      3)저장한 값을 출력
*/

use anyhow::{anyhow, Result};
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Configure {
    /// test value type String
    pub test_string: String,
    /// test value type u32
    pub test_u32: u32,
    /// test valu type f32
    pub test_f32: f32,
}

fn load_config() -> Result<Configure> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()?;
    match settings.try_deserialize::<Configure>() {
        Ok(ret) => Ok(ret),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

fn main() {
    let r_config = match load_config() {
        Ok(ret) => ret,
        _ => {
            panic!("Fail to Load Toml Config file");
        }
    };
    println!("{:?}", r_config);
}
