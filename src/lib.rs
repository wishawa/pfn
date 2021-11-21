//! # PFn
//! 
//! Provide [`fn_trait`](https://doc.rust-lang.org/nightly/unstable-book/library-features/fn-traits.html)'s
//! [`call`][std::ops::Fn::call],
//! [`call_mut`][std::ops::FnMut::call_mut], and
//! [`call_once`][std::ops::FnOnce::call_once]
//! on Stable Rust for functions / closures with ≤ 12 arguments.
//! 
//! ## Example Usage
//! ```rust
//! use pfn::PFn;
//! let closure = |x: i32, y: i32, z: &str| {
//! 	println!("{}", z);
//! 	x + y
//! };
//! 
//! // Once the `fn_trait` feature has stabilized, you would do this.
//! // let result = closure.call((5, 42, "Hello World"));
//! 
//! // For now, use PFn.
//! let result = closure.pfn_call((5, 42, "Hello World"));
//! ```

/// A trait implementing [`call_once`][std::ops::FnOnce::call_once] as `pfn_call_once`.
pub trait PFnOnce<Args> {
    type PFnOutput;
    fn pfn_call_once(self, args: Args) -> Self::PFnOutput;
}

/// A trait implementing [`call_mut`][std::ops::FnMut::call_mut] as `pfn_call_mut`.
pub trait PFnMut<Args>: PFnOnce<Args> {
    fn pfn_call_mut(&mut self, args: Args) -> Self::PFnOutput;
}

/// A trait implementing [`call`][std::ops::Fn::call] as `pfn_call`.
pub trait PFn<Args>: PFnMut<Args> {
    fn pfn_call(&self, args: Args) -> Self::PFnOutput;
}

macro_rules! impl_variadic_call {
	( $($name: ident)* ) => {
		impl<$($name,)* ZO, ZF: FnOnce($($name),*) -> ZO> PFnOnce<($($name,)*)> for ZF {
            type PFnOutput = ZO;
			fn pfn_call_once(self, args: ($($name,)*)) -> ZF::Output {
                #![allow(non_snake_case)]
                let ($($name,)*) = args;
                self($($name),*)
            }
		}
        impl<$($name,)* ZO, ZF: FnMut($($name),*) -> ZO> PFnMut<($($name,)*)> for ZF {
			fn pfn_call_mut(&mut self, args: ($($name,)*)) -> ZF::Output {
                #![allow(non_snake_case)]
                let ($($name,)*) = args;
                self($($name),*)
            }
		}
        impl<$($name,)* ZO, ZF: Fn($($name),*) -> ZO> PFn<($($name,)*)> for ZF {
			fn pfn_call(&self, args: ($($name,)*)) -> ZF::Output {
                #![allow(non_snake_case)]
                let ($($name,)*) = args;
                self($($name),*)
            }
		}
	};
}

impl_variadic_call! { }
impl_variadic_call! { A }
impl_variadic_call! { A B }
impl_variadic_call! { A B C }
impl_variadic_call! { A B C D }
impl_variadic_call! { A B C D E }
impl_variadic_call! { A B C D E F }
impl_variadic_call! { A B C D E F G }
impl_variadic_call! { A B C D E F G H }
impl_variadic_call! { A B C D E F G H I }
impl_variadic_call! { A B C D E F G H I J }
impl_variadic_call! { A B C D E F G H I J K }
impl_variadic_call! { A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clos_0() {
        let mut clos = || {
            42
        };
        assert_eq!(clos.pfn_call(()), 42);
        assert_eq!(clos.pfn_call_mut(()), 42);
        assert_eq!(clos.pfn_call_once(()), 42);
    }
    #[test]
    fn clos_1() {
        let mut clos = |x: i32| {
            x * 2
        };
        assert_eq!(clos.pfn_call((1, )), 2);
        assert_eq!(clos.pfn_call_mut((2, )), 4);
        assert_eq!(clos.pfn_call_once((3, )), 6);
    }
    #[test]
    fn generic() {
        struct Runnable<Args, Func: PFnOnce<Args>> {
            func: Func,
            args: Args
        }
        impl<Args, Func: PFnOnce<Args>> Runnable<Args, Func> {
            fn run(self) -> Func::PFnOutput {
                (self.func).pfn_call_once(self.args)
            }
        }
        let runnable = Runnable {
            func: |mut x: String| {
                x.push_str("!!!");
                x
            },
            args: ("hello world".into(),)
        };
        assert_eq!(runnable.run(), "hello world!!!");
    }
    #[test]
    fn mutating() {
        let mut number: i32 = 0;
        let mut clos = |x: i32| {
            number += x;
        };
        clos.pfn_call_mut((5, ));
        clos.pfn_call_mut((6, ));
        clos.pfn_call_once((7, ));
        assert_eq!(number, 18);
    }
}
