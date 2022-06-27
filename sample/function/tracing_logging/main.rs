/*
    tracing crate 를 이용하여 로깅 처리

    1.참조 사이트
        1)tracing : https://crates.io/crates/tracing
        2)tracing-appender : https://crates.io/crates/tracing-appender
        3)tracing-subscriber : https://crates.io/crates/tracing-subscriber

    2. Cargo.toml파일 버전 설정 :
        [dependencies]
        tracing = "0.1.34"
        tracing-subscriber ={version = "0.3.11" , features = ["fmt","local-time","time"]}
        tracing-appender = "0.2.2"

    3. 동작설명
    1) local timezone의 세팅값인 timer 정의
    2) tracing crate를 통한 로깅 설정(로깅폴더/로깅파일 설정 ,로깅파일주기(never),로깅 필터레베리 등등) init
    3) info,warn,error 로깅 출력 후 파일 확인

*/
use tracing::{error, info, warn, Level};
use tracing_subscriber::fmt::time::OffsetTime;

fn main() {
    let timer = OffsetTime::local_rfc_3339().expect("could not get local time");
    let (non_blocking, _guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::never("log", "log"));
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::INFO)
        .with_timer(timer)
        .init();

    info!("log info");
    warn!("log warn");
    error!("log error");
}
