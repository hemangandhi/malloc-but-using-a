use std::marker::PhantomData;

pub trait Freeness {
    fn reify() -> bool;
}

pub struct IsFree;
impl Freeness for IsFree {
    fn reify() -> bool { true }
}

pub struct IsNotFree;
impl Freeness for IsNotFree {
    fn reify() -> bool { false }
}

// A list that knows its length as a type and allows for
// safe toggling of the elements between two types.
pub trait FixedToggleList<A, B, L: Nat> {}

pub struct EmptyList;
impl<A, B> FixedToggleList<A, B, ZNat> for EmptyList {}

pub struct ConsLList<A, B, Lm1: Nat, Nxt: FixedList<A, B, Lm1>>(A, PhantomData<Nxt>, PhantomData<Lm1>);
impl<A, B, Lm1: Nat> FixedToggleList<A, B, SNat<Lm1>> for ConsLList<A, B, Lm1> {}

pub struct ConsRList<A, B, Lm1: Nat, Nxt: FixedList<A, B, Lm1>>(B, PhantomData<Nxt>, PhantomData<Lm1>);
impl<A, B, Lm1: Nat> FixedToggleList<A, B, SNat<Lm1>> for ConsLList<A, B, Lm1> {}

pub static CELL_DATA: [PhantomData<T: Freeness>; 100] = [IsFree; 100];
