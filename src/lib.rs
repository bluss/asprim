#![no_std]
#![doc(html_root_url="https://docs.rs/asprim/0.2/")]

/// Cast to a primitive numeric type using `as`.
///
/// `AsPrim` converts any primitive numeric type to any other,
/// using the regular cast operator `as`.
///
/// ```
/// use asprim::AsPrim;
///
/// // Compute the mean value using f64 then cast back
/// fn mean<P>(data: &[P]) -> P
///     where P: AsPrim
/// {
///     let mut sum = 0.;
///     for elt in data {
///         sum += elt.as_f64();
///     }
///     sum /= data.len() as f64;
///     sum.as_()
/// }
///
/// fn pi<P: AsPrim>() -> P {
///     3.1415926535897932384626433.as_()
/// }
/// ```
///
/// # Rust Version
///
/// Requires Rust 1.26 or later due to supporting as_u128
///
pub trait AsPrim : 'static + Copy {
    fn as_usize(self) -> usize;
    fn as_isize(self) -> isize;
    fn as_u128(self) -> u128;
    fn as_i128(self) -> i128;
    fn as_u64(self) -> u64;
    fn as_i64(self) -> i64;
    fn as_u32(self) -> u32;
    fn as_i32(self) -> i32;
    fn as_u16(self) -> u16;
    fn as_i16(self) -> i16;
    fn as_u8(self) -> u8;
    fn as_i8(self) -> i8;
    fn as_f32(self) -> f32;
    fn as_f64(self) -> f64;
    fn cast_from<T: AsPrim>(_: T) -> Self;
    /// Cast self to the type `T`
    #[inline(always)]
    fn as_<T: AsPrim>(self) -> T {
        T::cast_from(self)
    }
}

macro_rules! as_prim_impl {
    ($($method:ident $from:ty)*) => {
        $(
            impl AsPrim for $from {
                #[inline(always)]
                fn as_usize(self) -> usize { self as usize }
                #[inline(always)]
                fn as_isize(self) -> isize { self as isize }
                #[inline(always)]
                fn as_u128(self) -> u128 { self as u128 }
                #[inline(always)]
                fn as_i128(self) -> i128 { self as i128 }
                #[inline(always)]
                fn as_u64(self) -> u64 { self as u64 }
                #[inline(always)]
                fn as_i64(self) -> i64 { self as i64 }
                #[inline(always)]
                fn as_u32(self) -> u32 { self as u32 }
                #[inline(always)]
                fn as_i32(self) -> i32 { self as i32 }
                #[inline(always)]
                fn as_u16(self) -> u16 { self as u16 }
                #[inline(always)]
                fn as_i16(self) -> i16 { self as i16 }
                #[inline(always)]
                fn as_u8(self) -> u8 { self as u8 }
                #[inline(always)]
                fn as_i8(self) -> i8 { self as i8 }
                #[inline(always)]
                fn as_f32(self) -> f32 { self as f32 }
                #[inline(always)]
                fn as_f64(self) -> f64 { self as f64 }
                #[inline(always)]
                fn cast_from<T: AsPrim>(x: T) -> Self {
                    x.$method()
                }
            }
        )*
    }
}

as_prim_impl!{as_u8 u8 as_i8 i8 as_u16 u16 as_i16 i16 as_u32 u32 as_i32 i32
              as_u128 u128 as_i128 i128
              as_u64 u64 as_i64 i64 as_usize usize as_isize isize as_f32 f32 as_f64 f64}

#[cfg(test)]
mod tests {
    use super::AsPrim;
    #[test]
    fn it_works() {
        assert_eq!(1.as_::<f32>(), 1.0);
        assert_eq!(1i128.as_::<f32>(), 1.0);
    }
}
