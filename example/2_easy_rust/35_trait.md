1.blanket trait
------------
 - 내가 사용하고자 하는 모든 타입에서 특정 트레잇을 구현하고자 할떄 사용하는 트레잇(제네릭 표현을 사용하여 구현)
 - 간단한 blanket trait 사용법
 ```rust
  trait Prints{
    fn prints_something(&self){
        println!("I like to print things");
    }
  }

  struct Person;
  struct Building;

  //모든타입에 Prints트레잇이 구현됨
  impl<T> Prints for T{}

  fn main(){
    let my_person = Person;
    let my_building = Building;
    my_person.prints_something();
    my_building.prints_something();
    let x = String::from("hello");
    x.prints_something();
  }
 ```
 - 자기 자신을 debug 하고자 할떄 다음과 같이 변경
 ```rust
  use std::fmt::Debug;

  trait Prints:Debug{
    fn prints_something(&self){
        println!("I am : {:?}",self);
    }
  }
  #[derive(Debug)]
  struct Person;
  struct Building;

  impl<T> Prints for T{}

  fn main(){
    let my_person = Person;
    let my_building = Building;
    my_person.prints_something();
    let x = String::from("hello");
    x.prints_something();
  }
 ```

 - 자기 자신을 print/debug 하고자 할떄 다음과 같이 변경
 ```rust
  use std::fmt::{Debug,Display};

  trait Prints{
    fn debug_something(&self) where Self:Debug{
        println!("I am : {:?}",self);
    }
    fn display_something(&self)where Self:Display{
        println!("I am : {}",self);
    }
  }
  #[derive(Debug)]
  struct Person;

  #[derive(Display)]
  struct Building;

  impl<T> Prints for T{}

  fn main(){
    let my_person = Person;
    let my_building = Building;
    my_person.debug_something();
    my_building.display_something();
  }
 ```


2.AsRef trait
--------------
 - 특정 타입 만 올 수 있도록 하는 트레잇
 ```rust
    use std::fmt::{Debug, Display}; // add Debug

    fn print_it<T>(input: T) // Now this line is easy to read
    where
        T: AsRef<str> + Debug + Display, // and these traits are easy to read
    {
        println!("{}", input)
    }

    fn main() {
        print_it("Please print me");
        print_it("Also, please print me".to_string());
        print_it(7); //This will not print
    }
 ```
 위의 예제에서는 print_it(7)은 AsRef<Str>로 인해 에러 발생 합니다.
