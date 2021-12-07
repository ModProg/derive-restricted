#![allow(clippy::clone_on_copy)]

#[macro_use]
mod util;

use std::cmp::Ordering;

use derive_where::DeriveWhere;

use self::util::{
	AssertClone, AssertCopy, AssertDebug, AssertEq, AssertHash, AssertOrd, AssertPartialEq,
	AssertPartialOrd, Wrapper,
};

#[test]
fn single() {
	#[derive(DeriveWhere)]
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	enum Test<T> {
		#[derive_where(default)]
		A { a: Wrapper<T> },
	}

	let test_1 = Test::A { a: 42.into() };
	let test_2 = Test::A { a: 42.into() };
	let test_le = Test::A { a: 41.into() };
	let test_ge = Test::A { a: 43.into() };

	let _ = AssertClone(&test_1);
	let _ = AssertCopy(&test_1);
	let _ = AssertDebug(&test_1);
	let _ = AssertEq(&test_1);
	let _ = AssertHash(&test_1);
	let _ = AssertOrd(&test_1);
	let _ = AssertPartialEq(&test_1);
	let _ = AssertPartialOrd(&test_1);

	let test_clone = test_1.clone();
	assert!(matches!(test_clone, Test::A { a } if a == 42));

	let test_copy = test_1;
	assert!(matches!(test_copy, Test::A { a } if a == 42));

	assert_eq!(format!("{:?}", test_1), "A { a: 42 }");

	let test_default = Test::<i32>::default();
	assert!(matches!(test_default, Test::A { a } if a == i32::default()));

	util::hash_eq(test_1, test_2);
	util::hash_ne(test_1, test_ge);

	assert!(test_1 == test_2);
	assert!(test_1 != test_ge);

	assert_eq!(test_1.cmp(&test_2), Ordering::Equal);
	assert_eq!(test_1.cmp(&test_le), Ordering::Greater);
	assert_eq!(test_1.cmp(&test_ge), Ordering::Less);

	assert_eq!(test_1.partial_cmp(&test_2), Some(Ordering::Equal));
	assert!(test_1 > test_le);
	assert!(test_1 < test_ge);
}

#[test]
fn multiple() {
	#[derive(DeriveWhere)]
	#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
	enum Test<T> {
		#[derive_where(default)]
		A {
			a: Wrapper<T>,
			b: Wrapper<T>,
			c: Wrapper<T>,
		},
	}

	let test_1 = Test::A {
		a: 42.into(),
		b: 43.into(),
		c: 44.into(),
	};
	let test_2 = Test::A {
		a: 42.into(),
		b: 43.into(),
		c: 44.into(),
	};
	let test_le_1 = Test::A {
		a: 41.into(),
		b: 43.into(),
		c: 44.into(),
	};
	let test_le_2 = Test::A {
		a: 42.into(),
		b: 42.into(),
		c: 44.into(),
	};
	let test_le_3 = Test::A {
		a: 42.into(),
		b: 43.into(),
		c: 43.into(),
	};
	let test_ge_1 = Test::A {
		a: 43.into(),
		b: 43.into(),
		c: 44.into(),
	};
	let test_ge_2 = Test::A {
		a: 42.into(),
		b: 44.into(),
		c: 44.into(),
	};
	let test_ge_3 = Test::A {
		a: 42.into(),
		b: 44.into(),
		c: 45.into(),
	};

	let _ = AssertClone(&test_1);
	let _ = AssertCopy(&test_1);
	let _ = AssertDebug(&test_1);
	let _ = AssertEq(&test_1);
	let _ = AssertHash(&test_1);
	let _ = AssertOrd(&test_1);
	let _ = AssertPartialEq(&test_1);
	let _ = AssertPartialOrd(&test_1);

	let test_clone = test_1.clone();
	assert!(matches!(test_clone, Test::A { a, .. } if a == 42));
	assert!(matches!(test_clone, Test::A { b, .. } if b == 43));
	assert!(matches!(test_clone, Test::A { c, .. } if c == 44));

	let test_copy = test_1;
	assert!(matches!(test_copy, Test::A { a, .. } if a == 42));
	assert!(matches!(test_copy, Test::A { b, .. } if b == 43));
	assert!(matches!(test_copy, Test::A { c, .. } if c == 44));

	assert_eq!(format!("{:?}", test_1), "A { a: 42, b: 43, c: 44 }");

	let test_default = Test::<i32>::default();
	assert!(matches!(test_default, Test::A { a, .. } if a == i32::default()));
	assert!(matches!(test_default, Test::A { b, .. } if b == i32::default()));
	assert!(matches!(test_default, Test::A { c, .. } if c == i32::default()));

	util::hash_eq(test_1, test_2);
	util::hash_ne(test_1, test_ge_1);
	util::hash_ne(test_1, test_ge_2);
	util::hash_ne(test_1, test_ge_3);

	assert!(test_1 == test_2);
	assert!(test_1 != test_ge_1);
	assert!(test_1 != test_ge_2);
	assert!(test_1 != test_ge_3);
	assert!(
		test_1
			!= Test::A {
				a: 45.into(),
				b: 45.into(),
				c: 45.into(),
			}
	);

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
