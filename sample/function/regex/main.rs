/*
    regex를 이용한 web log 파싱

    1.참조 사이트
        1)정규식 나무위키: https://namu.wiki/w/%EC%A0%95%EA%B7%9C%20%ED%91%9C%ED%98%84%EC%8B%9D
        2)Regex crate : https://crates.io/crates/regex
        3)Rust cookbook(regex) : https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
        4)web log 정보: https://httpd.apache.org/docs/2.4/ko/logs.html

    2. Cargo.toml파일 버전 설정 :
        [dependencies]
        regex = "1.5"


    3. 동작 설명
      1)web log 정규식 처리를 위한 regex new 선언
      2)web log 문장을 regex의 capture function을 통해 처리
      3)captuer된 결과를 출력

    4. 기타
      1)new로 생성한 정규식 조건이 엄격해질수록 좀더 복잡하고 예외적인 처리가 가능하지만 대량 데이터 처리시 처리 속도가 배수로 느려집니다.
      2)현재 작성한 샘플에서는 정규식 조건을 simple하게 하였습니다.
*/

use regex::Regex;

fn main() {
    let sample_log = "127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] \"GET /apache_pb.gif HTTP/1.0\" 200 2326 \"http://www.example.com/start.html\" \"Mozilla/4.08 [en] (Win98; I ;Nav)\"";
    //30초 완
    let log_rex = Regex::new(
        r#"(?x)
        (?P<ip>[^\s]+).{3}
        (?P<id>[^\s]+).{2}
        (?P<date>[^]]+).{3}
        (?P<method>[^"]+).{2}
        (?P<state>[^\s]+)\s
        (?P<len>[^\s]+).{2}
        (?P<ref>[^"]*).{3}
        (?P<agent>[^"]+)"
        "#,
    )
    .expect("Failed to new Regex");

    if let Some(caps) = log_rex.captures(str_test) {
        println!("IP : {}", &caps["ip"]);
        println!("ID : {}", &caps["id"]);
        println!("Date : {}", &caps["date"]);
        println!("Method : {}", &caps["method"]);
        println!("State : {}", &caps["state"]);
        println!("Len : {}", &caps["len"]);
        println!("Ref : {}", &caps["ref"]);
        println!("Agent : {}", &caps["agent"]);
    } else {
        println!("Failed to capture ");
    }
}
