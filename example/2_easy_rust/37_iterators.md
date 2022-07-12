1.iterators
------------
 - next()를 호출 할 수 있는 것들의 집합(a collection of things that you can call .next() on)
 - 일련의 항목을 순서대로 어떤 작업을 수행 할 수 있도록 해줌
 - 기본적인 성향은 lazy
 ```rust
  let v_iter =v1.iter()
 ```
 이 자체로 아무런 동작을 안하고 유저가 추가 처리해줘야 의미가 있음.
 - 호출 할 수 있는 iter 종류
    1) iter() - iterator of references (&T : 불변 참조 iter())
    2) iter_mut() = iterator of mutable references(&mut T : 가변참조 iter())
    3) into_iter() : consuming iterator (소모적인 iter())
  ```rust
  fn main() {
      let vector1 = vec![1, 2, 3]; // we will use .iter() and .into_iter() on this one
      let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
      let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>(); //into_iter()를 써서 vector1은 전부 소모됨

      let mut vector2 = vec![10, 20, 30]; // we will use .iter_mut() on this one
      vector2.iter_mut().for_each(|x| *x +=100);

      println!("{:?}", vector1_a);
      println!("{:?}", vector2);
      println!("{:?}", vector1_b);
      println!("{:?}", vector1); //에러 발생(이미 소모되어서)
  }
  ```
 - Iterator 트레잇과 next메서드를 구현하였기 떄문에 next메서드를 직접 사용 가능
 ```rust
   let mut v1_iter = v1.iter();
   v1_iter.next()
 ```
 - next,sum 과 같이 반복자를 쓸 경우 소비되어 빈껍데기만 남게되는 방식으로 호출하는 메서드를 소비하는 어댑터라 합니다.
 ```rust
    v1_iter.iter().sum()
 ```
 - 반복자에서 다른 반복자로 변경하고 생성하는 메서드를 반복자 어댑터라고 한다.
 ```rust
     let v1: Vec<i32> = vec![1,2,3];
     v1.iter().map(|x| x+1).collect();
 ```

 - Iterator에 chaining으로 사용하는 여러 함수
    1) next() : 다음 요소를 가져옴
    ```rust
    let mut v1_iter = v1.iter();
    v1_iter.next()
    ```
    2) peek() : 현재 요소를 가져옴 , 여러번 호출 가능
    ```rust
    //전부 같은 요소가 결과로 출력됨
    v1_iter.iter().peekable();
    v1_iter.iter().peekable();
    v1_iter.iter().peekable();
    ```
    3) zip() : 여러가지 요소를 묶음
    ```rust
    let some_number = vec![0,1,2];
    let some_word = vec!["zero","one","two"];
    let number_word_hashmap:HashMap<i32,&str> = some_number
                              .into_iter()
                              .zip(some_word.into_iter())
                              .collection();
    ```
    4) enumerate() : 요소에 index를 포함한 (_,_)타입으로 만듬
    ```rust
    let num_vec = vec![2,4,6];
    v1_iter
        .iter() //2,4,6
        .enumerate() //(0,2) (1,4) (2,6)
        .for_each(|(idx,num)| {   //for문과 같이 모든 요소를 처리해줌
          println!("{idx} {num}");
        });
    ```
    5) map() :반복자 어댑터로 추가적인 연산을 클로저로 수행
    ```rust
    v1_iter.iter().map(|x| x*10).collect::Vec<i32>();
    ```
    6) filter() : filter 안의 클로저 결과로 true가 되는 요소만 반복자에 포함하는 함수
    ```rust
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    ```
    7) filter_map() : filter와 map을 동시에 하는 함수
    ```rust
        //map 처럼 무언가 동작(get_shoes호출)을 하고 결과가 Some,none 에 따라 filter처리
        shoes.into_iter().filter_map(|s| s.get_shoes()).collect()
    ```
    8) rev() : 순서를 거꾸로 함
    ```rust
      big_vec.iter().rev()
    ```
    9) find: 특정 조건을 만족하는 값을 찾아 option으로 리턴.
    ```rust
      vec_var.iter().find( |x| x > 0)
    ```
    10) position : 특정 조건을 만족하는 값의 위치를 찾아 option으로 리턴
    ```rust
      vec_var.iter().postion( |x| x == 0)
    ```
    11) Cycle :끝나지 않는 iterator, 순서대로 값을 넣어줄떄/사이즈가 다른 값과 엮을때 사용
    ```rust
      let even_odd=vec!["even","odd"].into_iter().cycle();
      let even_odd_vec = (0..6).zip(even_odd.into_iter().cycle()).collect::<Vec<(i32,&str)>>();
       // [(0,"even"),(1,"odd"),(2,"even"),(3,"odd"),(4,"even"),5,"odd")]가 결과로 나옴
    ```
    12) sum : 요소의 모든 값을 합함
    ```rust
      let numbers= vec![1,2,3,4,5];
      println!("{}",numbers.iter().sum::<i32>());
    ```

 - custom 타입의 iterator를 만들 수 있음.
    1) impl iterator 후 next 메서드 구현
    ```rust
      impl iterator for Library{
        fn next(&mut self) ->Option<String>{
            //........
        }
      }
    ```
    2) custom 타입은 next를 구현했으므로 zip, map, filter,sum 을 사용 할 수 있게 됨
    ```rust
      library.iter().map(....)
    ```

 - iterator의 장점
    1) 반복자를 사용하면 고수준의 추상화를 할 수 있지만 , 컴파일시 성능은 저수준(for문)과 동일 합니다.
    2) 또한 런타임 오버헤드가 없기 때문에 반복자는 제로비용 추상화 입니다.

    (<span style="color:red">Zero coast abstractions</span> : 런타임시 코스트 발생 x ( 컴파일떄는 시간이 걸릴지언정))
