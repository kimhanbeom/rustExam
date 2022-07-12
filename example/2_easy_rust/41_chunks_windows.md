1.chunks / windows
----------------
 - Chunks / windows : 그룹화 하여 출력 시 사용 하는 함수들 ,두 함수의 차이는 아래의 예제를 보면 됨.
 ```rust
    let numvect = vec![1,2,3,4,5,6,7]
    For chunk in numvect.chunks(3)   //결과 : [1,2,3] [4,5,6] [7]
    For chunk in numvect.windows(3)  //결과 : [1,2,3] [2,3,4] ,,, [5,6,7]
 ```
 chunks의 결과는 정확한 3등분의 그룹이 됩니다. 그룹의 갯수가 딱 떨어지지 않으면 마지막은 나머지만 포함합니다.

 windows의 결과는 모든 idx를 기준으로 3개씩이 가능한 그룹을 전부 생성합니다.
