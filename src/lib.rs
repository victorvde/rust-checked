#![feature(macro_rules)]
use std::num::{Int, SignedInt};
use std::fmt::{Show, Formatter, Result};

#[deriving(Copy)]
pub struct Checked<T : Int>(pub T);

impl<T : Int> Checked<T> {
    pub fn new(v : T) -> Checked<T> {
        Checked(v)
    }
}

impl<T : Int> Deref<T> for Checked<T> {
    fn deref(&self) -> &T {
        let Checked(ref v) = *self;
        v
    }
}

fn overflow_error() -> ! {
    panic!("overflow error")
}

macro_rules! impl_checked_trait(
    ($t:ident, $name:ident, $checked_op:ident) => (

impl<T : Int> $t<Checked<T>, Checked<T>> for Checked<T> {
    fn $name(self, y : Checked<T>) -> Checked<T> {
       match self.$checked_op(*y) {
           None => overflow_error(),
           Some(z) => Checked(z),
       }
    }
}

    );
);

impl_checked_trait!(Add, add, checked_add);
impl_checked_trait!(Sub, sub, checked_sub);
impl_checked_trait!(Mul, mul, checked_mul);
impl_checked_trait!(Div, div, checked_div);

macro_rules! impl_unchecked_trait(
    ($t:ident, $name:ident) => (

impl<T : Int> $t<Checked<T>, Checked<T>> for Checked<T> {
    fn $name(self, y : Checked<T>) -> Checked<T> {
       Checked((*self).$name(*y))
    }
}

    );
);

impl_unchecked_trait!(BitAnd, bitand);
impl_unchecked_trait!(BitOr, bitor);
impl_unchecked_trait!(BitXor, bitxor);
impl_unchecked_trait!(Rem, rem);

impl<T : SignedInt> Neg<Checked<T>> for Checked<T> {
    fn neg(self) -> Checked<T> {
        let min : T = Int::min_value();
        if *self == min {
            overflow_error()
        } else {
            Checked((*self).neg())
        }
    }
}

impl<T : Int + Not<T>> Not<Checked<T>> for Checked<T> {
    fn not(self) -> Checked<T> {
       Checked((*self).not())
    }
}

impl<T : Int + Shl<uint, T>> Shl<uint, Checked<T>> for Checked<T> {
    fn shl(self, y : uint) -> Checked<T> {
        let mut r = self;
        for _ in range(0, y) {
          r = r + r;
        }
        r
    }
}

impl<T : Int + Shr<uint, T>> Shr<uint, Checked<T>> for Checked<T> {
    fn shr(self, y : uint) -> Checked<T> {
       Checked((*self).shr(y))
    }
}

impl<T : Int + Show> Show for Checked<T> {
    fn fmt(&self, f : &mut Formatter) -> Result {
        (**self).fmt(f)
    }
}

impl<T : Int + PartialEq> PartialEq for Checked<T> {
    fn eq(&self, other : &Checked<T>) -> bool {
        (**self) == (**other)
    }
}
