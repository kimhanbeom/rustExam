1.task / skip / fold
---------------------
 - take : 특정 갯수만 취할 떄 사용하는 함수
 ```rust
   (‘a..’).take(10).collecti::<Vec<_>>();
 ```
 a부터 시작하는 무한의 문자중 10개만 취해서 vec의 결과로 리턴합니다.

 - skip :특정 횟수만큼을 스킵하는 함수
 ```rust
   (‘a..’).skip(1000).take(10).collecti::<Vec<_>>();
 ```
 a부터 시작하는 무한의 문자중에 1000개를 스킵하고 그 후 10개만 취해서 vec의 결과로 리턴합니다.

 - fold : 초기값을 주고 이전 결과를 계속 이어서 순차적인 연산 동작시 사용하는 함수
 ```rust
    let some_number = vec![9,6,9,10,11];
    println!("{}",some_number.iter().fold(0, |total,next| tatal-next));
 ```
 0을 초기값으로 시작하여  0+9 ,9+6 ,15+9 , 24+10, 34+11 순으로 누적이 진행되어 45가 출력
