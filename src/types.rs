
pub type CVoid = std::ffi::c_void;
pub type SizeT = usize;
pub type LuaNumber = f64; // All lua numbers are doubles in Lua 5.1 (Glua)
pub type LuaState = *mut CVoid; // Raw Lua state.
pub type CharBuf = *const i8; // const char*
pub type CInt = i32;
pub type LuaCFunction = extern "C" fn(LuaState) -> CInt;

pub mod luatypes {
    use super::CInt;
    pub static NONE: CInt = -1;
    pub static NIL: CInt = 0;
    pub static BOOL: CInt = 1;
    pub static LUSERDATA: CInt = 2;
    pub static NUMBER: CInt = 3;
    pub static STRING: CInt = 4;
    pub static TABLE: CInt = 5;
    pub static FUNCTION: CInt = 6;
    pub static USERDATA: CInt = 7;
    pub static THREAD: CInt = 8;
}