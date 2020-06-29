# const-sha1

A sha1 implementation useable in const contexts. 

## Use

 ```rust
 const fn signature() -> [u32; 5] {
     const_sha1::sha1(stringify!(MyType).as_bytes()).data
 }
 ```

 This crate currently requires nightly for [const if/match](https://github.com/rust-lang/rust/issues/49146) and [const loop](https://github.com/rust-lang/rust/issues/52000) which should be stable in Rust 1.46 which is due to release in the late August 2020 timeframe.

## Attribution

This code is largely inspired by the following repos:
* [vog/sha1](https://github.com/vog/sha1)
* [mitsuhiko/rust-sha1](https://github.com/mitsuhiko/rust-sha1)