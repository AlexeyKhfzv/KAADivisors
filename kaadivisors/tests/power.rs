use kaadivisors::get_power;

#[test]
fn power() {
    let mut number = 8;
    assert_eq!(3, get_power(&mut number, 2));
}

#[test]
#[should_panic]
fn wrong_power() {
    let mut number = 1024;
    assert_eq!(11, get_power(&mut number, 2));
}
