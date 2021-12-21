use crate::types::*;

/// Index of the lua registry. What you'd get from debug.getregistry()
pub const REGISTRYINDEX: c_int = -10000;
/// Index of the lua environment.
/// This is like getfenv() or _ENV in later lua versions
pub const ENVIRONINDEX: c_int = -10001;
/// Index of _G
pub const GLOBALSINDEX: c_int = -10002;

/// Number of returns to use in functions like lua_pcall to represent 0 or more.
pub const MULTRET: c_int = -1;

pub const NUMTYPES: c_int = 9;
pub const NUMTAGS: c_int = NUMTYPES;

pub const TNONE: c_int = -1;
pub const TNIL: c_int = 0;
pub const TBOOLEAN: c_int = 1;
pub const TLIGHTUSERDATA: c_int = 2;
pub const TNUMBER: c_int = 3;
pub const TSTRING: c_int = 4;
pub const TTABLE: c_int = 5;
pub const TFUNCTION: c_int = 6;
pub const TUSERDATA: c_int = 7;
pub const TTHREAD: c_int = 8;

pub const MINSTACK: c_int = 20;

pub const OK: c_int = 0;
pub const YIELD: c_int = 1;
pub const ERRRUN: c_int = 2;
pub const ERRSYNTAX: c_int = 3;
pub const ERRMEM: c_int = 4;
pub const ERRERR: c_int = 5;

pub const GCSTOP: c_int = 0;
pub const GCRESTART: c_int = 1;
pub const GCCOLLECT: c_int = 2;
pub const GCCOUNT: c_int = 3;
pub const GCCOUNTB: c_int = 4;
pub const GCSTEP: c_int = 5;
pub const GCSETPAUSE: c_int = 6;
pub const GCSETSTEPMUL: c_int = 7;

pub const HOOKCALL: c_int = 0;
pub const HOOKRET: c_int = 1;
pub const HOOKLINE: c_int = 2;
pub const HOOKCOUNT: c_int = 3;
pub const HOOKTAILRET: c_int = 4;

pub const MASKCALL: c_int = 1 << HOOKCALL;
pub const MASKRET: c_int = 1 << HOOKRET;
pub const MASKLINE: c_int = 1 << HOOKLINE;
pub const MASKCOUNT: c_int = 1 << HOOKCOUNT;

///  Size of LuaDebug.short_src
pub const IDSIZE: usize = 128;

// This is libc's default so we'll roll with it
pub const BUFFERSIZE: usize = 8192;

// Rust doesn't work well with  C Enums. So I'm just going to ditch the idea.
#[deprecated(since = "0.9.1", note = "Use rglua::lua::T* instead")]
#[allow(non_snake_case)]
pub mod Type {
	#![allow(non_upper_case_globals)]

	pub const None: i32 = -1;
	pub const Nil: i32 = 0;
	pub const Bool: i32 = 1;
	pub const LUserdata: i32 = 2;
	pub const Number: i32 = 3;
	pub const String: i32 = 4;
	pub const Table: i32 = 5;
	pub const Function: i32 = 6;
	pub const Userdata: i32 = 7;
	pub const Thread: i32 = 8;
}

#[repr(i32)]
#[deprecated(since = "0.9.1", note = "Use rglua::lua::T* instead")]
pub enum Status {
	Ok = 0,
	Yield,
	ErrRun,
	ErrSyntax,
	ErrMem,
	ErrErr,
}

// Garbage collection
#[repr(i32)]
#[deprecated(since = "0.9.1", note = "Use rglua::lua::T* instead")]
pub enum Gc {
	Stop = 0,
	Restart,
	Collect,
	Count,
	CountB,
	Step,
	SetPause,
	SetStepMul,
	IsRunning,
	Gen,
	Inc, // 11
}

// To be used with debug.sethook
#[deprecated(since = "0.9.1", note = "Use rglua::lua::T* instead")]
pub enum Hook {
	Call = 0,
	Ret,
	Line,
	Count,
	TailCall,
}

#[deprecated(since = "0.9.1", note = "Use rglua::lua::T* instead")]
pub enum Mask {
	#[allow(deprecated)]
	Call = (1 << Hook::Call as i32),
	#[allow(deprecated)]
	Ret = (1 << Hook::Ret as i32),
	#[allow(deprecated)]
	Line = (1 << Hook::Line as i32),
	#[allow(deprecated)]
	Count = (1 << Hook::Count as i32),
}

pub mod jit {
	use super::c_int;

	pub const VERSION: &str = "LuaJIT 2.0.4";
	pub const VERSION_NUM: c_int = 20004; /* Version 2.0.4 = 02.00.04. */

	pub const MODE_MASK: c_int = 0x00ff;

	pub const MODE_ENGINE: c_int = 1; /* Set mode for whole JIT engine. */
	pub const MODE_DEBUG: c_int = 2; /* Set debug mode (idx = level). */
	pub const MODE_FUNC: c_int = 3; /* Change mode for a function. */
	pub const MODE_ALLFUNC: c_int = 4; /* Recurse into subroutine protos. */
	pub const MODE_ALLSUBFUNC: c_int = 5; /* Change only the subroutines. */
	pub const MODE_TRACE: c_int = 6; /* Flush a compiled trace. */
	pub const MODE_WRAPCFUNC: c_int = 0x10; /* Set wrapper mode for C function calls. */
	pub const MODE_MAX: c_int = MODE_WRAPCFUNC + 1;

	pub const MODE_OFF: c_int = 0x0000; /* Turn feature off. */
	pub const MODE_ON: c_int = 0x0100; /* Turn feature on. */
	pub const MODE_FLUSH: c_int = 0x0200; /* Flush JIT-compiled code. */

	#[repr(i32)]
	#[deprecated(since = "0.9.1", note = "Use rglua::lua::T* instead")]
	pub enum Mode {
		ENGINE,
		DEBUG,
		FUNC,
		ALLFUNC,
		ALLSUBFUNC,
		TRACE,
		WRAPCFUNC = 0x10,
		MAX,
		MASK = 0x0ff, // LUAJIT_MODE_MASK
	}

	#[deprecated(since = "0.9.1", note = "Use rglua::lua::T* instead")]
	#[allow(deprecated)]
	// Associated Constants, woah
	impl Mode {
		pub const OFF: c_int = 0x0000;
		pub const ON: c_int = 0x0100;
		pub const FLUSH: c_int = 0x0200;
	}
}
