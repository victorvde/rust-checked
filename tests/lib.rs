#![feature(macro_rules)]
extern crate checked;

macro_rules! test_it(
    ($t:ty, $n:ident) => (

pub mod $n {

use checked::Checked;
use std::num::Int;

#[test]
fn test_add() {
    let x : Checked<$t> = Checked(1);
    assert_eq!(*(x + x), 2);
    let y : Checked<$t> = Checked(3);
    assert_eq!(*(x + y), 4);
    let max : $t = Int::max_value();
    let z : Checked<$t> = Checked(max - 1);
    assert_eq!(*(x + z), max);
}

#[test]
#[should_fail]
fn test_add_fail() {
   let max : $t = Int::max_value();
   Checked(max) + Checked(1);
}

#[test]
fn test_sub() {
    let x : Checked<$t> = Checked(1);
    assert_eq!(*(x - x), 0);
    let y : Checked<$t> = Checked(3);
    assert_eq!(*(y - x), 2);
    let min : $t = Int::min_value();
    let z : Checked<$t> = Checked(min + 1);
    assert_eq!(*(z - x), min);
}

#[test]
#[should_fail]
fn test_sub_fail() {
   let min : $t = Int::min_value();
   Checked(min) - Checked(1);
}

#[test]
fn test_mul() {
    let x : Checked<$t> = Checked(1);
    assert_eq!(*(x * x), 1);
    let y : Checked<$t> = Checked(3);
    assert_eq!(*(y * x), 3);
    let min : $t = Int::min_value();
    let z : Checked<$t> = Checked(min);
    assert_eq!(*(z * x), min);
    let max : $t = Int::max_value();
    let w : Checked<$t> = Checked(max);
    assert_eq!(*(w * x), max);
}

#[test]
#[should_fail]
fn test_mul_fail() {
   let max : $t = Int::max_value();
   Checked(max) * Checked(2);
}

#[test]
fn test_div() {
    let x : Checked<$t> = Checked(1);
    assert_eq!(*(x / x), 1);
    let y : Checked<$t> = Checked(3);
    assert_eq!(*(x / y), 0);
    let min : $t = Int::min_value();
    let z : Checked<$t> = Checked(min);
    assert_eq!(*(z / x), min);
    let max : $t = Int::max_value();
    let w : Checked<$t> = Checked(max);
    assert_eq!(*(w / x), max);
}

// TODO: test div fail for signed

#[test]
fn test_bitand() {
   let x : Checked<$t> = Checked(5);
   let y : Checked<$t> = Checked(3);
   assert_eq!(x&y, Checked(1));
}

#[test]
fn test_bitor() {
   let x : Checked<$t> = Checked(1);
   let y : Checked<$t> = Checked(2);
   assert_eq!(x|y, Checked(3));
}

#[test]
fn test_bitxor() {
   let x : Checked<$t> = Checked(5);
   let y : Checked<$t> = Checked(3);
   assert_eq!(x^y, Checked(6));
}

#[test]
fn test_rem() {
    let x : Checked<$t> = Checked(5);
    let y : Checked<$t> = Checked(3);
    assert_eq!(x % y, Checked(2));
}

#[test]
fn test_shl() {
    let x : Checked<$t> = Checked(1);
    assert_eq!(x << 1, Checked(2));
}

#[test]
#[should_fail]
fn test_shl_fail() {
    let max : $t = Int::max_value();
    Checked(max) << 1;
}

}
))

macro_rules! test_signed(
    ($t:ty, $n:ident) => (
pub mod $n {

use checked::Checked;
use std::num::Int;

#[test]
#[should_fail]
fn test_div_fail() {
    let min : $t = Int::min_value();
    Checked(min) / Checked(-1);
}

#[test]
fn test_neg() {
    let x : Checked<$t> = Checked(-1);
    assert_eq!(-x, Checked(1));
}

#[test]
#[should_fail]
fn test_neg_fail() {
    let min : $t = Int::min_value();
    -Checked(min);
}

#[test]
fn test_shl_s() {
    let x : Checked<$t> = Checked(-1);
    assert_eq!(x << 1, Checked(-2));
}


#[test]
#[should_fail]
fn test_shl_fail_s() {
    let min : $t = Int::min_value();
    Checked(min) << 1;
}

}
))

test_it!(u8, test_u8)
test_it!(u16, test_u16)
test_it!(u32, test_u32)
test_it!(u64, test_u64)
test_it!(uint, test_uint)

test_it!(i8, test_i8)
test_it!(i16, test_i16)
test_it!(i32, test_i32)
test_it!(i64, test_i64)
test_it!(int, test_int)

test_signed!(i8, test_i8_s)
test_signed!(i16, test_i16_s)
test_signed!(i32, test_i32_s)
test_signed!(i64, test_i64_s)
test_signed!(int, test_int_s)
