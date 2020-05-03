use crate::math;

pub trait FixedZList<L: math::Nat> {
    fn reify() -> Vec<i32>;
}

pub struct EmptyZList;
impl FixedZList<math::ZNat> for EmptyZList {
    fn reify() -> Vec<i32> { vec![] }
}

pub struct ConsZList<Z: math::Integer, Lm1: math::Nat, Nxt: FixedZList<Lm1>>(Z, PhantomData<Nxt>);
impl<Z: math::Integer, Lm1: math::Nat, Nxt: FixedZList<Lm1>> FixedZList<math::SNat<Lm1>> for ConsZList<Z, Lm1, Nxt>{
    fn reify() -> Vec<i32> {
	let mut rest = Nxt::reify();
	rest.insert(0, Z::reify());
	rest
    }
}

// Hereafter, math::ZeroIntMinus is code for "Option::None" but at the type level
// and just for the sort of Option<dyn math::Integer> we're dealing with.
// TODO: decide if there are more elegant ways to do this.
pub trait ZListNth<L: math::Nat, N: math::Nat>: FixedZList<L> {
    type Value: math::Integer;
}

impl<L: math::Nat, N: math::Nat> ZListNth<L, N> for EmptyZList {
    type Value = ZeroIntMinus;
}

impl<L: math::Nat, Z: math::Integer, Nxt: FixedZList<L>> ZListNth<math::SNat<L>, math::ZNat> for ConsZList<Z, L, Nxt> {
    type Value = Z;
}

impl<L: math::Nat, N: math::Nat, Z: math::Integer, Nxt: FixedZList<L> + ZListNth<L, N>> ZListNth<math::SNat<L>, N> for ConsZList<Z, L, Nxt> {
    type Value = <Nxt as ZListNth<L, N>>::Value;
}
