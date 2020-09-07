// SPDX-License-Identifier: Apache-2.0

//! This crate provides the `SignRel` trait which maps relationships between
//! integers that only differ by signedness. For example, both `a` and `b` in
//! this example have the type `usize`:
//!
//! ```rust
//! use signrel::SignRel;
//!
//! let a: <isize as SignRel>::Unsigned = 17;
//! let b: <usize as SignRel>::Unsigned = 17;
//!
//! assert_eq!(17usize, a);
//! assert_eq!(17usize, b);
//! ```

#![no_std]
#![deny(clippy::all)]
#![deny(missing_docs)]

/// The relationship between integers differing only by signedness
pub trait SignRel: Copy {
    /// The unsigned integer type with the same size as `Self`
    type Unsigned: SignRel;

    /// The signed integer type with the same size as `Self`
    type Signed: SignRel;
}

macro_rules! sr_impl {
    ($($s:ident:$u:ident)*) => (
        $(
            impl SignRel for $s {
                type Unsigned = $u;
                type Signed = $s;
            }

            impl SignRel for $u {
                type Unsigned = $u;
                type Signed = $s;
            }
        )+
    )
}

sr_impl! { isize:usize i128:u128 i64:u64 i32:u32 i16:u16 i8:u8 }
