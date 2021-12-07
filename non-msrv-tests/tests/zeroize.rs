#![cfg(feature = "zeroize")]

extern crate zeroize_ as zeroize;

mod util;

use std::{
	marker::PhantomData,
	ops::{Deref, DerefMut},
};

use derive_where::DeriveWhere;
use zeroize::Zeroize;

use self::util::Wrapper;

#[test]
fn basic() {
	#[derive(DeriveWhere)]
	#[derive_where(Zeroize)]
	struct Test<T>(Wrapper<T>);

	let mut test = Test(42.into());
	test.zeroize();

	assert_eq!(test.0, 0);

	util::test_drop(Test(42.into()), |test| assert_eq!(test.0, 42))
}

#[test]
fn crate_() {
	#[derive(DeriveWhere)]
	#[derive_where(Zeroize(crate = "zeroize_"))]
	struct Test<T>(Wrapper<T>);

	let mut test = Test(42.into());
	test.zeroize();

	assert_eq!(test.0, 0);

	util::test_drop(Test(42.into()), |test| assert_eq!(test.0, 42))
}

#[test]
fn drop() {
	#[derive(DeriveWhere)]
	#[derive_where(Zeroize(drop))]
	struct Test<T>(Wrapper<T>);

	let mut test = Test(42.into());
	test.zeroize();

	assert_eq!(test.0, 0);

	util::test_drop(Test(42.into()), |test| assert_eq!(test.0, 0))
}

#[test]
fn fqs() {
	struct Fqs<T>(Wrapper<T>);

	impl<T> Zeroize for Fqs<T> {
		fn zeroize(&mut self) {
			self.0.zeroize()
		}
	}

	impl<T> Fqs<T> {
		#[allow(dead_code)]
		fn zeroize(&mut self) {
			unimplemented!()
		}
	}

	#[derive(DeriveWhere)]
	#[derive_where(Zeroize(drop))]
	struct Test<T>(#[derive_where(Zeroize(fqs))] Fqs<T>);

	let mut test = Test(Fqs(42.into()));
	test.zeroize();

	assert_eq!(test.0 .0, 0);

	util::test_drop(Test(Fqs(42.into())), |test| assert_eq!(test.0 .0, 0))
}

#[test]
fn deref() {
	struct ZeroizeDeref<T>(i32, PhantomData<T>);

	impl<T> Deref for ZeroizeDeref<T> {
		type Target = i32;

		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}

	impl<T> DerefMut for ZeroizeDeref<T> {
		fn deref_mut(&mut self) -> &mut Self::Target {
			&mut self.0
		}
	}

	#[derive(DeriveWhere)]
	#[derive_where(Zeroize(drop))]
	struct Test<T>(ZeroizeDeref<T>);

	let mut test = Test::<()>(ZeroizeDeref(42, PhantomData));
	test.zeroize();

	assert_eq!(test.0 .0, 0);

	util::test_drop(Test::<()>(ZeroizeDeref(42, PhantomData)), |test| {
		assert_eq!(test.0 .0, 0)
	})
}
