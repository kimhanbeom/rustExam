1.copy
------------
 - 기본적으로 copy가 일어날떄 공간을 많이 차지하지 않는 타입들이 대부분
   copy trait을 가지고 있다.(이 타입들은 값을 stack에 저장하는 타입들이다.) ex) i32,char
 - 그렇기 떄문에 소유권 이전이 아니라 항상 값의 copy가 발생한다.

  ```rust
  fn print_num(number:i32){
    println!("{}",number);
  }
  fn main(){
    let num = 10;
    print_num(num);
  }
  ```
 - String은 clone을 통해 copy를 할 수 있다.
  ```rust
  fn print_string(country:String){
    println!("{}",country);
  }
  fn main(){
    let country_name = String::from("korea");
    print_string(country_name.clone());
  }
  ```

2.uninitialized variable
------------------------
 - uninitialized variable 선언 방법(자료형을 명시해서 선언)
```rust
  let val :u8;
```
 - 특정 scope범위 밖에서도 유효하고 싶을 때 사용하면 좋음
   (해당 변수는 아직 초기화하지 않았으므로 mut로 하지 않아도 됨)
 ```rust
  fn main() {
    let my_number;
    {
        my_number = 10;
    }
    println!("{}", my_number);
  }
 ```
 - 특정 logic뒤 후 변수에 값을 넣어줘야 하는 경우 다음과 같은 방식이 rust다운 방식
 ```rust
   let un_variable = {
    let x= 5;
    //something logic
    x
   }
 ```
