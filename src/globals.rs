use crate::types::*;

pub mod Lua {
	use super::CInt;

	pub static VERSION: &'static str = "Lua 5.1";
	pub static RELEASE: &'static str = "Lua 5.1.4";
	pub static VERSION_NUM: CInt = 501;
	pub static COPYRIGHT: &'static str = "Copyright (C) 1994-2008 Lua.org, PUC-Rio";
	pub static AUTHORS: &'static str = "R. Ierusalimschy, L. H. de Figueiredo & W. Celes";

	pub static REGISTRYINDEX: CInt = -10000;
	pub static ENVIRONINDEX: CInt = -10001;
	pub static GLOBALSINDEX: CInt = -10002;

	pub static MULTRET: CInt = -1;
	pub static SIGNATURE: &'static str = "\x1bLua";
	pub static MINSTACK: CInt = 20;

	pub static NUMTYPES: CInt = 9;
	pub static NUMTAGS: CInt = NUMTYPES;


	// Proper enums to use. Cast these to integers when using them
	pub enum Type {
		None = -1,
		Nil,
		Bool,
		LUserData,
		Number,
		String,
		Table,
		Function,
		UserData,
		Thread
	}

	pub enum Status {
		Ok = 0,
		Yield,
		ErrRun,
		ErrSyntax,
		ErrMem,
		ErrErr
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
		Inc // 11
	}

	// To be used with debug.sethook
	pub enum Hook {
		Call = 0,
		Ret,
		Line,
		Count,
		TailCall
	}

	pub enum Mask {
		Call = (1 << Hook::Call as i32),
		Ret = (1 << Hook::Ret as i32),
		Line = (1 << Hook::Line as i32),
		Count = (1 << Hook::Count as i32)
	}
}

pub mod Jit {
	pub enum Mode {
		ENGINE,
		DEBUG,
		FUNC,
		ALLFUNC,
		ALLSUBFUNC,
		TRACE,
		WRAPCFUNC = 0x10,
		MAX,
		MASK = 0x0ff // LUAJIT_MODE_MASK
	}

	use super::CInt;
	// Associated Constants, woah
	impl Mode {
		pub const OFF: CInt = 0x0000;
		pub const ON: CInt = 0x0100;
		pub const FLUSH: CInt = 0x0200;
	}
}