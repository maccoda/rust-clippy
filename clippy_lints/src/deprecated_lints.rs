// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


macro_rules! declare_deprecated_lint {
    (pub $name: ident, $_reason: expr) => {
        declare_lint!(pub $name, Allow, "deprecated lint")
    }
}

/// **What it does:** Nothing. This lint has been deprecated.
///
/// **Deprecation reason:** This used to check for `assert!(a == b)` and recommend 
/// replacement with `assert_eq!(a, b)`, but this is no longer needed after RFC 2011.
declare_deprecated_lint! {
    pub SHOULD_ASSERT_EQ,
    "`assert!()` will be more flexible with RFC 2011"
}

/// **What it does:** Nothing. This lint has been deprecated.
///
/// **Deprecation reason:** This used to check for `Vec::extend`, which was slower than
/// `Vec::extend_from_slice`. Thanks to specialization, this is no longer true.
declare_deprecated_lint! {
    pub EXTEND_FROM_SLICE,
    "`.extend_from_slice(_)` is a faster way to extend a Vec by a slice"
}

/// **What it does:** Nothing. This lint has been deprecated.
///
/// **Deprecation reason:** `Range::step_by(0)` used to be linted since it's
/// an infinite iterator, which is better expressed by `iter::repeat`,
/// but the method has been removed for `Iterator::step_by` which panics
/// if given a zero
declare_deprecated_lint! {
    pub RANGE_STEP_BY_ZERO,
    "`iterator.step_by(0)` panics nowadays"
}

/// **What it does:** Nothing. This lint has been deprecated.
///
/// **Deprecation reason:** This used to check for `Vec::as_slice`, which was unstable with good
/// stable alternatives. `Vec::as_slice` has now been stabilized.
declare_deprecated_lint! {
    pub UNSTABLE_AS_SLICE,
    "`Vec::as_slice` has been stabilized in 1.7"
}


/// **What it does:** Nothing. This lint has been deprecated.
///
/// **Deprecation reason:** This used to check for `Vec::as_mut_slice`, which was unstable with good
/// stable alternatives. `Vec::as_mut_slice` has now been stabilized.
declare_deprecated_lint! {
    pub UNSTABLE_AS_MUT_SLICE,
    "`Vec::as_mut_slice` has been stabilized in 1.7"
}

/// **What it does:** Nothing. This lint has been deprecated.
///
/// **Deprecation reason:** This used to check for `.to_string()` method calls on values
/// of type `&str`. This is not unidiomatic and with specialization coming, `to_string` could be
/// specialized to be as efficient as `to_owned`.
declare_deprecated_lint! {
    pub STR_TO_STRING,
    "using `str::to_string` is common even today and specialization will likely happen soon"
}

/// **What it does:** Nothing. This lint has been deprecated.
///
/// **Deprecation reason:** This used to check for `.to_string()` method calls on values
/// of type `String`. This is not unidiomatic and with specialization coming, `to_string` could be
/// specialized to be as efficient as `clone`.
declare_deprecated_lint! {
    pub STRING_TO_STRING,
    "using `string::to_string` is common even today and specialization will likely happen soon"
}

/// **What it does:** Nothing. This lint has been deprecated.
///
/// **Deprecation reason:** This lint should never have applied to non-pointer types, as transmuting
/// between non-pointer types of differing alignment is well-defined behavior (it's semantically
/// equivalent to a memcpy). This lint has thus been refactored into two separate lints:
/// cast_ptr_alignment and transmute_ptr_to_ptr.
declare_deprecated_lint! {
    pub MISALIGNED_TRANSMUTE,
    "this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr"
}

/// **What it does:** Nothing. This lint has been deprecated.
///
/// **Deprecation reason:** This lint is too subjective, not having a good reason for being in clippy.
/// Additionally, compound assignment operators may be overloaded separately from their non-assigning
/// counterparts, so this lint may suggest a change in behavior or the code may not compile.
declare_deprecated_lint! {
    pub ASSIGN_OPS,
    "using compound assignment operators (e.g. `+=`) is harmless"
}
