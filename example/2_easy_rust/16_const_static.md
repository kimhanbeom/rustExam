1.const static
------------------
- const:
 1) global scope 인 상수
 2) 표시는 대문자 , 자료형 반드시 명시
 3) 값의 변경은 불가능,이름을 통해 사용시 값으로 변경 매칭

 ```rust
   const NUMBER : i32 = 20;
 ```

- static :
 1) 고정된 메모리를 사용
 2) lifetime에서 'static 으로 사용(프로그램의 끝까지 살 수 있음)

 ```rust
  // 실제로는 &'static str
  let my_name ="david"

  //staic으로 상수 만들기 , 해당 값은 고정된 메모리에 위치
  static NUMBER: i32 = 0;
 ```
