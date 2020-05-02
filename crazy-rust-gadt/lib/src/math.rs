use std::marker::PhantomData;

// Booleans
pub trait Bool {
    fn reify() -> bool;
}
pub struct BTrue;
impl Bool for BTrue{
    fn reify() -> bool { true }
}
pub struct BFalse;
impl Bool for BFalse{
    fn reify() -> bool { false }
}

pub trait Nat: Clone {
    fn reify() -> u32;
}

// Zero is a number
#[derive(Clone)]
pub struct ZNat;
impl Nat for ZNat {
    fn reify() -> u32 { 0 }
}

// 1 + a number is a number (the successor)
#[derive(Clone)]
pub struct SNat<A: Nat = ZNat>(PhantomData<A>);
impl<A: Nat> Nat for SNat<A> {
    fn reify() -> u32 {
        1 + A::reify()
    }
}

// Addition of natural numbers
pub trait AddNats<A: Nat>: Nat{
    type Sum: Nat;
}
impl<T: Nat> AddNats<T> for ZNat {
    type Sum = T;
}
impl<T: Nat, U: AddNats<T> + Nat> AddNats<T> for SNat<U> {
    type Sum = SNat<<U as AddNats<T>>::Sum>;
}

type Plus<X: Nat, Y: Nat> = <X as AddNats<Y>>::Sum;

// The integers
pub trait Integer: Clone {
    type Positive: Bool;
    fn reify() -> i32;
}

pub struct ZeroIntPlus;
impl Integer for ZeroIntPlus {
    type Positive = BTrue;
    fn reify() -> i32 { 0 }
}

pub struct ZeroIntMinus;
impl Integer for ZeroIntMinus {
    type Positive = BFalse;
    fn reify() -> i32 { 0 }
}

pub struct PositiveInt<I: Integer<Positive = BTrue>>(PhantomData<I>);
impl<I: Integer> Integer for PositiveInt<I> {
    type Positive = BTrue;
    fn reify() -> i32 { 1 + I::reify() } 
}

pub struct NegativeInt<I: Integer<Postive = BFalse>>(PhantomData<I>);
impl Integer for NegativeInt {
    type Positive = False;
    fn reify() -> i32 { I::reify() - 1 } 
}

