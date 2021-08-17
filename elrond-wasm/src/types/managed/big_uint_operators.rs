use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

use crate::api::ManagedTypeApi;

use super::BigUint;

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait for BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: BigUint<M>) -> BigUint<M> {
                self.api.$api_func(self.handle, self.handle, other.handle);
                BigUint {
                    handle: self.handle,
                    api: self.api.clone(),
                }
            }
        }

        impl<'a, 'b, M: ManagedTypeApi> $trait<&'b BigUint<M>> for &'a BigUint<M> {
            type Output = BigUint<M>;

            fn $method(self, other: &BigUint<M>) -> BigUint<M> {
                let result = self.api.bi_new_zero();
                self.api.$api_func(result, self.handle, other.handle);
                BigUint {
                    handle: result,
                    api: self.api.clone(),
                }
            }
        }
    };
}

binary_operator! {Add, add, bi_add}
binary_operator! {Sub, sub, bi_sub}
binary_operator! {Mul, mul, bi_mul}
binary_operator! {Div, div, bi_t_div}
binary_operator! {Rem, rem, bi_t_mod}

macro_rules! binary_assign_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl<M: ManagedTypeApi> $trait<BigUint<M>> for BigUint<M> {
            #[inline]
            fn $method(&mut self, other: Self) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }

        impl<M: ManagedTypeApi> $trait<&BigUint<M>> for BigUint<M> {
            #[inline]
            fn $method(&mut self, other: &BigUint<M>) {
                self.api.$api_func(self.handle, self.handle, other.handle);
            }
        }
    };
}

binary_assign_operator! {AddAssign, add_assign, bi_add}
binary_assign_operator! {SubAssign, sub_assign, bi_sub}
binary_assign_operator! {MulAssign, mul_assign, bi_mul}
binary_assign_operator! {DivAssign, div_assign, bi_t_div}
binary_assign_operator! {RemAssign, rem_assign, bi_t_mod}

impl<M: ManagedTypeApi> PartialEq for BigUint<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.api.bi_cmp(self.handle, other.handle).is_eq()
    }
}

impl<M: ManagedTypeApi> Eq for BigUint<M> {}

impl<M: ManagedTypeApi> PartialOrd for BigUint<M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: ManagedTypeApi> Ord for BigUint<M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.api.bi_cmp(self.handle, other.handle)
    }
}

fn cmp_i64<M: ManagedTypeApi>(bi: &BigUint<M>, other: i64) -> Ordering {
    if other == 0 {
        match bi.api.bi_sign(bi.handle) {
            crate::api::Sign::Plus => Ordering::Greater,
            crate::api::Sign::NoSign => Ordering::Equal,
            crate::api::Sign::Minus => Ordering::Less,
        }
    } else {
        bi.api.bi_cmp(bi.handle, bi.api.bi_new(other))
    }
}

impl<M: ManagedTypeApi> PartialEq<i64> for BigUint<M> {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        cmp_i64(self, *other).is_eq()
    }
}

impl<M: ManagedTypeApi> PartialOrd<i64> for BigUint<M> {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        Some(cmp_i64(self, *other))
    }
}
