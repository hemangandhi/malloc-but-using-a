# What?

A provably correct version of malloc implemented using GADTs in Rust.

# How

Same.

To understand:

- How to have trait objects without a box?
  - How to manage the size of various proof terms?
- What is a useful Rust allocator API? (This will be done after making a useless one.)

## Trait objects without a box (and other sizing notes)

Phantom data?

Currently everything seems to fit into the generic params. Hence, every value is a type, so if the heap is n bytes, we have 2n Integer types and potentially n cons cells, so we reach 2n^2 types. However, each list also has its own Nat type adding an n factor and the intermediate lists would have had to be shorter lists with different cons cells after them.

Is generating 10 million types worth 5K bytes of heap? Or would the compiler not actually instantiate all the types until a particular usage pattern calls for it?

# TODO List

- [x] Basic integers GADT
- [x] List GADT with indexing
- [ ] Refl for integers, bools, and nats.
- [ ] List replacement function with refl-based verification
- [ ] Allocator that doesn't group contiguous free blocks
- [ ] Allocator that groups contiguous free blocks
- [ ] Cleaner API into GADT-land for parity with malloc and free (or desirable Rust API).

# Alternatives to Consider

| Current thing | Alternative | Doubts |
|---|---|---|
| Implementation of any GADT | Use of the type_operator crate | How to have the `FixedZList`? (Also, I find this easier to read and debug for now, so this is a refactor at best.) |
| Implementation of Ints | A `TypeShort` type with 16 `Bool` parameters to be a 2s complement int16 | Less elegant and 2s complement logic will be a mess (would need a `HalfAdder` and `FullAdder` type operator). Note also: the HoTT construction seems to use an equivalence on `Nat x Nat` which might be shitty to write out in Rust. |
| List indexing trait's (`ZListNth`) use of `ZeroIntMinus` | A type-level optional integer. | Not sure why `ZeroIntMinus` is not yet good enough. Perhaps since this confuses the refl types for Integers since ZeroIntMinus /= ZeroIntPlus now? Also: how to unbox the optional? |