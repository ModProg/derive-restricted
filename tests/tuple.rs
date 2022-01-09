#![allow(clippy::clone_on_copy)]

mod util;

use std::cmp::Ordering;

use derive_where::derive_where;

use self::util::{
	AssertClone, AssertCopy, AssertDebug, AssertEq, AssertHash, AssertOrd, AssertPartialEq,
	AssertPartialOrd, Wrapper,
};

#[test]
fn single() {
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	struct Test<T>(Wrapper<T>);

	let test_1 = Test(42.into());
	let test_2 = Test(42.into());
	let test_le = Test(41.into());
	let test_ge = Test(43.into());

	let _ = AssertClone(&test_1);
	let _ = AssertCopy(&test_1);
	let _ = AssertDebug(&test_1);
	let _ = AssertEq(&test_1);
	let _ = AssertHash(&test_1);
	let _ = AssertOrd(&test_1);
	let _ = AssertPartialEq(&test_1);
	let _ = AssertPartialOrd(&test_1);

	let test_clone = test_1.clone();
	assert_eq!(test_clone.0, 42);

	let test_copy = test_1;
	assert_eq!(test_copy.0, 42);

	assert_eq!(format!("{:?}", test_1), "Test(42)");

	let test_default = Test::<i32>::default();
	assert_eq!(test_default.0, i32::default());

	util::hash_eq(test_1, test_2);
	util::hash_ne(test_1, test_ge);

	assert!(test_1 == test_2);
	assert!(test_1 != test_ge);

	assert_eq!(test_1.cmp(&test_2), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_le), Ordering::Greater);
	assert_eq!(test_1.cmp(&test_ge), Ordering::Less);

	assert_eq!(test_1.partial_cmp(&test_2), Some(Ordering::Equal));
	assert!(test_1 > Test(41.into()));
	assert!(test_1 < test_ge);
}

#[test]
fn multiple() {
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	struct Test<T>(Wrapper<T>, Wrapper<T>, Wrapper<T>);

	let test_1 = Test(42.into(), 43.into(), 44.into());
	let test_2 = Test(42.into(), 43.into(), 44.into());
	let test_ge_1 = Test(43.into(), 43.into(), 44.into());
	let test_ge_2 = Test(42.into(), 44.into(), 44.into());
	let test_ge_3 = Test(42.into(), 43.into(), 45.into());
	let test_le_1 = Test(41.into(), 43.into(), 44.into());
	let test_le_2 = Test(42.into(), 42.into(), 44.into());
	let test_le_3 = Test(42.into(), 43.into(), 43.into());

	let _ = AssertClone(&test_1);
	let _ = AssertCopy(&test_1);
	let _ = AssertDebug(&test_1);
	let _ = AssertEq(&test_1);
	let _ = AssertHash(&test_1);
	let _ = AssertOrd(&test_1);
	let _ = AssertPartialEq(&test_1);
	let _ = AssertPartialOrd(&test_1);

	let test_clone = test_1.clone();
	assert_eq!(test_clone.0, 42);
	assert_eq!(test_clone.1, 43);
	assert_eq!(test_clone.2, 44);

	let test_copy = test_1;
	assert_eq!(test_copy.0, 42);
	assert_eq!(test_copy.1, 43);
	assert_eq!(test_copy.2, 44);

	assert_eq!(format!("{:?}", test_1), "Test(42, 43, 44)");

	let test_default = Test::<i32>::default();
	assert_eq!(test_default.0, i32::default());
	assert_eq!(test_default.1, i32::default());
	assert_eq!(test_default.2, i32::default());

	util::hash_eq(test_1, test_2);
	util::hash_ne(test_1, test_ge_1);
	util::hash_ne(test_1, test_ge_2);
	util::hash_ne(test_1, test_ge_3);

	assert!(test_1 == test_2);
	assert!(test_1 != test_ge_1);
	assert!(test_1 != test_ge_2);
	assert!(test_1 != test_ge_3);
	assert!(test_1 != Test(45.into(), 45.into(), 45.into()));

	assert_eq!(test_1.cmp(&test_2), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_le_1), Ordering::Greater);
	assert_eq!(test_1.cmp(&test_le_2), Ordering::Greater);
	assert_eq!(test_1.cmp(&test_le_3), Ordering::Greater);
	assert_eq!(test_1.cmp(&test_ge_1), Ordering::Less);
	assert_eq!(test_1.cmp(&test_ge_2), Ordering::Less);
	assert_eq!(test_1.cmp(&test_ge_3), Ordering::Less);

	assert_eq!(test_1.partial_cmp(&test_2), Some(Ordering::Equal));
	assert!(test_1 > test_le_1);
	assert!(test_1 > test_le_2);
	assert!(test_1 > test_le_3);
	assert!(test_1 < test_ge_1);
	assert!(test_1 < test_ge_2);
	assert!(test_1 < test_ge_3);
}
