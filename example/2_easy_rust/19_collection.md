1.collection
-------------
 1) Array
  - 같은 타입만 담을 수 있고 , 사이즈의 변화를 줄 수 없는 타입
  - 버퍼를 만들떄 주로 사용합니다.
  - 값을 호출 하는 방법은 2가지 있습니다.
     1. index 바로 접근(arr[0]) : 결과로 값들을 가져옴,out of index에 취약함
     2. get 이용(arr.get(0)) : 결과로 Option 타입을 가져옴,out of index 처리 유용
    ```rust
      //1.사용법
      let arry = [8,9,10];

      //2.같은 array지만 밑의 2개는 타입이 틀립니다.
      let arry2 = ["one","two"];          //[&str:2] 타입
      let arry3 = ["one","two","three"];  //[&str:3] 타입

      //3.만일 현재 타입을 모르겠으면 다음과 같이 존재하지 않은 함수를 호출는 식으로 작성하면 컴파일러가 타입을 알려줍니다.
      arry2.sdaklfjlaskdfljsad()
      (debug message : method not found in `[&str; 2]`)

      //4.buffer를 만들때 주로 사용합니다.
      let arr = [0;640]  //0을 채운 640의 버퍼공간 생성

      //5.값을 호출하기
      arr[0]
      arr.get(0)
    ```


 2) slice (dynamically sized type)
   - 특정 collection의 부분(slice한)
   ```rust
    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = array_of_ten[2..5];
  ```

 3) Vectors
  - 같은 타입만 담을 수 있고 , 사이즈의 변화를 줄 수 있는 타입
  ```rust
  fn main(){
    //vec에서 String 넣기
    let name1 = String::from("hello");
    let name2 = String::from("world");
    let mut my_vec = Vec::new()
    my_vec.push(name1);
    my_vec.push(name2);

    //push 없는 경우, 생성시 타입명시 필수
    let mut my_vec1:Vec<String> = Vec:new();

    //값을 넣주면서 생성 가능
    let mu my_vec2 = vec![8,9,10];
  }
  ```
