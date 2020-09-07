// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![deny(clippy::all)]

/// Expressed relationship between integers differing only by signedness.
pub trait SignRel {
    /// The unsigned integer type with the same size as Self.
    type Unsigned;

    /// The signed integer type with the same size as Self.
    type Signed;
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
