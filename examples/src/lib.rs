#![no_std]

use std::X;

pub struct Y {
    x: X,
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);

    let x = X {};

    let y = Y { x };
}
