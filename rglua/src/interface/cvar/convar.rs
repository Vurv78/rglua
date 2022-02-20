use super::icvar::ConCommandBase;
use super::prelude::*;

// type ChangeCallback = extern "C" fn(var: *mut IConVar, old: *const c_char, fl_old: c_float);

#[vtable]
pub struct CVar {
	pub base: ConCommandBase,

	pub parent: *mut CVar,
	pub default_value: *const c_char
	/*value: *mut c_char,
	len: c_int,

	has_min: bool,
	min_value: c_float,
	has_max: bool,
	max_value: c_float,

	callback: ChangeCallback*/
}

impl std::fmt::Debug for CVar {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(
			f,
			"CVar {{ base: {:?}, parent: {:?}, default_value: {:?} }}",
			self.base, self.parent, self.default_value
		)
	}
}
