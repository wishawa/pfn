# PFn
[<img alt="crates.io" src="https://img.shields.io/crates/v/pfn?style=for-the-badge" height="20">](https://crates.io/crates/pfn)
[<img alt="crates.io" src="https://img.shields.io/docsrs/pfn?style=for-the-badge" height="20">](https://docs.rs/pfn)
Provide [`fn_trait`](https://doc.rust-lang.org/nightly/unstable-book/library-features/fn-traits.html)'s
[`call`](https://doc.rust-lang.org/stable/core/ops/trait.Fn.html#tymethod.call),
[`call_mut`](https://doc.rust-lang.org/stable/core/ops/trait.FnMut.html#tymethod.call_mut), and
[`call_once`](https://doc.rust-lang.org/stable/core/ops/trait.FnOnce.html#tymethod.call_once)
on Stable Rust for functions / closures with ≤ 12 arguments.

## Examples
#### Basic usage
```rust
let closure = |x: i32, y: i32, z: String| {
	println!("{}", z);
	x + y
};

// Once the `fn_trait` feature has stabilized, you would do this.
let result = closure.call((5, 42, "Hello World"));

// For now, use PFn.
let result = closure.pfn_call((5, 42, "Hello World"));
```

#### Generalizing over functions with different number of arguments
```rust
// Here, Func can be a function or closure that takes any number of arguments (actually 1 - 12 arguments).
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
```