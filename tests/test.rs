#[test]
fn cstr_test() {
	use rglua::cstr;
	let a_ptr = cstr!("Hello world!");
	unsafe {
		assert_eq!( *a_ptr, *(b"Hello world!\0".as_ptr() as *const i8) );
		let a_str = std::ffi::CStr::from_ptr(a_ptr);

		assert_eq!( a_str.to_str(), Ok("Hello world!") );
	}
}

#[test]
fn rstr_test() {
	use rglua::{rstr, cstr};

	let a = cstr!("How are you?");

	let b = rstr!(a);
	assert_eq!(b, "How are you?");
}