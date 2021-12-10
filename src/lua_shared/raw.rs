use crate::{
	globals::Lua::{self, GLOBALSINDEX},
	types::*,
};

use once_cell::sync::Lazy;
use super::LUA_SHARED_RAW;

macro_rules! dyn_symbols {
	(
		$(#[$outer:meta])*
		$vis:vis extern $abi:literal fn $name:ident ( $($arg:ident : $argty:ty),* $(,)? ) -> $ret:ty; $($rest:tt)*
	) => {
		$(#[$outer])*
		pub static $name: Lazy<extern $abi fn( $($arg: $argty),* ) -> $ret> = Lazy::new(|| unsafe {
			std::mem::transmute( LUA_SHARED_RAW.get::<extern $abi fn($($argty),*) -> $ret>( stringify!($name).as_bytes() ).unwrap() )
		});
		dyn_symbols!( $($rest)* );
	};

	() => ()
}

macro_rules! lua_macros {
	(
		$(#[$outer:meta])*
		$vis:vis fn $name:ident ( $($arg:ident : $argty:ty),* $(,)? ) -> $ret:ty $body:block; $($rest:tt)*
	) => {
		#[inline(always)]
		$(#[$outer])*
		$vis fn $name( $($arg: $argty),* ) -> $ret $body
		lua_macros!( $($rest)* );
	};
	() => ()
}


// Create Lazy cells that'll find the functions at runtime when called.
dyn_symbols! {
	pub extern "C" fn luaL_loadbufferx(
		L: LuaState,
		code: LuaString,
		size: SizeT,
		id: LuaString,
		mode: LuaString,
	) -> c_int;

	pub extern "C" fn luaL_loadbuffer(
		L: LuaState,
		code: LuaString,
		size: SizeT,
		id: LuaString,
	) -> c_int;

	pub extern "C" fn luaL_loadstring(L: LuaState, code: LuaString) -> c_int;
	pub extern "C" fn luaL_loadfile(L: LuaState, filename: LuaString) -> c_int;
	pub extern "C" fn luaL_loadfilex(L: LuaState, filename: LuaString, mode: LuaString) -> c_int;

	// Call lua code
	pub extern "C" fn lua_pcall(L: LuaState, nargs: c_int, nresults: c_int, msgh: c_int) -> c_int;
	pub extern "C" fn lua_call(L: LuaState, nargs: c_int, nresults: c_int) -> c_int;
	pub extern "C" fn lua_cpcall(L: LuaState, func: LuaCFunction, userdata: *mut c_void) -> c_int;
	pub extern "C" fn luaL_callmeta(L: LuaState, obj: c_int, name: LuaString) -> c_int;

	// Setters
	pub extern "C" fn lua_setfield(L: LuaState, idx: c_int, name: LuaString) -> ();

	pub extern "C" fn lua_setmetatable(L: LuaState, idx: c_int) -> ();
	pub extern "C" fn lua_settop(L: LuaState, ind: c_int) -> ();
	pub extern "C" fn lua_setupvalue(L: LuaState, fidx: c_int, idx: c_int) -> LuaString;
	pub extern "C" fn lua_setfenv(L: LuaState, idx: c_int) -> c_int;
	pub extern "C" fn lua_settable(L: LuaState, idx: c_int) -> ();
	pub extern "C" fn lua_rawset(L: LuaState, idx: c_int) -> (); // lua_settable but no metamethods called
	pub extern "C" fn lua_rawseti(L: LuaState, idx: c_int, n: c_int) -> (); // t[n] = v

	// Getters
	pub extern "C" fn lua_gettable(L: LuaState, idx: c_int) -> ();
	pub extern "C" fn lua_rawget(L: LuaState, idx: c_int) -> (); // lua_gettable but no metamethods called
	pub extern "C" fn lua_rawgeti(L: LuaState, idx: c_int, n: c_int) -> (); // lua_gettable but no metamethods called
	pub extern "C" fn lua_getfenv(L: LuaState, idx: c_int) -> ();

	pub extern "C" fn lua_getfield(L: LuaState, idx: c_int, key: LuaString) -> ();
	pub extern "C" fn lua_getupvalue(L: LuaState, fidx: c_int, idx: c_int) -> LuaString;

	// Non-stack getters
	pub extern "C" fn lua_type(L: LuaState, idx: c_int) -> c_int;
	pub extern "C" fn lua_typename(L: LuaState, typeid: c_int) -> LuaString; // To be used with the return value of lua_type

	// Type conversion getters
	pub extern "C" fn lua_tolstring(L: LuaState, ind: c_int, size: SizeT) -> LuaString;
	pub extern "C" fn lua_toboolean(L: LuaState, idx: c_int) -> c_int;
	pub extern "C" fn lua_tocfunction(L: LuaState, idx: c_int) -> LuaCFunction;
	pub extern "C" fn lua_tointeger(L: LuaState, idx: c_int) -> LuaInteger;
	pub extern "C" fn lua_tonumber(L: LuaState, idx: c_int) -> LuaNumber;
	pub extern "C" fn lua_topointer(L: LuaState, idx: c_int) -> *mut c_void;
	pub extern "C" fn lua_tothread(L: LuaState, idx: c_int) -> LuaState;
	pub extern "C" fn lua_touserdata(L: LuaState, idx: c_int) -> *mut c_void;

	// Push functions
	pub extern "C" fn lua_pushstring(L: LuaState, s: LuaString) -> ();
	pub extern "C" fn lua_pushboolean(L: LuaState, s: c_int) -> ();
	pub extern "C" fn lua_pushlstring(L: LuaState, s: LuaString, sz: SizeT) -> ();
	pub extern "C" fn lua_pushnil(L: LuaState) -> ();
	pub extern "C" fn lua_pushnumber(L: LuaState, num: LuaNumber) -> ();
	pub extern "C" fn lua_pushvalue(L: LuaState, idx: c_int) -> ();
	pub extern "C" fn lua_pushcclosure(L: LuaState, fnc: LuaCFunction, idx: c_int) -> ();
	pub extern "C" fn lua_pushlightuserdata(L: LuaState, p: *mut c_void) -> ();
	pub extern "C" fn lua_pushthread(L: LuaState) -> ();
	//pub extern "C" fn lua_pushfstring(L: LuaState, fmt: LuaString, ...) -> LuaString;
	pub extern "C" fn lua_pushinteger(L: LuaState, n: LuaInteger) -> ();

	// Type checking getters
	/// Same as luaL_checknumber, but casts it to an integer.
	pub extern "C" fn luaL_checkinteger(L: LuaState, narg: c_int) -> LuaInteger;
	/// Checks whether the value at stack index 'narg' is a number and returns this number.
	/// If it is not a lua number, will throw an error to Lua.
	pub extern "C" fn luaL_checknumber(L: LuaState, narg: c_int) -> LuaNumber;
	pub extern "C" fn luaL_checklstring(L: LuaState, narg: c_int, len: SizeT) -> LuaString;

	// Type checking getters that push to stack
	pub extern "C" fn luaL_checkstack(L: LuaState, size: c_int, msg: LuaString) -> ();
	pub extern "C" fn luaL_checkany(L: LuaState, narg: c_int) -> ();
	pub extern "C" fn luaL_checktype(L: LuaState, narg: c_int, typeid: c_int) -> ();
	pub extern "C" fn luaL_checkudata(L: LuaState, narg: c_int, len: SizeT) -> ();

	// Creation
	pub extern "C" fn luaL_newstate() -> LuaState;
	pub extern "C" fn lua_createtable(L: LuaState, narr: c_int, nrec: c_int) -> ();

	// Destruction
	/// Destroys the given lua state.
	/// You *probably* don't want to do this, unless you just want to self destruct the server / your client.
	pub extern "C" fn lua_close(L: LuaState) -> ();

	// JIT
	// Returns 1 for success, 0 for failure
	pub extern "C" fn luaJIT_setmode(L: LuaState, idx: c_int, jit_mode: c_int) -> c_int;
	pub extern "C" fn luaJIT_profile_stop(L: LuaState) -> ();

	pub extern "C" fn luaJIT_profile_start(
		L: LuaState,
		mode: LuaString,
		cb: LuaJITProfileCallback,
		data: *mut c_void,
	) -> ();
	pub extern "C" fn luaJIT_profile_dumpstack(
		L: LuaState,
		fmt: LuaString,
		depth: c_int,
		len: SizeT,
	) -> LuaString;

	// Coroutines
	pub extern "C" fn lua_yield(L: LuaState, nresults: c_int) -> c_int;
	pub extern "C" fn lua_status(L: LuaState) -> c_int;
	/// Starts and resumes a coroutine in a given thread.
	/// Blame garry for the _real
	pub extern "C" fn lua_resume_real(L: LuaState, narg: c_int) -> c_int;

	// Comparison
	pub extern "C" fn lua_equal(L: LuaState, ind1: c_int, ind2: c_int) -> c_int; // Returns 1 or 0 bool
	pub extern "C" fn lua_rawequal(L: LuaState, ind1: c_int, ind2: c_int) -> c_int;

	// Raising Errors
	pub extern "C" fn luaL_typerror(L: LuaState, narg: c_int, typename: LuaString) -> c_int;
	//pub extern "C" fn luaL_error(L: LuaState, fmt: LuaString, ...) -> c_int;
	pub extern "C" fn luaL_argerror(L: LuaState, narg: c_int, extramsg: LuaString) -> c_int;
	pub extern "C" fn lua_error(L: LuaState) -> c_int;

	// Libraries
	/// Opens the standard 'table' library for a lua state
	pub extern "C" fn luaopen_table(L: LuaState) -> c_int;
	/// Opens the standard 'string' library for a lua state
	pub extern "C" fn luaopen_string(L: LuaState) -> c_int;
	/// Opens the standard 'package' library for a lua state
	pub extern "C" fn luaopen_package(L: LuaState) -> c_int;
	/// Opens the standard 'os' library for a lua state
	pub extern "C" fn luaopen_os(L: LuaState) -> c_int;
	/// Opens the standard 'table' library for a lua state
	pub extern "C" fn luaopen_math(L: LuaState) -> c_int;
	/// Opens the standard 'table' library for a lua state
	pub extern "C" fn luaopen_jit(L: LuaState) -> c_int;
	/// Opens the standard 'table' library for a lua state
	pub extern "C" fn luaopen_debug(L: LuaState) -> c_int;
	/// Opens the standard 'table' library for a lua state
	pub extern "C" fn luaopen_bit(L: LuaState) -> c_int;
	/// Opens the standard 'table' library for a lua state
	pub extern "C" fn luaopen_base(L: LuaState) -> c_int;
	/// Opens the standard 'table' library for a lua state
	pub extern "C" fn luaL_openlibs(L: LuaState) -> ();
	/// Internally called by luaL_register, opens given list of LuaRegs with number of functions provided explicitly
	pub extern "C" fn luaL_openlib(L: LuaState, libname: LuaString, l: *const LuaReg, nup: c_int) -> ();

	/// When called with libname as nullptr, it simply registers all functions in the list l reg! into the table on the top of the stack.
	/// # Example
	/// ```rust, ignore
	/// let lib = reg! [
	///     "my_function" => my_function,
	///     "my_other_function" => my_other_function,
	/// ];
	///
	/// luaL_register(L, cstr!("mylib"), lib.as_ptr());
	/// ```
	pub extern "C" fn luaL_register(L: LuaState, libname: LuaString, l: *const LuaReg) -> ();

	// Ref
	pub extern "C" fn luaL_ref(L: LuaState, t: c_int) -> c_int;
	pub extern "C" fn luaL_unref(L: LuaState, t: c_int, r: c_int) -> ();

	// Metatables
	pub extern "C" fn luaL_newmetatable(L: LuaState, tname: LuaString) -> c_int;
	pub extern "C" fn luaL_newmetatable_type(L: LuaState, tname: LuaString, typ: c_int) -> c_int;
	pub extern "C" fn luaL_getmetafield(L: LuaState, obj: c_int, e: LuaString) -> c_int;

	// Optional / Default to ``d``
	pub extern "C" fn luaL_optinteger(L: LuaState, narg: c_int, d: LuaInteger) -> c_int;
	pub extern "C" fn luaL_optlstring(L: LuaState, arg: c_int, d: LuaString, l: SizeT)
		-> LuaString;
	pub extern "C" fn luaL_optnumber(L: LuaState, arg: c_int, d: LuaNumber) -> LuaNumber;

	// x / ref functions
	pub extern "C" fn lua_tointegerx(L: LuaState, index: c_int, isnum: *mut c_int) -> LuaInteger;
	pub extern "C" fn lua_tonumberx(L: LuaState, index: c_int, isnum: *mut c_int) -> LuaNumber;

	// Debug
	pub extern "C" fn luaL_traceback(
		L: LuaState,
		state1: LuaState,
		msg: LuaString,
		level: c_int,
	) -> ();
	pub extern "C" fn luaL_where(L: LuaState, lvl: c_int) -> ();

	// Misc
	pub extern "C" fn luaL_testudata(L: LuaState, arg: c_int, tname: LuaString) -> ();
	pub extern "C" fn luaL_execresult(L: LuaState, stat: c_int) -> c_int;
	pub extern "C" fn luaL_fileresult(L: LuaState, stat: c_int, fname: LuaString) -> c_int;
	pub extern "C" fn luaL_findtable(
		L: LuaState,
		idx: c_int,
		fname: LuaString,
		szhint: c_int,
	) -> LuaString;

	pub extern "C" fn lua_checkstack(L: LuaState, extra: c_int) -> c_int;
	/// Sets the error handler for the lua state.
	pub extern "C" fn lua_atpanic(L: LuaState, panicf: LuaCFunction) -> LuaCFunction;
	pub extern "C" fn lua_gettop(L: LuaState) -> c_int;
	pub extern "C" fn lua_remove(L: LuaState, index: c_int) -> ();

	// luaL_Buffer
	pub extern "C" fn luaL_buffinit(L: LuaState, b: *mut LuaBuffer) -> ();
	pub extern "C" fn luaL_prepbuffer(b: *mut LuaBuffer) -> *mut i8;

	// String methods
	/// Creates a copy of string 's' by replacing any occurrence of the string 'p' with the string 'r'
	/// Pushes the resulting string on the stack and returns it
	pub extern "C" fn luaL_gsub(s: LuaString, pattern: LuaString, replace: LuaString) -> LuaString;
}

// Inline functions to mirror the C macros that come with the lua api
lua_macros! {
	/// Pops n elements from the lua stack.
	pub fn lua_pop(L: LuaState, ind: c_int) -> () {
		lua_settop(L, -(ind) - 1);
	};

	/// Gets a value from _G
	/// Internally calls lua_getfield with [crate::globals::Lua::GLOBALSINDEX]
	pub fn lua_getglobal(L: LuaState, name: LuaString) -> () {
		lua_getfield(L, GLOBALSINDEX, name);
	};

	/// Sets a value in _G
	/// Internally calls lua_setfield with [crate::globals::Lua::GLOBALSINDEX]
	pub fn lua_setglobal(L: LuaState, name: LuaString) -> () {
		lua_setfield(L, GLOBALSINDEX, name);
	};

	/// Pushes a "C" function to the stack
	pub fn lua_pushcfunction(L: LuaState, fnc: LuaCFunction) -> () {
		lua_pushcclosure(L, fnc, 0);
	};

	/// Equivalent to lua_tolstring with len equal to 0
	pub fn lua_tostring(L: LuaState, idx: c_int) -> LuaString {
		lua_tolstring(L, idx, 0)
	};

	/// Starts and resumes a coroutine in a given thread
	pub fn lua_resume(L: LuaState, narg: c_int) -> c_int {
		lua_resume_real(L, narg)
	};

	/// Returns if the value at the given index is a C or Lua function.
	pub fn lua_isfunction(L: LuaState, n: c_int) -> bool {
		lua_type(L, n) == Lua::Type::Function
	};

	/// Returns if the value at the given index is a table.
	pub fn lua_istable(L: LuaState, n: c_int) -> bool {
		lua_type(L, n) == Lua::Type::Table
	};

	pub fn lua_islightuserdata(L: LuaState, n: c_int) -> bool {
		lua_type(L, n) == Lua::Type::LUserData
	};

	/// Returns if the value at the given index is nil.
	/// You might want to use [lua_isnoneornil] instead.
	pub fn lua_isnil(L: LuaState, n: c_int) -> bool {
		lua_type(L, n) == Lua::Type::Nil
	};

	/// Returns if the value at the given index is a boolean.
	pub fn lua_isboolean(L: LuaState, n: c_int) -> bool {
		lua_type(L, n) == Lua::Type::Bool
	};

	/// Returns if the value at the given index is a thread.
	pub fn lua_isthread(L: LuaState, n: c_int) -> bool {
		lua_type(L, n) == Lua::Type::Thread
	};

	/// Returns if the value at the given index is none (element outside of stack / invalid)
	pub fn lua_isnone(L: LuaState, n: c_int) -> bool {
		lua_type(L, n) == Lua::Type::None
	};

	/// Returns if the value at the given index is none (invalid) or nil.
	pub fn lua_isnoneornil(L: LuaState, n: c_int) -> bool {
		lua_type(L, n) <= 0
	};

	/// Loads and pcalls a string of lua code
	/// Returns if the code was successfully executed
	/// Error will be left on the stack if the code failed to execute
	pub fn luaL_dostring(L: LuaState, str: LuaString) -> bool {
		luaL_loadstring(L, str) == 0 || lua_pcall(L, 0, Lua::MULTRET, 0) == 0
	};

	/// Loads and pcalls a file's lua code
	/// Returns if the code was successfully executed
	/// Error will be left on the stack if the code failed to execute
	pub fn luaL_dofile(L: LuaState, filename: LuaString) -> bool {
		luaL_loadfile(L, filename) == 0 || lua_pcall(L, 0, Lua::MULTRET, 0) == 0
	};

	/// Returns value at [crate::globals::Lua::REGISTRYINDEX] with name 'name'
	pub fn luaL_getmetatable(L: LuaState, name: LuaString) -> () {
		lua_getfield(L, Lua::REGISTRYINDEX, name);
	};

	/// If a condition is false, throws an argument error at numarg
	pub fn luaL_argcheck(L: LuaState, cond: bool, numarg: c_int, extramsg: LuaString) -> () {
		if !cond {
			luaL_argerror(L, numarg, extramsg);
		}
	};

	/// Returns the type name of object at index i
	pub fn luaL_typename(L: LuaState, i: c_int) -> LuaString {
		lua_typename(L, lua_type(L, i))
	};
}