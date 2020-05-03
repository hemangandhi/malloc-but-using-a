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
// The bool determines the direction so that you can't
// just take the predecessor of a positive number.
// TODO: figure out if there's a better representation.
// 2s complement fixed-size ints?
//   - would just be n Bool params for n bits.
//   - would need Bool operation traits.
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

// Addition
pub trait AddInts<I: Integer>: Integer {
    type Sum: Integer;
}

// Addition with 0 is trivial
impl<I: Integer> AddInts<I> for ZeroIntPlus {
    type Sum = I;
}

impl<I: Integer> AddInts<I> for ZeroIntMinus {
    type Sum = I;
}

// Addition with the same sign is recursive.
impl<I: Integer<Positive = BTrue>, J: AddInts<I> + Integer<Positive = BTrue>> AddInts<I> for PositiveInt<J> {
    type Sum = PositiveInt<<J as AddInts<I>>::Sum>;
}

impl<I: Integer<Positive = BFalse>, J: AddInts<I> + Integer<Positive = BFalse>> AddInts<I> for NegativeInt<J> {
    type Sum = NegativeInt<<J as AddInts<I>>::Sum>;
}

// This is about cancellation: (x + 1) + (y - 1) = x + y and its commuted version.
// Since one of these will reach 0 first, the other one will be the correct difference.
// TODO: make sure that the 0 case also commutes (0 + x = x is proven above, but x + 0 = x may not be).
//   - this may require more specificity in the impls to actually specify each struct instead of "Integer<Positive = B____>"
impl<I: Integer<Positive = BTrue>, J: AddInts<I> + Integer<Positive = BFalse>> AddInts<PositiveInteger<I>> for NegativeInteger<J> {
    type Sum = <J as AddInts<I>>::Sum;
}

impl<I: Integer<Positive = BFalse>, J: AddInts<I> + Integer<Positive = BTrue>> AddInts<NegativeInteger<I>> for PositiveInteger<J> {
    type Sum = <J as AddInts<I>>::Sum;
}

type PlusZ<X: Integer, Y: Integer> = <X as AddInts<Y>>::Sum;
