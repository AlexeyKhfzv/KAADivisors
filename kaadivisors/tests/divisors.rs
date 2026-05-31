use kaadivisors::get_divisors;

#[test]
fn divisors() {
    assert_eq!(vec![(2, 2), (3, 1)], get_divisors(12));
}

#[test]
#[should_panic]
fn wrong_divisors() {
    assert_eq!(vec![(8, 1), (125, 1)], get_divisors(1000));
}
