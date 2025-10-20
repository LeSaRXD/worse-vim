use super::set_line_width;

fn test_str(input: &str, expected: &str, width: usize) {
	assert_eq!(set_line_width(input, width), expected);
}

#[test]
fn less_length() {
	test_str("abcd", "abcd  ", 6);
}

#[test]
fn exact_length() {
	test_str("abcd", "abcd", 4);
}

#[test]
fn greater_length() {
	test_str("abcdef", "abcd", 4);
}

#[test]
fn unicode() {
	let line = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
	test_str(line, "Ｈｅ ", 5);
	test_str(line, "Ｈｅｌｌｏ, ｗｏｒｌｄ!  ", 25);
}
