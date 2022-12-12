# lzf-sys

Rust wrapper of `liblzf`. This version is based on release 3.6.

The library can be built statically using the crate feature `static`. License is a BSD-2-Clause for `liblzf`.

========================chenkx: to more like [Compress::LZF](https://metacpan.org/pod/Compress::LZF) in perl========================

- [x] decompress function
- [x] compress function
- [ ] compress-best function


```shell
cargo test --features "paranoid"
cargo test --features "paranoid"  -- --nocapture
```
