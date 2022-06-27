1.rustc로 생성 및 실행
--------------------
```rust
mkdir hello_world
cd hello_world
vim main.rs
rustc main.rs
.\main.exe
````

2.cargo 이용 (cargo :러스트의 빌드 시스템 및 패키지 매니저)
-------------------------------------------------
```rust
cargo new hello_cargo --bin  (모듈을 위한 옵션을 추후 뒤에서 작성 예정)
cd hello_cargo
cargo build  ./target/debug/hello_cargo
     or
cargo run
```

3.cargo 추가 옵션
---------------
1)cargo check : 컴파일 되는지 체크

2)cargo fmt : 코드를 공식 스탕리 가이드라인으로 포멧팅 하여 정리/정렬

3)cargo clippy : 코드 린터(linter)

  * linter : 소스코드를 분석하여 오류/버그/스타일 오류/의심스러운 부분을 찾아내고 표시
