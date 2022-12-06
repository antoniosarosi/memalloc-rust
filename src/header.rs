use std::ptr::NonNull;

use crate::list::Node;

/// Since all the headers we store point to their previous and next header we
/// might as well consider them linked list nodes. This is just a type alias
/// that we use when we want to refer to a block or region header without
/// thinking about linked list nodes.
pub type Header<T> = Node<T>;

impl<T> Header<T> {
    /// Returns a pointer to a [`Header<T>`] given an address that points right
    /// after a valid [`Header<T>`].
    ///
    /// ```text
    /// +-------------+
    /// |  Header<T>  | <- Returned address points here.
    /// +-------------+
    /// |   Content   | <- Given address should point here.
    /// +-------------+
    /// |     ...     |
    /// +-------------+
    /// |     ...     |
    /// +-------------+
    /// ```
    ///
    /// # Safety
    ///
    /// Caller must guarantee that the given address points exactly to the first
    /// memory cell after a [`Header<T>`]. This function will be mostly used for
    /// deallocating memory, so the allocator user should give us an address
    /// that we previously provided when allocating. As long as that's true,
    /// this is safe, otherwise it's undefined behaviour.
    pub unsafe fn from_content_address(address: NonNull<u8>) -> NonNull<Self> {
        NonNull::new_unchecked(address.as_ptr().cast::<Self>().offset(-1))
    }

    /// Returns the address after the header.
    ///
    /// ```text
    /// +---------+
    /// | Header  | <- Header<T> struct.
    /// +---------+
    /// | Content | <- Returned address points to the first cell after header.
    /// +---------+
    /// |   ...   |
    /// +---------+
    /// |   ...   |
    /// +---------+
    /// ```
    ///
    /// # Safety
    ///
    /// If `header` is a valid [`NonNull<Header<T>>`], the offset will return an
    /// address that points right after the header. That address is safe to use
    /// as long as no more than `size` bytes are written, where `size` is a
    /// field of [`Block`] or [`Region`].
    ///
    /// # Notes
    ///
    /// - We are using this function as `Header::content_address_of(header)`
    /// because we want to avoid creating references to `self` to keep Miri
    /// happy. See [Stacked Borrows](https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md).
    pub unsafe fn content_address_of(header: NonNull<Self>) -> NonNull<u8> {
        NonNull::new_unchecked(header.as_ptr().offset(1) as *mut u8)
    }
}
