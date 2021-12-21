use crate::{
	lua::{self, GLOBALSINDEX},
	types::*,
};

use super::LUA_SHARED_RAW;
use once_cell::sync::Lazy;

macro_rules! dyn_symbols {
	(
		$(#[$outer:meta])*
		$vis:vis extern $abi:literal fn $name:ident ( $($arg:ident : $argty:ty),* $(,)? ) -> $ret:ty; $($rest:tt)*
	) => {
		$(#[$outer])*
		#[allow(non_upper_case_globals)]
		pub static $name: Lazy<extern $abi fn( $($arg: $argty),* ) -> $ret> = Lazy::new(|| unsafe {
			std::mem::transmute( LUA_SHARED_RAW.get::<extern $abi fn($($argty),*) -> $ret>( stringify!($name).as_bytes() ).unwrap() )
		});
		dyn_symbols!( $($rest)* );
	};

	(
		$(#[$outer:meta])*
		$vis:vis extern $abi:literal fn $name:ident ( $($arg:ident : $argty:ty),+ , ... ) -> $ret:ty; $($rest:tt)*
	) => {
		$(#[$outer])*
		#[allow(non_upper_case_globals)]
		pub static $name: Lazy<extern $abi fn( $($arg: $argty),+ , ... ) -> $ret> = Lazy::new(|| unsafe {
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
		#[allow(non_snake_case)]
		$(#[$outer])*
		$vis fn $name( $($arg: $argty),* ) -> $ret $body
		lua_macros!( $($rest)* );
	};
	() => ()
}

// Create Lazy cells that'll find the functions at runtime when called.
// Special thanks to https://pgl.yoyo.org/luai/i/about for excellent examples and descriptions of the lua c api that could be integrated here.
// (Were tweaked to be more concise and fit with rglua of course.)
dyn_symbols! {
	pub extern "C" fn luaL_loadbufferx(
		l: LuaState,
		code: LuaString,
		size: SizeT,
		id: LuaString,
		mode: LuaString,
	) -> c_int;

	pub extern "C" fn luaL_loadbuffer(
		l: LuaState,
		code: LuaString,
		size: SizeT,
		id: LuaString,
	) -> c_int;

	pub extern "C" fn luaL_loadstring(l: LuaState, code: LuaString) -> c_int;
	pub extern "C" fn luaL_loadfile(l: LuaState, filename: LuaString) -> c_int;
	pub extern "C" fn luaL_loadfilex(l: LuaState, filename: LuaString, mode: LuaString) -> c_int;

	// Call lua code
	pub extern "C" fn lua_pcall(l: LuaState, nargs: c_int, nresults: c_int, msgh: c_int) -> c_int;
	pub extern "C" fn lua_call(l: LuaState, nargs: c_int, nresults: c_int) -> c_int;
	pub extern "C" fn lua_cpcall(l: LuaState, func: LuaCFunction, userdata: *mut c_void) -> c_int;
	pub extern "C" fn luaL_callmeta(l: LuaState, obj: c_int, name: LuaString) -> c_int;

	// Setters
	pub extern "C" fn lua_setfield(l: LuaState, idx: c_int, name: LuaString) -> ();

	pub extern "C" fn lua_setmetatable(l: LuaState, idx: c_int) -> ();
	pub extern "C" fn lua_settop(l: LuaState, ind: c_int) -> ();
	pub extern "C" fn lua_setfenv(l: LuaState, idx: c_int) -> c_int;
	pub extern "C" fn lua_settable(l: LuaState, idx: c_int) -> ();
	pub extern "C" fn lua_rawset(l: LuaState, idx: c_int) -> (); // lua_settable but no metamethods called
	pub extern "C" fn lua_rawseti(l: LuaState, idx: c_int, n: c_int) -> (); // t[n] = v

	// Getters
	pub extern "C" fn lua_gettable(l: LuaState, idx: c_int) -> ();
	pub extern "C" fn lua_rawget(l: LuaState, idx: c_int) -> (); // lua_gettable but no metamethods called
	pub extern "C" fn lua_rawgeti(l: LuaState, idx: c_int, n: c_int) -> (); // lua_gettable but no metamethods called
	pub extern "C" fn lua_getfenv(l: LuaState, idx: c_int) -> ();

	pub extern "C" fn lua_getfield(l: LuaState, idx: c_int, key: LuaString) -> ();

	// Non-stack getters
	pub extern "C" fn lua_type(l: LuaState, idx: c_int) -> c_int;
	pub extern "C" fn lua_typename(l: LuaState, typeid: c_int) -> LuaString; // To be used with the return value of lua_type

	// Type conversion getters
	pub extern "C" fn lua_tolstring(l: LuaState, ind: c_int, size: SizeT) -> LuaString;
	pub extern "C" fn lua_toboolean(l: LuaState, idx: c_int) -> c_int;
	pub extern "C" fn lua_tocfunction(l: LuaState, idx: c_int) -> LuaCFunction;
	pub extern "C" fn lua_tointeger(l: LuaState, idx: c_int) -> LuaInteger;
	pub extern "C" fn lua_tonumber(l: LuaState, idx: c_int) -> LuaNumber;
	pub extern "C" fn lua_topointer(l: LuaState, idx: c_int) -> *mut c_void;
	pub extern "C" fn lua_tothread(l: LuaState, idx: c_int) -> LuaState;
	pub extern "C" fn lua_touserdata(l: LuaState, idx: c_int) -> *mut c_void;

	// Push functions
	pub extern "C" fn lua_pushstring(l: LuaState, s: LuaString) -> ();
	pub extern "C" fn lua_pushboolean(l: LuaState, s: c_int) -> ();
	pub extern "C" fn lua_pushlstring(l: LuaState, s: LuaString, sz: SizeT) -> ();
	pub extern "C" fn lua_pushnil(l: LuaState) -> ();
	pub extern "C" fn lua_pushnumber(l: LuaState, num: LuaNumber) -> ();
	pub extern "C" fn lua_pushvalue(l: LuaState, idx: c_int) -> ();
	/// Pushes a c function on the stack with associated values.
	/// # Parameters
	/// * `l` - LuaState
	/// * `f` - Lua function
	/// * `n` - Number of upvalues to associate and pull from stack with the function
	pub extern "C" fn lua_pushcclosure(l: LuaState, fnc: LuaCFunction, nargs: c_int) -> ();
	pub extern "C" fn lua_pushlightuserdata(l: LuaState, p: *mut c_void) -> ();
	/// Pushes a given thread (representing ``l``) to the stack.
	/// # Parameters
	/// * `l` - The thread to push.
	/// # Returns
	/// 1 if the thread is the main thread of the state.
	pub extern "C" fn lua_pushthread(l: LuaState) -> c_int;
	/// Pushes a formatted LuaString to the stack
	pub extern "C" fn lua_pushfstring(l: LuaState, fmt: LuaString, ...) -> LuaString;
	pub extern "C" fn lua_pushinteger(l: LuaState, n: LuaInteger) -> ();

	// Type checking getters
	/// Same as luaL_checknumber, but casts it to an integer.
	pub extern "C" fn luaL_checkinteger(l: LuaState, narg: c_int) -> LuaInteger;
	/// Checks whether the value at stack index 'narg' is a number and returns this number.
	/// If it is not a lua number, will throw an error to Lua.
	pub extern "C" fn luaL_checknumber(l: LuaState, narg: c_int) -> LuaNumber;
	pub extern "C" fn luaL_checklstring(l: LuaState, narg: c_int, len: SizeT) -> LuaString;

	// Type checking getters that push to stack
	pub extern "C" fn luaL_checkstack(l: LuaState, size: c_int, msg: LuaString) -> ();
	pub extern "C" fn luaL_checkany(l: LuaState, narg: c_int) -> ();
	pub extern "C" fn luaL_checktype(l: LuaState, narg: c_int, typeid: c_int) -> ();
	pub extern "C" fn luaL_checkudata(l: LuaState, narg: c_int, len: SizeT) -> ();

	// Creation
	pub extern "C" fn luaL_newstate() -> LuaState;
	pub extern "C" fn lua_newstate(f: LuaAlloc, ud: *mut c_void) -> c_int;
	pub extern "C" fn lua_createtable(l: LuaState, narr: c_int, nrec: c_int) -> ();

	// Destruction
	/// Destroys the given lua state.
	/// You *probably* don't want to do this, unless you just want to self destruct the server / your client.
	pub extern "C" fn lua_close(l: LuaState) -> ();

	// JIT
	// Returns 1 for success, 0 for failure
	pub extern "C" fn luaJIT_setmode(l: LuaState, idx: c_int, jit_mode: c_int) -> c_int;
	pub extern "C" fn luaJIT_profile_stop(l: LuaState) -> ();

	pub extern "C" fn luaJIT_profile_start(
		l: LuaState,
		mode: LuaString,
		cb: LuaJITProfileCallback,
		data: *mut c_void,
	) -> ();
	pub extern "C" fn luaJIT_profile_dumpstack(
		l: LuaState,
		fmt: LuaString,
		depth: c_int,
		len: SizeT,
	) -> LuaString;

	// Coroutines
	pub extern "C" fn lua_yield(l: LuaState, nresults: c_int) -> c_int;
	pub extern "C" fn lua_status(l: LuaState) -> c_int;
	/// Starts and resumes a coroutine in a given thread.
	/// Blame garry for the _real
	pub extern "C" fn lua_resume_real(l: LuaState, narg: c_int) -> c_int;

	// Comparison
	pub extern "C" fn lua_equal(l: LuaState, ind1: c_int, ind2: c_int) -> c_int; // Returns 1 or 0 bool
	pub extern "C" fn lua_rawequal(l: LuaState, ind1: c_int, ind2: c_int) -> c_int;

	// Raising Errors
	/// Generates an error with a message like the following:
	/// ```text
	/// location: bad argument narg to 'func' (tname expected, got rt)
	/// ```
	/// where location is produced by luaL_where, func is the name of the current function, and rt is the type name of the actual argument.
	pub extern "C" fn luaL_typerror(l: LuaState, narg: c_int, typename: LuaString) -> !;
	pub extern "C" fn luaL_error(l: LuaState, fmt: LuaString, ...) -> !;
	/// Raises an error with the following message, where func is retrieved from the call stack:
	/// ```text
	/// bad argument #<narg> to <func> (<extramsg>)
	/// ```
	/// This function never returns
	pub extern "C" fn luaL_argerror(l: LuaState, narg: c_int, extramsg: LuaString) -> !;
	pub extern "C" fn lua_error(l: LuaState) -> !;

	// Libraries
	/// Opens the standard 'table' library for a lua state
	pub extern "C" fn luaopen_table(l: LuaState) -> c_int;
	/// Opens the standard 'string' library for a lua state
	pub extern "C" fn luaopen_string(l: LuaState) -> c_int;
	/// Opens the standard 'package' library for a lua state
	pub extern "C" fn luaopen_package(l: LuaState) -> c_int;
	/// Opens the standard 'os' library for a lua state
	pub extern "C" fn luaopen_os(l: LuaState) -> c_int;
	/// Opens the standard 'math' library for a lua state
	pub extern "C" fn luaopen_math(l: LuaState) -> c_int;
	/// Opens the standard 'jit' library for a lua state
	pub extern "C" fn luaopen_jit(l: LuaState) -> c_int;
	/// Opens the standard 'debug' library for a lua state
	pub extern "C" fn luaopen_debug(l: LuaState) -> c_int;
	/// Opens the standard 'bit' library for a lua state
	pub extern "C" fn luaopen_bit(l: LuaState) -> c_int;
	/// Opens the standard library functions (like assert) for a lua state
	pub extern "C" fn luaopen_base(l: LuaState) -> c_int;
	/// Opens all of the standard libraries for a lua state
	pub extern "C" fn luaL_openlibs(l: LuaState) -> ();
	/// Internally called by luaL_register, opens given list of LuaRegs with number of functions provided explicitly
	pub extern "C" fn luaL_openlib(l: LuaState, libname: LuaString, l: *const LuaReg, nup: c_int) -> ();

	/// When called with libname as nullptr, it simply registers all functions in the list l reg! into the table on the top of the stack.
	/// # Example
	/// ```rust
	/// use rglua::prelude::*;
	/// extern "C" fn add(l: LuaState) -> i32 {0}
	/// extern "C" fn sub(l: LuaState) -> i32 {0}
	///
	/// extern "C" fn gmod13_open(l: LuaState) -> i32 {
	/// 	let lib = reg! [
	/// 		"add" => add,
	/// 		"subtract" => sub
	/// 	];
	/// 	luaL_register(l, cstr!("math"), lib.as_ptr());
	///		0
	/// }
	/// ```
	pub extern "C" fn luaL_register(l: LuaState, libname: LuaString, l: *const LuaReg) -> ();

	// Ref
	pub extern "C" fn luaL_ref(l: LuaState, t: c_int) -> c_int;
	pub extern "C" fn luaL_unref(l: LuaState, t: c_int, r: c_int) -> ();

	// Metatables
	pub extern "C" fn luaL_newmetatable(l: LuaState, tname: LuaString) -> c_int;
	pub extern "C" fn luaL_newmetatable_type(l: LuaState, tname: LuaString, typ: c_int) -> c_int;
	pub extern "C" fn luaL_getmetafield(l: LuaState, obj: c_int, e: LuaString) -> c_int;

	// Optional / Default to ``d``
	pub extern "C" fn luaL_optinteger(l: LuaState, narg: c_int, d: LuaInteger) -> c_int;
	pub extern "C" fn luaL_optlstring(l: LuaState, arg: c_int, d: LuaString, l: SizeT)
		-> LuaString;
	pub extern "C" fn luaL_optnumber(l: LuaState, arg: c_int, d: LuaNumber) -> LuaNumber;

	// x / ref functions
	pub extern "C" fn lua_tointegerx(l: LuaState, index: c_int, isnum: *mut c_int) -> LuaInteger;
	pub extern "C" fn lua_tonumberx(l: LuaState, index: c_int, isnum: *mut c_int) -> LuaNumber;

	// Debug
	pub extern "C" fn luaL_traceback(
		l: LuaState,
		state1: LuaState,
		msg: LuaString,
		level: c_int,
	) -> ();
	pub extern "C" fn luaL_where(l: LuaState, lvl: c_int) -> ();

	// Misc
	pub extern "C" fn luaL_testudata(l: LuaState, arg: c_int, tname: LuaString) -> ();
	pub extern "C" fn luaL_execresult(l: LuaState, stat: c_int) -> c_int;
	pub extern "C" fn luaL_fileresult(l: LuaState, stat: c_int, fname: LuaString) -> c_int;
	pub extern "C" fn luaL_findtable(
		l: LuaState,
		idx: c_int,
		fname: LuaString,
		szhint: c_int,
	) -> LuaString;

	/// Pops a key from the stack, and pushes a key-value pair from the table at the given index (the "next" pair after the given key).
	/// If there are no more elements in the table, then lua_next returns 0 (and pushes nothing).
	///
	/// # Safety
	/// Do not call lua_tolstring on a string while traversing a table. This will confuse ``next`` since it modifies the key.
	///
	/// # Examples
	/// ```rust
	/// use rglua::prelude::*;
	/// #[lua_function]
	/// fn table_traverse(l: LuaState) -> i32 {
	///     // Assume a table is in the stack at index 1 (first argument of this function)
	///     lua_pushnil(l);  // first key
	///     // This is nil as how ``pairs()`` passes nil to ``next()`` in lua.
	///     while lua_next(l, 1) != 0 {
	///         // Uses 'key' (at index -2) and 'value' (at index -1)
	///         println!("{} - {}", rstr!(luaL_typename(l, -2)), rstr!(luaL_typename(l, -1)));
	///         // Removes 'value'; keeps 'key' for next iteration
	///         lua_pop(l, 1);
	///     }
	///     0
	/// }
	/// ```
	pub extern "C" fn lua_next(l: LuaState, idx: c_int) -> c_int;

	/// Replaces object at index (idx) with the object at the top of the stack (-1) and pops the stack.
	pub extern "C" fn lua_replace(l: LuaState, idx: c_int) -> ();

	/// Returns 1 if the value at acceptable index index1 is smaller than the value at acceptable index index2,
	/// following the semantics of the Lua < operator (that is, may call metamethods).
	///
	/// Otherwise returns 0. Also returns 0 if any of the indices is non valid.
	pub extern "C" fn lua_lessthan(l: LuaState, idx1: c_int, idx2: c_int) -> c_int;

	pub extern "C" fn lua_checkstack(l: LuaState, extra: c_int) -> c_int;
	/// Sets the error handler for the lua state.
	pub extern "C" fn lua_atpanic(l: LuaState, panicf: LuaCFunction) -> LuaCFunction;
	pub extern "C" fn lua_gettop(l: LuaState) -> c_int;
	pub extern "C" fn lua_remove(l: LuaState, index: c_int) -> ();

	/// Controls lua's garbage collector
	/// Performs different tasks depending on what you provide to the `what` parameter.
	/// # Parameters
	/// * `l` - LuaState
	/// * `what` - c_int
	///     * `[GCSTOP]` - Stops the garbage collector.
	///     * `[GCRESTART]` - Restarts the garbage collector.
	///     * `[GCCOLLECT]` - Performs a full garbage-collection cycle.
	///     * `[GCCOUNT]` - Returns the total number of live Lua objects in the current Lua state.
	///     * `[GCCOUNTB]` - Returns the total number of live Lua objects in the current Lua state, plus the total number of Lua objects in unreachable threads.
	///     * `[GCSTEP]` - Performs a single step of the garbage collector.
	///     * `[GCSETPAUSE]` - Sets `lua_gc`'s pause threshold.
	///     * `[GCSETSTEPMUL]` - Sets `lua_gc`'s step multiplier.
	/// * `data` - c_int
	pub extern "C" fn lua_gc(l: LuaState, what: c_int, data: c_int) -> c_int;

	/// Moves the top element into the given valid index, shifting up the elements above this index to open space.
	/// Cannot be called with a pseudo-index, because a pseudo-index is not an actual stack position.
	pub extern "C" fn lua_insert(l: LuaState, idx: c_int) -> ();


	/// Creates a new thread, pushes it on the stack, and returns a pointer to a lua_State that represents this new thread.
	/// The new state returned by this function shares with the original state all global objects (such as tables), but has an independent execution stack.
	/// There is no explicit function to close or to destroy a thread. Threads are subject to garbage collection, like any Lua object.
	pub extern "C" fn lua_newthread(l: LuaState) -> LuaState;

	/// Returns information about a specific function or function invocation.
	///
	/// To get information about a function you push it onto the stack and start the what string with the character '>'.
	/// (In that case, lua_getinfo pops the function in the top of the stack.)
	///
	/// # Examples
	/// To know in which line a function f was defined, you can write the following code:
	/// ```rust
	/// use rglua::prelude::*;
	/// #[gmod_open]
	/// fn entry(l: LuaState) -> i32 {
	///     let mut ar = LuaDebug::default();
	///     lua_getglobal(l, cstr!("f"));  // Get global 'f'
	///     lua_getinfo(l, cstr!(">S"), &mut ar);
	///
	///     printgm!(l, "{}", ar.linedefined);
	///     0
	/// }
	/// ```
	pub extern "C" fn lua_getinfo(
		l: LuaState,
		what: LuaString,
		ar: *mut LuaDebug,
	) -> c_int;

	/// Returns the "length" of the value at the given acceptable index.
	/// For strings, this is the string length;
	/// For tables, this is the result of the length operator ('#');
	/// For userdata, this is the size of the block of memory allocated for the userdata;
	/// For other values, it is 0.
	pub extern "C" fn lua_objlen(l: LuaState, idx: c_int) -> SizeT;

	pub extern "C" fn lua_gethook(l: LuaState) -> LuaHook;
	pub extern "C" fn lua_gethookcount(l: LuaState) -> c_int;
	pub extern "C" fn lua_gethookmask(l: LuaState) -> c_int;

	/// Gets information about a local variable of a given activation record.
	/// The parameter ar must be a valid activation record that was filled by a previous call to lua_getstack or given as argument to a hook (see lua_Hook).
	/// The index n selects which local variable to inspect (1 is the first parameter or active local variable, and so on, until the last active local variable).
	/// lua_getlocal pushes the variable's value onto the stack and returns its name.
	/// # Returns
	/// Returns NULL (and pushes nothing) when the index is greater than the number of active local variables.
	pub extern "C" fn lua_getlocal(l: LuaState, ar: *mut LuaDebug, n: c_int) -> LuaString;

	/// Get information about the interpreter runtime stack.
	/// This function fills in the priv part of the LuaDebug structure with information about the function that is running at the given level.
	pub extern "C" fn lua_getstack(l: LuaState, level: c_int, ar: *mut LuaDebug) -> c_int;
	pub extern "C" fn lua_getupvalue(l: LuaState, fidx: c_int, idx: c_int) -> LuaString;

	/// Sets the value of a closure's upvalue. Parameters funcindex and n are as in lua_getupvalue (see lua_getupvalue). It assigns the value at the top of the stack to the upvalue and returns its name. It also pops the value from the stack.
	pub extern "C" fn lua_setupvalue(l: LuaState, fidx: c_int, idx: c_int) -> LuaString;

	/// Sets the value of a local variable of a given activation record.
	/// Parameters ar and n are as in lua_getlocal (see lua_getlocal).
	/// lua_setlocal assigns the value at the top of the stack to the variable and returns its name.
	/// It also pops the value from the stack.
	pub extern "C" fn lua_setlocal(l: LuaState, ar: *mut LuaDebug, n: c_int) -> LuaString;

	/// Creates a copy of string 's' by replacing any occurrence of the string 'p' with the string 'r'
	/// Pushes the resulting string on the stack and returns it
	pub extern "C" fn luaL_gsub(s: LuaString, pattern: LuaString, replace: LuaString) -> LuaString;

	/// Exchange values between different threads of the same global state.
	/// This function pops `n` values from the stack `from`, and pushes them onto the stack `to`.
	pub extern "C" fn lua_xmove(from: LuaState, to: LuaState, n: c_int) -> ();

	pub extern "C" fn lua_upvalueid(l: LuaState, fidx: c_int, n: c_int) -> *mut c_void;

	pub extern "C" fn lua_upvaluejoin(l: LuaState, fidx1: c_int, n1: c_int, fidx2: c_int, n2: c_int) -> ();

	/// Initializes a buffer `b`.
	/// This function does not allocate any space; the buffer must be declared as a variable.
	pub extern "C" fn luaL_buffinit(l: LuaState, b: *mut LuaBuffer) -> ();

	/// Returns an address to a space of size $crate::lua::BUFFERSIZE where you can copy a string to be added to buffer [LuaBuffer] `b`.
	/// After copying the string into this space you must call luaL_addsize with the size of the string to actually add it to the buffer.
	pub extern "C" fn luaL_prepbuffer(b: *mut LuaBuffer) -> *mut c_char;

	/// Adds the zero-terminated string pointed to by `s` to the [LuaBuffer] `b` (see luaL_Buffer).
	/// The string may not contain embedded zeros.
	pub extern "C" fn luaL_addstring(b: *mut LuaBuffer, s: LuaString) -> ();

	/// Adds the string pointed to by `s` with length `l` to the [LuaBuffer] `b`.
	/// The string may contain embedded zeros.
	pub extern "C" fn luaL_addlstring(b: *mut LuaBuffer, s: LuaString, l: SizeT) -> ();

	/// Adds the value at the top of the stack to the buffer [LuaBuffer] `b`. Pops the value.
	/// This is the only function on string buffers that can (and must) be called with an extra element on the stack, which is the value to be added to the buffer.
	pub extern "C" fn luaL_addvalue(b: *mut LuaBuffer) -> ();

	/// Finishes the use of buffer `b` leaving the final string on the top of the stack.
	pub extern "C" fn luaL_pushresult(b: *mut LuaBuffer) -> ();

	/// Returns the memory-allocation function of a given state.
	/// If ud is not NULL, Lua stores in *ud the opaque pointer passed to lua_newstate.
	pub extern "C" fn lua_getallocf(l: LuaState, ud: *mut *mut c_void) -> LuaAlloc;

	/// Changes the allocator function of a given state to f with user data ud.
	pub extern "C" fn lua_setallocf(l: LuaState, f: LuaAlloc, ud: *mut c_void) -> ();

	pub extern "C" fn lua_loadx(
		L: LuaState,
		reader: LuaReader,
		dt: *mut c_void,
		chunkname: LuaString,
		mode: LuaString,
	) -> c_int;
}

// Inline functions to mirror the C macros that come with the lua api
lua_macros! {
	/// Pops n elements from the lua stack.
	pub fn lua_pop(l: LuaState, ind: c_int) -> () {
		lua_settop(l, -(ind) - 1);
	};

	/// Gets a value from _G
	/// Internally calls lua_getfield with [crate::lua::GLOBALSINDEX]
	pub fn lua_getglobal(l: LuaState, name: LuaString) -> () {
		lua_getfield(l, GLOBALSINDEX, name);
	};

	/// Sets a value in _G
	/// Internally calls lua_setfield with [crate::lua::GLOBALSINDEX]
	pub fn lua_setglobal(l: LuaState, name: LuaString) -> () {
		lua_setfield(l, GLOBALSINDEX, name);
	};

	/// Pushes a "C" function to the stack
	pub fn lua_pushcfunction(l: LuaState, fnc: LuaCFunction) -> () {
		lua_pushcclosure(l, fnc, 0);
	};

	/// Equivalent to lua_tolstring with len equal to 0
	pub fn lua_tostring(l: LuaState, idx: c_int) -> LuaString {
		lua_tolstring(l, idx, 0)
	};

	/// Starts and resumes a coroutine in a given thread
	pub fn lua_resume(l: LuaState, narg: c_int) -> c_int {
		lua_resume_real(l, narg)
	};

	/// Returns if the value at the given index is a C or Lua function.
	pub fn lua_isfunction(l: LuaState, n: c_int) -> bool {
		lua_type(l, n) == lua::TFUNCTION
	};

	/// Returns if the value at the given index is a table.
	pub fn lua_istable(l: LuaState, n: c_int) -> bool {
		lua_type(l, n) == lua::TTABLE
	};

	pub fn lua_islightuserdata(l: LuaState, n: c_int) -> bool {
		lua_type(l, n) == lua::TLIGHTUSERDATA
	};

	/// Returns if the value at the given index is nil.
	/// You might want to use [lua_isnoneornil] instead.
	pub fn lua_isnil(l: LuaState, n: c_int) -> bool {
		lua_type(l, n) == lua::TNIL
	};

	/// Returns if the value at the given index is a boolean.
	pub fn lua_isboolean(l: LuaState, n: c_int) -> bool {
		lua_type(l, n) == lua::TBOOLEAN
	};

	/// Returns if the value at the given index is a thread.
	pub fn lua_isthread(l: LuaState, n: c_int) -> bool {
		lua_type(l, n) == lua::TTHREAD
	};

	/// Returns if the value at the given index is none (element outside of stack / invalid)
	pub fn lua_isnone(l: LuaState, n: c_int) -> bool {
		lua_type(l, n) == lua::TNONE
	};

	/// Returns if the value at the given index is none (invalid) or nil.
	pub fn lua_isnoneornil(l: LuaState, n: c_int) -> bool {
		lua_type(l, n) <= 0
	};

	/// Loads and pcalls a string of lua code
	/// Returns if the code was successfully executed
	/// Error will be left on the stack if the code failed to execute
	pub fn luaL_dostring(l: LuaState, str: LuaString) -> bool {
		luaL_loadstring(l, str) == 0 || lua_pcall(l, 0, lua::MULTRET, 0) == 0
	};

	/// Loads and pcalls a file's lua code
	/// Returns if the code was successfully executed
	/// Error will be left on the stack if the code failed to execute
	pub fn luaL_dofile(l: LuaState, filename: LuaString) -> bool {
		luaL_loadfile(l, filename) == 0 || lua_pcall(l, 0, lua::MULTRET, 0) == 0
	};

	/// Returns value at [crate::lua::REGISTRYINDEX] with name 'name'
	pub fn luaL_getmetatable(l: LuaState, name: LuaString) -> () {
		lua_getfield(l, lua::REGISTRYINDEX, name);
	};

	/// If a condition is false, throws an argument error at numarg
	pub fn luaL_argcheck(l: LuaState, cond: bool, numarg: c_int, extramsg: LuaString) -> () {
		if !cond {
			luaL_argerror(l, numarg, extramsg);
		}
	};

	/// Returns the type name of object at index i
	pub fn luaL_typename(l: LuaState, i: c_int) -> LuaString {
		lua_typename(l, lua_type(l, i))
	};
}
