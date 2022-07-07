1.reference / ownership / borrow
------------------
- 힙영역에서 사용하는 자료들(런타임떄 크기 계산)은 rust에서 메모리 관리차원에서 스코프를 벗어나면 drop(free)하게 함
- 그러나 힙영역의 자료를 가르키는 변수들이 복수개(같은 것을 가르킨다면)가 되면 다중 free가 이뤄질 수 있는것이 문제(코더 직접 관리 힘듬)
- 그래서 하나의 변수만이 해당 힙역역 자료의 포인터를 가질 수 있는 것이 소유권(Ownership)개념 (소유권을 가지고 있는 변수는 owner라고 함)
- 소유권(Ownership)은 변수에서 변수로 move할 수 있음(함수의 매개변수로 이동 , 변수 대 변수의 대입으로 이동)
```rust
  fn print_country(country_name: String) {
      println!("{}", country_name);
  }
  fn main() {
      let country = String::from("Austria");
      let country_own =country;   //country에서 country_own로 소유권 이동
      print_country(country_own); //country_own에서 매개변수 country_name으로 소유권 이동
  }
```
- 위의 예시처럼 코드상에서 매번 소유권을 이리저리 넘겨(move)하는 것은 비효율적이고 불편함
- 그래서 소유권의 move 없이 그 값에 대한 참조와 수정을 가능케 하는 것이 reference의 개념
  (reference를 받는다 -> borrow 한다 라고도 표현함)
```rust
  fn add_hungary(country_name: &mut String) { // first we say that the function takes a mutable reference
      country_name.push_str("-Hungary"); // push_str() adds a &str to a String
      println!("Now it says: {}", country_name);
  }

  fn main() {
      let mut country = String::from("Austria");
      add_hungary(&mut country); // we also need to give it a mutable reference.
}
```
- reference 2가지 사용
  1) Mutable reference(unique) : &mut  (값의 변경도 가능함)
  ```rust
    fn main() {
      let mut my_number = 8;
      let num_ref = &mut my_number;
      *num_ref += 10; // Use * to change the i32 value.
      println!("{}", my_number);

    }
  ```
  2) Immutable reference(shared) :&  (참조만 함)
  ```rust
    fn main() {
      let country = String::from("Austria");
      let ref_one = &country;
      let ref_two = &country;

      println!("{}", ref_one);
    }
  ```
- mutable/immutable reference 사용의 룰
  1) 특정 값에 대한 mutable references는 복수개를 가질 수 있다.
  2) 특정 값에 대한 immutable references는 한개만 가질 수 있다.
  3) 특정 값에 대한 mutable ref와 immutable ref를 동시에 가질 수 없다.
     (mutable Ref는 값의 변화가 없는 참조인데 immutable은 값의 변화를 일으킬 수 있기 때문에 동시 사용 x)
