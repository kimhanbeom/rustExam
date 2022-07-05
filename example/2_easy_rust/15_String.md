1.String
------------------
- rust는 여러개지 String 타입들 있음
- String 과 &str을 그중에서 많이 사용
  1) &str :
      - 단순 문자
      - 매우 빠름
      - ex) let val = "hello"
      - str : dynamic type(크기가 미정) , &str : sized type(포인터의 크기는 고정되있으므)
  2) String :
      - 복합적인 문자,
      - 약간 느리지만 여러 기능을 제공 ,
      - String은 heap을 가르키는 포인터이다.
      - owned type(자기 데이터를 가지고 있는 타입) + sized type(포인터의 크기는 고정되있으므로)
   ```rust
     let my_name = "david" //&str
     let my_name2 = String::from("david") //String
   ```
- String 생성 방법
  1) String::from()
  2) .to_String()
  3) format!
  4) .into()
   ```rust
   주의:변경 결과를 저장하는곳에서 타입을 명시해야함
   let val2 :String = "hello".into();
   ```

- String의 주요기능
   ```rust
     1)push
     let mut my_name = "david".to_string();
     my_name.push('!'); //문자 추가

     2)push_str
     my_name.push_str("!!!!!!"); //문장 추가

     3)capacity
     my_name.capacity();  //usize의 실제 문자 크기를 리턴

     4)with_capacity
     let mut val = String::with_capacity(26) //26의 빈 String 공간 할당
  ```
