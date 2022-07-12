1.Option 관련 functions
------------------
 - and() /or() : option 값들의 조건을 체크하는 함수
 ```rust
    let x= Some(2) let y = None   x.and(y)     //결과 : None
    let x= Some(4) let y = Some(5)   x.and(y)  //결과 : Some(5)

    let a= Some(2) let b = None      a.or(b)   //결과 : Some(2)
    let a= Some(6) let b = Some(7)   a.or(b)   //결과 : Some(6)
 ```
 and의 결과가 유효한 값이 나올 경우 y에 해당되는 값 ,즉 2번쨰 값이 결과로 출력됩니다.

 or의 결과가 유효한 값이 나올 경우 a에 해당되는 값 ,즉 1번째 값이 결과로 출력됩니다.
