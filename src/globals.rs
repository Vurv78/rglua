use crate::types::*;

pub mod Lua {
	use super::c_int;

	pub static REGISTRYINDEX: c_int = -10000;
	pub static ENVIRONINDEX: c_int = -10001;
	pub static GLOBALSINDEX: c_int = -10002;

	pub static MULTRET: c_int = -1;

	pub static NUMTYPES: c_int = 9;
	pub static NUMTAGS: c_int = NUMTYPES;

	pub mod Type {
		pub const None: i32 = -1;
		pub const Nil: i32 = 0;
		pub const Bool: i32 = 1;
		pub const LUserData: i32 = 2;
		pub const Number: i32 = 3;
		pub const String: i32 = 4;
		pub const Table: i32 = 5;
		pub const Function: i32 = 6;
		pub const UserData: i32 = 7;
		pub const Thread: i32 = 8;
	}

	#[repr(i32)]
	pub enum Status {
		Ok = 0,
		Yield,
		ErrRun,
		ErrSyntax,
		ErrMem,
		ErrErr,
	}

	// Garbage collection
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
	pub enum Hook {
		Call = 0,
		Ret,
		Line,
		Count,
		TailCall,
	}

	pub enum Mask {
		Call = (1 << Hook::Call as i32),
		Ret = (1 << Hook::Ret as i32),
		Line = (1 << Hook::Line as i32),
		Count = (1 << Hook::Count as i32),
	}
}

pub mod Jit {
	#[repr(i32)]
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

	use super::c_int;
	// Associated Constants, woah
	impl Mode {
		pub const OFF: c_int = 0x0000;
		pub const ON: c_int = 0x0100;
		pub const FLUSH: c_int = 0x0200;
	}
}
