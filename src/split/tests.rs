use super::divide_into_intervals;

#[test]
fn even_splits() {
	let splits: Vec<_> = divide_into_intervals(20, 2).collect();
	assert_eq!(&splits, &[10, 10]);
}

#[test]
fn uneven_splits() {
	let splits: Vec<_> = divide_into_intervals(20, 3).collect();
	assert_eq!(&splits, &[7, 7, 6]);
}
