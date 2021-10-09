#![no_std]
extern crate alloc;
use alloc::boxed::Box;

pub struct GoodnightCallback(Option<Box<dyn FnOnce()>>);
impl GoodnightCallback
{
	pub fn new(callback: Box<dyn FnOnce()>) -> Self
	{
		Self(Some(callback))
	}
}

impl Drop for GoodnightCallback
{
	fn drop(&mut self)
	{
		(self.0.take().unwrap())();
	}
}
#[macro_export]
macro_rules! goodnight
{
	($closure:ident) => {
		let _goodnight_callback_whatever = $crate::GoodnightCallback::new(Bow::new($closure));
	};
	(move || $callback:expr) => {
		let close = move || $callback;
		goodnight!(close);
	};
	(|| $callback:expr) => {
		let close = || $callback;
		goodnight!(close);
	};
	($callback:expr) => {
		let close = || $callback;
		goodnight!(close);
	};
	($callback:block) =>
	{
		let close = || $cb;
		goodnight!(close);
	};
}
