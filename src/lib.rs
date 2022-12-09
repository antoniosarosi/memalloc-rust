#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(is_some_and)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(strict_provenance)]

use std::ptr::NonNull;

mod alignment;
mod allocator;
mod block;
mod bucket;
mod freelist;
mod header;
mod list;
mod mmap;
mod region;

/// Non-null pointer to `T`. We use this in most cases instead of `*mut T`
/// because the compiler will yell at us if we don't write code for the `None`
/// case. I think variance doesn't have much implications here except for
/// [`list::LinkedList<T>`], but that should probably be covariant anyway.
pub(crate) type Pointer<T> = Option<NonNull<T>>;

pub use allocator::MmapAllocator;
