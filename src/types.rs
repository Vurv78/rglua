pub type CVoid = std::ffi::c_void;
pub type SizeT = usize;

pub type LuaNumber = f64; // All lua numbers are doubles in Lua 5.1 (Glua)
pub type LuaInteger = isize;

pub type LuaState = *mut CVoid; // Raw Lua state.
pub type CharBuf = *const i8; // const char*
pub type CInt = i32;
pub type LuaCFunction = extern "C" fn(LuaState) -> CInt;
pub type CLong = i64;