//
// Copyright 2018 Red Hat, Inc.
//
// Author: Nathaniel McCallum <npmccallum@redhat.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

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

sr_impl! { isize:usize i64:u64 i32:u32 i16:u16 i8:u8 }

#[cfg(has_i128)]
sr_impl! { i128:u128 }
