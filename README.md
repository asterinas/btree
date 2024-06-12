# btree
This repo provides B-Tree implementation (mainly `BTreeMap`) for [`MlsDisk`](https://github.com/asterinas/mlsdisk) target on [`Rust-for-Linux`](https://rust-for-linux.com/).

Currently, the implementation is vendored from `alloc::collections`, for two reasons:
- The `alloc` crate compiled by `Rust-for-Linux` is incomplete.
- The `alloc::collections::BTreeMap` is also used by other target os of `MlsDisk`.

`TODO`: we could implement `BTreeMap` based on C structs of kernel.
