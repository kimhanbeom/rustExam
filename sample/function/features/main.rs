/*
    features를 이용하여 선택적 빌드하기
    (간단하게 만들어봤습니다. 기본적으로 다른 크레이트의 feature도
     이런방식으로 동작하는듯??)

    1.참조 사이트
        1) https://doc.rust-lang.org/cargo/reference/features.html

    2. Cargo.toml에 features 설정 :
        [features]
        features1 =[]
        features2 =[]
        features3 = ["features1","features2"]

    3. 동작설명
      1)cargo run --features features1 으로 실행시
        features1만 출력
      2)cargo run --features features2 으로 실행시
        features2만 출력
      3)cargo run --features features3 으로 실행시
        features1,features2,features3, 전부 출력
      4)cargo run --all-features 으로 실행시
        features1,features2,features3, 전부 출력
*/

fn main() {
    #[cfg(feature = "features1")]
    run_f1();
    #[cfg(feature = "features2")]
    run_f2();
    #[cfg(all(feature = "features1", feature = "features2"))]
    run_f3();
}

#[cfg(feature = "features1")]
fn run_f1() {
    println!("features1");
}

#[cfg(feature = "features2")]
fn run_f2() {
    println!("features2");
}

#[cfg(all(feature = "features1", feature = "features2"))]
fn run_f3() {
    println!("features3");
}
