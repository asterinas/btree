//! B-Tree implementations.

#![no_std]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![feature(dropck_eyepatch)]
#![feature(error_in_core)]
#![feature(exclusive_range_pattern)]
#![feature(extend_one)]
#![feature(hasher_prefixfree_extras)]
#![feature(inline_const)]
#![feature(maybe_uninit_slice)]
#![feature(min_specialization)]
#![feature(new_uninit)]
#![feature(slice_ptr_get)]

extern crate alloc;

mod btree_from_alloc;

pub mod btree_map {
    //! An ordered map based on a B-Tree.
    pub use super::btree_from_alloc::map::*;
}

pub mod btree_set {
    //! An ordered set based on a B-Tree.
    pub use super::btree_from_alloc::set::*;
}

pub use btree_map::BTreeMap;
pub use btree_set::BTreeSet;
