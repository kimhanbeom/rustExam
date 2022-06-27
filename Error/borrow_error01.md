1.코드
------
```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!

    println!("the first word is: {}", word);
}
```

2.error message
-----------------
```rust
error: cannot borrow `s` as mutable because it is also borrowed as
            immutable [E0502]
    s.clear(); // Error!
    ^
note: previous borrow of `s` occurs here; the immutable borrow prevents
            subsequent moves or mutable borrows of `s` until the borrow ends
    let word = first_word(&s);
                           ^
note: previous borrow ends here
fn main() {

}
```

3.원인
-------
s에 대한 불변 참조자를 만들고 그 불변 참조자의 유효 scope(main)내에서

clear()를 통해 s의 수정을 진행하였기 때문에 에러 발생

(borrow rule에 의해 불변참조자와 가변참조자가 공존 할 수 없는 이유와 같음)
