# What?

A provably correct version of malloc implemented using GADTs in Rust.

# How

Same.

To understand:

- How to have trait objects without a box?
  - How to manage the size of various proof terms?
- What is a useful Rust allocator API? (This will be done after making a useless one.)

## Trait objects without a box

Phantom data?