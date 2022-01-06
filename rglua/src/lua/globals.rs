use crate::lua::types::*;

/// Index of the lua registry.  
/// What you'd get from debug.getregistry()
pub const REGISTRYINDEX: c_int = -10000;

/// Index of the lua environment.  
/// This is like ``getfenv()`` or ``_ENV`` in later lua versions
pub const ENVIRONINDEX: c_int = -10001;

/// Index of _G
pub const GLOBALSINDEX: c_int = -10002;

/// Number of returns to use in functions like lua_pcall to represent 0 or more.
pub const MULTRET: c_int = -1;

/// Number of primitive lua types (excluding garrysmod userdata types)
pub const NUMTYPES: c_int = 9;
pub const NUMTAGS: c_int = NUMTYPES;

/// 'None' / 'No value' type.
pub const TNONE: c_int = -1;

/// 'nil' type
pub const TNIL: c_int = 0;

/// Boolean type
pub const TBOOLEAN: c_int = 1;

/// 'Light' Userdata type.
/// This is just a pointer to something owned by C without a custom metatable, as a [TUSERDATA] may have.
pub const TLIGHTUSERDATA: c_int = 2;

/// Number type.  
/// This is a double, or [f64]
pub const TNUMBER: c_int = 3;

/// String type, this is a [LuaString]
pub const TSTRING: c_int = 4;

/// Table type created by [lua_newtable](super::lua_newtable)
pub const TTABLE: c_int = 5;

/// Function type created by [lua_pushcfunction](super::lua_pushcfunction), [lua_pushcfunction](super::lua_pushcclosure) or retrieved from lua.
pub const TFUNCTION: c_int = 6;

/// 'Heavy' Userdata type managed by lua.  
/// Created by [lua_newuserdata](super::lua_newuserdata)
pub const TUSERDATA: c_int = 7;

/// Thread / Coroutine type, created by [lua_newthread](super::lua_newthread)
pub const TTHREAD: c_int = 8;

/// Minimum number of stack levels guaranteed to C whenever it is called into by lua.
pub const MINSTACK: c_int = 20;

/// OK status code used by several functions like [lua_status](super::lua_status), [lua_pcall](super::lua_pcall)
pub const OK: c_int = 0;

/// YIELD status code used by [lua_status](super::lua_status)
pub const YIELD: c_int = 1;

/// Runtime error, code used by functions like [lua_pcall](super::lua_pcall)
pub const ERRRUN: c_int = 2;

/// Syntax error, code used by functions like [lua_load](super::lua_load)
pub const ERRSYNTAX: c_int = 3;

/// Memory allocation, error code used by many functions like [super::lua_load]
pub const ERRMEM: c_int = 4;

/// Error when running the  error handler, code used by functions like [lua_pcall](super::lua_pcall)
pub const ERRERR: c_int = 5;

/// Enum used with [lua_gc](super::lua_gc) - Stops the garbage collector.
pub const GCSTOP: c_int = 0;

/// Enum used with [lua_gc](super::lua_gc) - Restarts the garbage collector
pub const GCRESTART: c_int = 1;

/// Enum used with [lua_gc](super::lua_gc) - Restarts the garbage collector
pub const GCCOLLECT: c_int = 2;

/// Enum used with [lua_gc](super::lua_gc) - Returns the total number of live Lua objects in the current Lua state
pub const GCCOUNT: c_int = 3;

/// Enum used with [lua_gc](super::lua_gc) - Returns the total number of live Lua objects in the current Lua state, plus the total number of Lua objects in unreachable threads
pub const GCCOUNTB: c_int = 4;

/// Enum used with [lua_gc](super::lua_gc) - Performs a single step of the garbage collector.
pub const GCSTEP: c_int = 5;

/// Enum used with [lua_gc](super::lua_gc) - Sets `lua_gc`'s pause threshold.
pub const GCSETPAUSE: c_int = 6;

/// Enum used with [lua_gc](super::lua_gc) - Sets `lua_gc`'s step multiplier.
pub const GCSETSTEPMUL: c_int = 7;

pub const HOOKCALL: c_int = 0;
pub const HOOKRET: c_int = 1;
pub const HOOKLINE: c_int = 2;
pub const HOOKCOUNT: c_int = 3;
pub const HOOKTAILRET: c_int = 4;

/// Enum used by [lua_sethook](super::lua_sethook)
pub const MASKCALL: c_int = 1 << HOOKCALL;
/// Enum used by [lua_sethook](super::lua_sethook)
pub const MASKRET: c_int = 1 << HOOKRET;
/// Enum used by [lua_sethook](super::lua_sethook)
pub const MASKLINE: c_int = 1 << HOOKLINE;
/// Enum used by [lua_sethook](super::lua_sethook)
pub const MASKCOUNT: c_int = 1 << HOOKCOUNT;

/// Size of [LuaDebug].short_src
pub const IDSIZE: usize = 128;

/// This is libc's default so we'll roll with it
/// Used internally for [LuaBuffer].
pub const BUFFERSIZE: usize = 8192;

pub const NOREF: c_int = -2;
pub const REFNIL: c_int = -1;

/// LuaJIT specific global constants
pub mod jit {
	use super::c_int;

	/// Version of LuaJIT that garrysmod uses
	pub const VERSION: &str = "LuaJIT 2.0.4";
	/// Semver number
	pub const VERSION_NUM: c_int = 20004; /* Version 2.0.4 = 02.00.04. */

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode)
	pub const MODE_MASK: c_int = 0x00ff;

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Set mode for the whole JIT engine
	pub const MODE_ENGINE: c_int = 1;

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Set debug mode (idx = level).
	pub const MODE_DEBUG: c_int = 2;

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Change mode for a function.
	pub const MODE_FUNC: c_int = 3;

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Recurse into subroutine protos.
	pub const MODE_ALLFUNC: c_int = 4;
	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Change only the subroutines.
	pub const MODE_ALLSUBFUNC: c_int = 5;

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Flush a compiled trace.
	pub const MODE_TRACE: c_int = 6;

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Set wrapper mode for C function calls.
	pub const MODE_WRAPCFUNC: c_int = 0x10;

	pub const MODE_MAX: c_int = MODE_WRAPCFUNC + 1;

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Turn a feature off
	pub const MODE_OFF: c_int = 0x0000;

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Turn a feature on
	pub const MODE_ON: c_int = 0x0100;

	/// Enum used by [luaJIT_setmode](crate::lua::luaJIT_setmode) -- Flush JIT compiled code
	pub const MODE_FLUSH: c_int = 0x0200;
}
