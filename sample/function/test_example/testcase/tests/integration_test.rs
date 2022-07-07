//integration test
mod common;

#[test]
fn integ_test1() {
    common::setup();
    assert_eq!(4, 4);
}
#[test]
fn integ_test2() {
    common::setup();
    assert_eq!(4, 2);
}
#[test]
fn integ_test3() {
    common::setup();
    assert_eq!(4, 4);
}
#[test]
fn integ_test4() {
    common::setup();
    assert_eq!(4, 2);
}
