1.closuers
------------
 - 클로저 : 변수에 저장하거나 다른 함수(map()과 같은)에 인자로 넘길 수 있는 익명 함수
  ```rust
  //num : 파라미터 (파이프[||] 사이에 들어간 것)
  //result : 반환값
  let value = | num:u32 | -> u32 {
	……
         ……
         result
  };

  //파라미터 ,반환 없는 형태도 가능
  let value = |_| println!("I am function!!!");
  ```

 - 컴파일러는 클로저를 호출하는 곳을 통해 타입의 추론을 해줌
 ```rust
   let exam_c= |x| x;
 ```
  예를 들어  exam_c(3)으로 하면 x는 i32로 추론되어 정상동작함

  하지만 exam_c(3) exam_c(“hi”)를 연속으로 써주면 에러가 발생 (exam_c(3)에서 컴파일러는 x를 i32로 추론했는데 그다음줄에 “hi”라는 &str타입이 들어왔기 때문)

 - 클로저를 구조체에 넣기
 ```rust
    struct Cacher<T>
    where T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>,
    }
 ```
  Fn 트레잇은 클로저를 위한 것입니다.

  struct Cacher의 제네릭 T는 클로저를 위한 Fn트레잇을 구현한 타입만 사용가능하다는 뜻

 - 클로저는 환경에서의 값을 캡쳐하여 사용 할 수 있음
 ```rust
    fn main() {
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));
    }
 ```
 위의 예제에서 클로저에서 외부변수 x를 사용하고 하여 내용을 정의하였지만 사용시 문제가 없음

 - 클로저 구현시 사용하는 트레잇에 따라 값을 캡쳐하는 방식이 정해짐
   - FnOnce : 소유권 이동
   - Fn :불변으로 빌려옴
   - FnMut : 가변으로 빌려옴

 - 만일 캡쳐 값의 소유권을 가져오고 싶다면 move 키워드를 사용한다.
 ```rust
     fn main() {
        let x = 4;
        let equal_to_x = move |z| z==x;
    }
 ```

 - 기본적으로 xxx_else() 같은 함수들은 클로저와 동반하여 사용한다.
 ```rust
    ok_or_else(|a| .....) . unwrap_or_else(|a| .....)
 ```

 - 그외에 클로저를 동반하는 몇몇 함수
   - any : 특정 그룹 안에서 값의 유무를 체크, 하나라도 맞으면 true
   - all : 특정 그룹 안에서 값이 유무를 체크, 전부 맞아야 true
   ```rust
    char_vec.iter().any(|character| character == check)
    char_vec.iter().all(|character| character == check)
   ```
