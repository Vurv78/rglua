pub(crate) use super::prelude::{self, *};

#[vtable]
pub struct LuaObject {
	pub Set: extern "C" fn(obj: *mut LuaObject),
	pub SetFromStack: extern "C" fn(i: c_int),
	pub Unreference: extern "C" fn(),
	pub GetType: extern "C" fn() -> c_int,
	pub GetString: extern "C" fn() -> *const c_char,
	pub GetFloat: extern "C" fn() -> c_float,
	pub GetInt: extern "C" fn() -> c_int,
	pub GetUserdata: extern "C" fn() -> *mut c_void,

	pub SetMember: extern "C" fn(name: *const c_char),
	pub SetMemberObj: extern "C" fn(name: *const c_char, obj: *mut LuaObject),
	pub SetMemberFloat: extern "C" fn(name: *const c_char, f: c_float),
	pub SetMemberBool: extern "C" fn(name: *const c_char, b: bool),
	pub SetMemberStr: extern "C" fn(name: *const c_char, s: *const c_char),
	//pub SetMemberFunc: extern "C" fn(name: *const c_char, func: crate::types::LuaCFunction),
	#[offset(14)]
	pub GetMemberBool: extern "C" fn(name: *const c_char, default: bool) -> bool,
	pub GetMemberInt: extern "C" fn(name: *const c_char, default: c_int) -> c_int,
	pub GetMemberFloat: extern "C" fn(name: *const c_char, default: c_float) -> c_float,
	pub GetMemberStr: extern "C" fn(name: *const c_char, default: *const c_char) -> *const c_char,
	pub GetMemberUserdata: extern "C" fn(name: *const c_char, default: *mut c_void) -> *mut c_void,
	pub GetMemberUserdataNum: extern "C" fn(ind: c_float, default: *mut c_void) -> *mut c_void,

	pub SetMetatable: extern "C" fn(mt: *mut LuaObject),
	pub SetUserdata: extern "C" fn(ud: *mut c_void),

	pub Push: extern "C" fn(),
	pub isNil: extern "C" fn() -> bool,
	pub isTable: extern "C" fn() -> bool,
	pub isString: extern "C" fn() -> bool,
	pub isNumber: extern "C" fn() -> bool,
	pub isFunction: extern "C" fn() -> bool,
	pub isUserdata: extern "C" fn() -> bool,

	pub GetMemberF: extern "C" fn(key: c_float) -> *mut LuaObject
}

mod base;
mod interface;
mod shared;

pub use base::LuaBase;
pub use interface::LuaInterface;
pub use shared::LuaShared;
