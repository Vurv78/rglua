use crate::{
	lua::{self, *},
	userdata::{Angle, Vector},
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
		$vis:vis extern $abi:literal fn $name:ident <$generic:ident>( $($arg:ident : $argty:ty),* $(,)? ) -> $ret:ty; $($rest:tt)*
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
// Credit to https://pgl.yoyo.org/luai/i/about for most of the documentation below here.
// (Of course they were tweaked to be more concise and fit for this library)

// Loading functions
dyn_symbols! {
	/// Function used by [luaL_loadbuffer].
	///
	pub extern "C" fn luaL_loadbufferx(
		l: LuaState,
		code: LuaString,
		size: SizeT,
		id: LuaString,
		mode: LuaString,
	) -> c_int;

	/// Loads a buffer as a Lua chunk.
	/// This function uses [lua_load] to load the chunk in the buffer pointed to by buff with size ``sz``.
	pub extern "C" fn luaL_loadbuffer(
		l: LuaState,
		code: LuaString,
		size: SizeT,
		id: LuaString,
	) -> c_int;

	/// Loads a Lua chunk.
	/// If there are no errors, lua_load pushes the compiled chunk as a Lua function on top of the stack.
	/// Otherwise, it pushes an error message.
	/// # Parameters
	/// * `l` - Lua state,
	/// * `reader` - [LuaReader] function used to read the chunk.
	/// * `data` - Opaque value (userdata) passed to the reader function
	/// * `chunkname` - Name to identify the chunk, used in error messages / debugging.
	/// # Returns
	/// * 0 - No errors, [OK]
	/// * [ERRSYNTAX] - Syntax error,
	/// * [ERRMEM] - Memory allocation error,
	/// # Notes
	/// * This function only loads a chunk; it does not run it.
	/// * [lua_load] automatically detects whether the chunk is text or binary, and loads it accordingly.
	pub extern "C" fn lua_load(
		l: LuaState,
		reader: LuaReader,
		data: *mut c_void,
		chunkname: LuaString
	) -> c_int;

	/// Function used by [lua_load] internally.
	/// ``mode`` is whether to take the chunk as bytecode or as text.
	/// You should just use [lua_load] instead though.
	pub extern "C" fn lua_loadx(
		l: LuaState,
		reader: LuaReader,
		dt: *mut c_void,
		chunkname: LuaString,
		mode: LuaString,
	) -> c_int;

	/// Loads a string as a Lua chunk. This function uses [lua_load] to load the chunk in the zero-terminated string ``s``.
	/// This function returns the same results as [lua_load].
	/// Also as [lua_load], this function only loads the chunk; it does not run it.
	pub extern "C" fn luaL_loadstring(l: LuaState, code: LuaString) -> c_int;

	/// Loads a file as a Lua chunk.
	/// This function uses [lua_load] to load the chunk in the file named ``filename``.
	/// If filename is None, then it loads from the standard input.
	/// The first line in the file is ignored if it starts with a # (shebang)
	pub extern "C" fn luaL_loadfile(l: LuaState, filename: Option<LuaString>) -> c_int;

	/// Same as how [lua_loadx] is to [lua_load].
	/// You should probably use [luaL_loadfile] instead.
	pub extern "C" fn luaL_loadfilex(l: LuaState, filename: LuaString, mode: LuaString) -> c_int;
}

// Calling lua code
dyn_symbols! {
	/// Calls a function in protected mode.
	/// Both nargs and nresults have the same meaning as in [lua_call].
	/// If there are no errors during the call, [lua_pcall] behaves exactly like [lua_call].
	///
	/// However, if there is any error, [lua_pcall] catches it, pushes a single value on the stack (the error message), and returns an error code.
	/// Like [lua_call], [lua_pcall] always removes the function and its arguments from the stack.
	///
	/// If errfunc is 0, then the error message returned on the stack is exactly the original error message.
	/// Otherwise, errfunc is the stack index of an error handler function. (In the current implementation, this index cannot be a pseudo-index like [GLOBALSINDEX])
	/// In case of runtime errors, this function will be called with the error message and its return value will be the message returned on the stack by [lua_pcall].
	///
	/// Typically, the error handler function is used to add more debug information to the error message, such as a stack traceback.
	/// Such information cannot be gathered after the return of lua_pcall, since by then the stack has unwound.
	/// # Returns
	/// This function returns 0 in case of success or these error codes:
	/// * [ERRRUN] - There was an error at runtime
	/// * [ERRMEM] - There was a memory allocation error
	/// * [ERRERR] - Error when running the error handler
	pub extern "C" fn lua_pcall(l: LuaState, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;


	/// Calls a function.
	/// To call a function you must use the following protocol: first, the function to be called is pushed onto the stack;
	/// then, the arguments to the function are pushed in direct order -- that is, the first argument is pushed first.
	///
	/// Finally you call [lua_call].
	/// # Params
	/// * `l` - The lua state.
	/// * `nargs` - The number of arguments that you pushed onto the stack.
	/// * `nresults` - Number of expected results to push onto the stack, or [MULTRET] to push all results.
	/// # Stack Behavior
	/// All arguments and the function value are popped from the stack when the function is called.
	/// The function results are pushed onto the stack when the function returns in direct order, so the last result is on the top of the stack.
	pub extern "C" fn lua_call(l: LuaState, nargs: c_int, nresults: c_int) -> c_int;

	/// Calls the C function func in protected mode.
	/// ``func`` starts with only one element in its stack, a light userdata containing ud.
	/// In case of errors, this returns the same error codes as lua_pcall, plus the error object on the top of the stack.
	/// Otherwise, it returns zero, and does not change the stack.
	/// All values returned by func are discarded.
	pub extern "C" fn lua_cpcall(l: LuaState, func: LuaCFunction, userdata: *mut c_void) -> c_int;

	/// Calls a metamethod.
	/// If the object at index obj has a metatable and this metatable has a field e, this function calls this field and passes the object as its only argument.
	/// # Returns
	/// In this case this function returns 1 and pushes onto the stack the value returned by the call.
	/// If there is no metatable or no metamethod, this function returns 0 (without pushing any value on the stack).
	pub extern "C" fn luaL_callmeta(l: LuaState, obj: c_int, name: LuaString) -> c_int;
}

dyn_symbols! {
	/// Does the equivalent to t\[k\] = v, where t is the value at the given valid index and v is the value at the top of the stack.
	/// This function pops the value from the stack.
	/// As in Lua, this function may trigger the __newindex metamethod.
	pub extern "C" fn lua_setfield(l: LuaState, idx: c_int, name: LuaString) -> ();

	/// Pops a table from the stack and sets it as the new metatable for the value at the given acceptable index.
	pub extern "C" fn lua_setmetatable(l: LuaState, idx: c_int) -> ();

	/// Accepts any acceptable index, or 0, and sets the stack top to this index.
	/// If the new top is larger than the old one, then the new elements are filled with nil.
	/// If index is 0, then all stack elements are removed.
	pub extern "C" fn lua_settop(l: LuaState, ind: c_int) -> ();

	/// Pops a table from the stack and sets it as the new environment for the value at the given index.
	/// # Returns
	/// If the value at the given index is neither a function nor a thread nor a userdata, returns 0.
	/// Otherwise returns 1.
	pub extern "C" fn lua_setfenv(l: LuaState, idx: c_int) -> c_int;

	/// Does the equivalent to t\[k\] = v, where t is the value at the given valid index, v is the value at the top of the stack, and k is the value just below the top.
	pub extern "C" fn lua_settable(l: LuaState, idx: c_int) -> ();

	/// Same as lua_settable, but without calling any metamethods.
	pub extern "C" fn lua_rawset(l: LuaState, idx: c_int) -> ();

	/// Does the equivalent of t\[n\] = v, where t is the value at the given valid index and v is the value at the top of the stack.
	/// This function pops the value from the stack. The assignment is raw; that is, it does not invoke metamethods.
	pub extern "C" fn lua_rawseti(l: LuaState, idx: c_int, n: c_int) -> ();
}

// Getters
dyn_symbols! {
	/// Pushes onto the stack the value t\[k\], where t is the value at the given valid index and k is the value at the top of the stack.
	/// This function pops the key from the stack (putting the resulting value in its place). As in Lua, this function may trigger a metamethod for the "index" event (see §2.8).
	pub extern "C" fn lua_gettable(l: LuaState, idx: c_int) -> ();

	/// This is the same as lua_gettable, but without calling any metamethods
	pub extern "C" fn lua_rawget(l: LuaState, idx: c_int) -> ();

	/// Pushes onto the stack the value t\[n\], where t is the value at the given valid index.
	/// The access is raw; that is, it does not invoke metamethods.
	pub extern "C" fn lua_rawgeti(l: LuaState, idx: c_int, n: c_int) -> ();

	/// Pushes onto the stack the environment table of the value at the given index.
	pub extern "C" fn lua_getfenv(l: LuaState, idx: c_int) -> ();

	/// Pushes onto the stack the metatable of the value at the given acceptable index.
	/// If the index is not valid, or if the value does not have a metatable, the function returns 0 and pushes nothing on the stack.
	pub extern "C" fn lua_getmetatable(l: LuaState, idx: c_int) -> c_int;

	/// Pushes onto the stack the value t\[k\], where t is the value at ``idx``.
	/// As in Lua, this function may trigger a metamethod for the "index" event (see §2.8).
	pub extern "C" fn lua_getfield(l: LuaState, idx: c_int, key: LuaString) -> ();
}

// Non-stack getters
dyn_symbols! {
	/// Returns the type of the value in the given acceptable index, or LUA_TNONE for a non-valid index (that is, an index to an "empty" stack position).
	/// The types returned by lua_type are coded by the following constants:
	/// [TNIL], [TNUMBER], [TBOOLEAN], [TSTRING], [TTABLE], [TFUNCTION], [TUSERDATA], [TTHREAD], and [TLIGHTUSERDATA].
	pub extern "C" fn lua_type(l: LuaState, idx: c_int) -> c_int;

	/// Returns the name of the type ``typeid`` which must be one the values returned by [lua_type].
	/// Use [luaL_typename] if you want to get it directly from a value in the stack.
	pub extern "C" fn lua_typename(l: LuaState, typeid: c_int) -> LuaString; // To be used with the return value of lua_type

	// Type conversion getters

	/// Converts the Lua value at the given index to a C string.
	/// If len is not 0, it also sets *len with the string length.
	/// The Lua value must be a string or a number; otherwise, the function returns a nullptr.
	/// If the value is a number, then lua_tolstring also changes the actual value in the stack to a string.
	/// (This change confuses lua_next when lua_tolstring is applied to keys during a table traversal.)
	pub extern "C" fn lua_tolstring(l: LuaState, ind: c_int, size: SizeT) -> Option<LuaString>;

	/// Converts the Lua value at the given acceptable index to a C boolean value (0 or 1).
	/// Like all tests in Lua, lua_toboolean returns 1 for any Lua value different from false and nil; otherwise returning 0.
	/// This also returns 0 when called with a non-valid index. (If you want to accept only actual boolean values, use [lua_isboolean] to test the value's type.)
	pub extern "C" fn lua_toboolean(l: LuaState, idx: c_int) -> c_int;

	/// Converts a value at the given acceptable index to a C function.
	/// That value must be a C function; otherwise, returns None.
	/// # Example
	/// ```rust
	/// use rglua::prelude::*;
	/// #[gmod_open]
	/// fn entry(l: LuaState) -> i32 {
	/// 	lua_getglobal(l, cstr!("CurTime"));
	/// 	let curtime = lua_tocfunction(l, -1).unwrap();
	/// 	0
	/// }
	/// ```
	pub extern "C" fn lua_tocfunction(l: LuaState, idx: c_int) -> Option<LuaCFunction>;

	/// Converts the Lua value at the given acceptable index to the signed integral type [LuaInteger].
	/// The Lua value must be a number or a string convertible to a number; otherwise, this returns 0.
	/// If the number is not an integer, it is truncated in some non-specified way.
	pub extern "C" fn lua_tointeger(l: LuaState, idx: c_int) -> LuaInteger;

	/// Converts the Lua value at the given acceptable index to a [LuaNumber].
	/// The Lua value must be a number or a string convertible to a number; otherwise, this returns 0.
	pub extern "C" fn lua_tonumber(l: LuaState, idx: c_int) -> LuaNumber;

	/// Converts the value at the given acceptable index to a generic C pointer (void*).
	/// The value can be a userdata, a table, a thread, or a function; otherwise this returns None.
	/// Different objects will give different pointers.
	/// There is no way to convert the pointer back to its original value.
	pub extern "C" fn lua_topointer(l: LuaState, idx: c_int) -> Option<*mut c_void>;

	/// Converts the value at the given acceptable index to a Lua thread (represented as lua_State*).
	/// This value must be a thread; otherwise, the function returns None.
	pub extern "C" fn lua_tothread(l: LuaState, idx: c_int) -> Option<LuaState>;

	/// Returns the value at the given index assuming it is a userdata.
	/// # Returns
	/// If the value at the given acceptable index is a full userdata, returns its block address.
	/// If the value is a light userdata, returns its pointer.
	/// Otherwise, returns None.
	pub extern "C" fn lua_touserdata(l: LuaState, idx: c_int) -> Option<*mut c_void>;
}

dyn_symbols! {
	/// Pushes the zero-terminated string pointed to by s onto the stack. Lua makes (or reuses) an internal copy of the given string, so the memory at s can be freed or reused immediately after the function returns. The string cannot contain embedded zeros; it is assumed to end at the first zero.
	pub extern "C" fn lua_pushstring(l: LuaState, s: LuaString) -> ();

	/// Pushes a boolean onto the stack. Note this is still a [c_int] so use 0 for false and 1 for true.
	pub extern "C" fn lua_pushboolean(l: LuaState, s: c_int) -> ();

	/// Pushes a string of length ``sz`` onto the stack.
	pub extern "C" fn lua_pushlstring(l: LuaState, s: LuaString, sz: SizeT) -> ();

	/// Pushes a `nil` value onto the stack.
	pub extern "C" fn lua_pushnil(l: LuaState) -> ();

	/// Pushes the number ``num`` onto the stack.
	pub extern "C" fn lua_pushnumber(l: LuaState, num: LuaNumber) -> ();

	/// Pushes a copy of the element at the given valid index onto the stack.
	pub extern "C" fn lua_pushvalue(l: LuaState, idx: c_int) -> ();
	/// Pushes a c function on the stack with associated values.
	/// # Parameters
	/// * `l` - LuaState
	/// * `f` - Lua function
	/// * `n` - Number of upvalues to associate and pull from stack with the function
	pub extern "C" fn lua_pushcclosure(l: LuaState, fnc: LuaCFunction, nargs: c_int) -> ();

	/// Pushes a light userdata onto the stack.
	/// Userdata represent C values in Lua.
	/// A light userdata represents a pointer.
	/// It is a value (like a number): you do not create it, it has no individual metatable, and it is not collected (as it was never created).
	/// A light userdata is equal to "any" light userdata with the same C address.
	pub extern "C" fn lua_pushlightuserdata(l: LuaState, p: *mut c_void) -> ();

	/// Pushes a given thread (representing ``l``) to the stack.
	/// # Parameters
	/// * `l` - The thread to push.
	/// # Returns
	/// 1 if the thread is the main thread of the state.
	pub extern "C" fn lua_pushthread(l: LuaState) -> c_int;

	/// Pushes a formatted [LuaString] to the stack
	pub extern "C" fn lua_pushfstring(l: LuaState, fmt: LuaString, ...) -> LuaString;

	/// Pushes a number with value ``n`` onto the stack.
	pub extern "C" fn lua_pushinteger(l: LuaState, n: LuaInteger) -> ();
}

// Type checking getters
dyn_symbols! {
	/// Same as luaL_checknumber, but casts it to an integer.
	pub extern "C" fn luaL_checkinteger(l: LuaState, narg: c_int) -> LuaInteger;
	/// Checks whether the value at stack index 'narg' is a number and returns this number.
	/// If it is not a lua number, will throw an error to Lua.
	pub extern "C" fn luaL_checknumber(l: LuaState, narg: c_int) -> LuaNumber;

	/// Checks whether the function argument ``narg`` is a string and returns this string.
	/// If len is not 0 fills *len with the string's length.
	pub extern "C" fn luaL_checklstring(l: LuaState, narg: c_int, len: SizeT) -> LuaString;

	/// Checks whether the function has an argument of any type (including nil) at position narg.
	pub extern "C" fn luaL_checkany(l: LuaState, narg: c_int) -> ();

	/// Checks whether the function argument narg has type ``t``.
	/// See [lua_type] for the encoding of types for ``t``.
	pub extern "C" fn luaL_checktype(l: LuaState, narg: c_int, typeid: c_int) -> ();

	/// Checks whether the function argument narg is a userdata of the type tname (see luaL_newmetatable).
	pub extern "C" fn luaL_checkudata(l: LuaState, ud: c_int, tname: LuaString) -> *mut Userdata;
}

// Creation
dyn_symbols! {
	/// Creates a new Lua state.
	/// This calls [lua_newstate] with an allocator based on the standard C realloc function and then sets a panic function (see lua_atpanic) that prints an error message to the standard error output in case of fatal errors.
	/// # Returns
	/// The newly created [LuaState], or None if the allocation failed (due to memory).
	pub extern "C" fn luaL_newstate() -> Option<LuaState>;

	/// Creates a new, independent state.
	/// Note you might be looking for [luaL_newstate], which has no parameters
	/// Returns None if cannot create the state (due to lack of memory).
	/// The argument f is the allocator function;
	/// Lua does all memory allocation for this state through this function.
	/// The second argument, ud, is an opaque pointer that Lua simply passes to the allocator in every call.
	pub extern "C" fn lua_newstate(f: LuaAlloc, ud: *mut c_void) -> Option<LuaState>;

	/// Creates a new empty table and pushes it onto the stack.
	/// The new table has space pre-allocated for ``narr`` array elements and ``nrec`` non-array elements.
	/// This pre-allocation is useful when you know exactly how many elements the table will have.
	/// Otherwise you can use the function [lua_newtable].
	pub extern "C" fn lua_createtable(l: LuaState, narr: c_int, nrec: c_int) -> ();
}

// Destruction
dyn_symbols! {
	/// Destroys the given lua state.
	/// You *probably* don't want to do this, unless you just want to self destruct the server / your client.
	pub extern "C" fn lua_close(l: LuaState) -> ();
}

// LuaJIT
dyn_symbols! {
	/// This is a C API extension to allow control of the VM from "C"
	/// # Parameters
	/// * `l` - Lua state
	/// * `idx` - Stack index of the function to set the mode of. None to set the mode of the entirety of luajit.
	/// * `mode` - The mode to set, 'or'ed with a flag from [lua::jit]
	/// # Returns
	/// 1 for success, 0 for failure.
	pub extern "C" fn luaJIT_setmode(l: LuaState, idx: Option<c_int>, jit_mode: c_int) -> c_int;
}

// Coroutines
dyn_symbols! {
	/// Yields a coroutine.
	/// This function should only be called as the return expression of a C function, as follows:
	/// ```ignore
	/// return lua_yield (L, nresults);
	/// ```
	/// When a function calls [lua_yield] in that way, the running coroutine suspends its execution, and the call to [lua_resume] that started this coroutine returns.
	/// The parameter nresults is the number of values from the stack that are passed as results to [lua_resume].
	pub extern "C" fn lua_yield(l: LuaState, nresults: c_int) -> c_int;

	/// Returns the status of the thread/coroutine l.
	/// # Returns
	/// 0 for a normal thread, error code if it's finished with an error, or [lua::YIELD] if it is suspended.
	pub extern "C" fn lua_status(l: LuaState) -> c_int;

	/// Starts and resumes a coroutine in a given thread.
	/// Blame garry for the _real
	pub extern "C" fn lua_resume_real(l: LuaState, narg: c_int) -> c_int;
}

// Comparison
dyn_symbols! {
	/// Returns 1 or 0 for if the two values at given indices are equal, calling ``__eq`` metamethods along the way unlike [lua_rawequal].
	/// Also returns 0 if any of the indices are non valid.
	pub extern "C" fn lua_equal(l: LuaState, ind1: c_int, ind2: c_int) -> c_int; // Returns 1 or 0 bool

	/// Returns 1 or 0 for if the two values at given indices are equal, without calling metamethods, as [lua_equal] does.
	/// Also returns 0 if any of the indices are non valid.
	pub extern "C" fn lua_rawequal(l: LuaState, ind1: c_int, ind2: c_int) -> c_int;
}

dyn_symbols! {
	// Raising Errors
	/// Generates an error with a message like the following:
	/// ```text
	/// location: bad argument narg to 'func' (tname expected, got rt)
	/// ```
	/// where location is produced by luaL_where, func is the name of the current function, and rt is the type name of the actual argument.
	pub extern "C" fn luaL_typerror(l: LuaState, narg: c_int, typename: LuaString) -> !;

	/// Raises an error.
	/// The error message format is given by fmt plus any extra arguments, following the same rules of [lua_pushfstring].
	/// It also adds at the beginning of the message the file name and the line number where the error occurred, if this information is available.
	pub extern "C" fn luaL_error(l: LuaState, fmt: LuaString, ...) -> !;

	/// Raises an error with the following message, where func is retrieved from the call stack:
	/// ```text
	/// bad argument #<narg> to <func> (<extramsg>)
	/// ```
	/// This function never returns
	pub extern "C" fn luaL_argerror(l: LuaState, narg: c_int, extramsg: LuaString) -> !;

	/// Generates a Lua error.
	/// The error message (which can actually be a Lua value of any type) must be on the stack top.T
	/// This function does a long jump, and therefore never returns. (see [luaL_error]).
	pub extern "C" fn lua_error(l: LuaState) -> !;
}

dyn_symbols! {
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

	/// Registers a ``reg`` of functions onto the Lua State's _G\[libname\].
	/// For example you could set libname to cstr!("math") to add functions onto the ``math`` table or create it if it does not exist.
	///
	/// When called with libname as std::ptr::null(), it simply registers all functions in the list ``lib`` into the table on the top of the stack.
	/// # Example
	/// ```rust
	/// use rglua::prelude::*;
	///
	/// #[lua_function] fn add(l: LuaState) -> i32 {0}
	/// #[lua_function] fn sub(l: LuaState) -> i32 {0}
	///
	/// #[gmod_open]
	/// fn entry(l: LuaState) -> i32 {
	/// 	let lib = reg! [
	/// 		"add" => add,
	/// 		"subtract" => sub
	/// 	];
	/// 	luaL_register(l, cstr!("math"), lib.as_ptr());
	///		0
	/// }
	/// ```
	pub extern "C" fn luaL_register(l: LuaState, libname: LuaString, lib: *const LuaReg) -> ();
}

dyn_symbols! {
	/// Creates and returns a reference, in the table at index t, for the object at the top of the stack (and pops the object).
	/// A reference is a unique integer key.
	/// As long as you do not manually add integer keys into table t, luaL_ref ensures the uniqueness of the key it returns.
	/// You can retrieve an object referred by reference r by calling lua_rawgeti(L, t, r).
	/// Function luaL_unref frees a reference and its associated object.
	///
	/// If the object at the top of the stack is nil, luaL_ref returns the constant LUA_REFNIL.
	/// The constant LUA_NOREF is guaranteed to be different from any reference returned by luaL_ref.
	pub extern "C" fn luaL_ref(l: LuaState, t: c_int) -> c_int;

	/// Releases reference ref from the table at index t (see luaL_ref).
	/// The entry is removed from the table, so that the referred object can be collected.
	/// The reference ref is also freed to be used again.
	/// If ref is LUA_NOREF or LUA_REFNIL, this does nothing.
	pub extern "C" fn luaL_unref(l: LuaState, t: c_int, r: c_int) -> ();
}

// Metatables
dyn_symbols! {
	/// If the registry already has the key tname, returns 0. Otherwise, creates a new table to be used as a metatable for userdata, adds it to the registry with key tname, and returns 1.
	/// In both cases pushes onto the stack the final value associated with ``tname`` in the registry.
	pub extern "C" fn luaL_newmetatable(l: LuaState, tname: LuaString) -> c_int;

	/// Creates a metatable with type and typeid
	/// Same as luaL_newmetatable, but also sets the MetaName and MetaID fields of the metatable
	/// # Parameters
	/// * `l` - LuaState
	/// * `tname` - TypeName to be added to the metatable
	/// * `tid` - TypeID to be applied to the metatable
	pub extern "C" fn luaL_newmetatable_type(l: LuaState, tname: LuaString, tid: c_int) -> c_int;

	/// Pushes onto the stack the field ``e`` from the metatable of the object at index ``obj``.
	/// If the object does not have a metatable, or if the metatable does not have this field, returns 0 and pushes nothing.
	pub extern "C" fn luaL_getmetafield(l: LuaState, obj: c_int, e: LuaString) -> c_int;
}

// Optional
dyn_symbols! {
	/// If the function argument ``narg`` is a number, returns this number cast to a [LuaInteger].
	/// If this argument is absent or is nil, returns d. Otherwise, raises an error.
	pub extern "C" fn luaL_optinteger(l: LuaState, narg: c_int, d: LuaInteger) -> c_int;

	/// If the function argument narg is a string, returns this string.
	/// If this argument is absent or is nil, returns d. Otherwise, raises an error.
	///
	/// If ``sz`` is not 0, fills the position *``sz`` with the results's length.
	pub extern "C" fn luaL_optlstring(l: LuaState, arg: c_int, d: LuaString, sz: SizeT)
		-> LuaString;

	/// If the function argument ``arg`` is a number, returns this number.
	/// If this argument is absent or is nil, returns ``d``. Otherwise, raises an error.
	pub extern "C" fn luaL_optnumber(l: LuaState, arg: c_int, d: LuaNumber) -> LuaNumber;
}

dyn_symbols! {
	// x / ref functions
	/// Converts the Lua value at the given index to the signed integral type lua_Integer.
	/// The Lua value must be an integer, or a number or string convertible to an integer; otherwise, this returns 0.
	/// If ``isnum`` is not None, its referent is assigned a boolean value that indicates whether the operation succeeded.
	pub extern "C" fn lua_tointegerx(l: LuaState, index: c_int, isnum: Option<*mut c_int>) -> LuaInteger;


	/// Converts the Lua value at the given index to a LuaNumber (f64).
	/// The Lua value must be a number or a string convertible to a number; otherwise, this returns 0.
	/// If ``isnum`` is not None, its referent is assigned a boolean value that indicates whether the operation succeeded.
	pub extern "C" fn lua_tonumberx(l: LuaState, index: c_int, isnum: Option<*mut c_int>) -> LuaNumber;
}

dyn_symbols! {
	/// Creates and pushes a traceback of the stack L1.
	/// If msg is not None it is appended at the beginning of the traceback.
	/// The level parameter tells at which level to start the traceback.
	pub extern "C" fn luaL_traceback(
		l: LuaState,
		state1: LuaState,
		msg: Option<LuaString>,
		level: c_int,
	) -> ();

	/// Pushes onto the stack a string identifying the current position of the control at level ``lvl`` in the call stack.
	/// Typically this string has the following format:
	/// ```text
	/// chunkname:currentline:
	/// ```
	/// Level 0 is the running function, level 1 is the function that called the running function, etc.
	/// This function is used to build a prefix for error messages.
	pub extern "C" fn luaL_where(l: LuaState, lvl: c_int) -> ();

	/// This function produces the return values for process-related functions in the standard library (os.execute and io.close).
	/// Although, those don't exist in gmod..
	pub extern "C" fn luaL_execresult(l: LuaState, stat: c_int) -> c_int;

	/// This function produces the return values for file-related functions in the standard library (like File:seek)
	pub extern "C" fn luaL_fileresult(l: LuaState, stat: c_int, fname: LuaString) -> c_int;

	/// Function used internally by lua
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
	/// Do not call [lua_tolstring] on a string while traversing a table. This will confuse ``next`` since it modifies the key.
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

	/// Ensures that there are at least extra free stack slots in the stack.
	/// It returns C 'false' if it cannot grow the stack to that size.
	/// This function never shrinks the stack; if the stack is already larger than the new size, it is left unchanged.
	pub extern "C" fn lua_checkstack(l: LuaState, extra: c_int) -> c_int;

	/// Sets a new panic function and returns the old one.
	/// If an error happens outside any protected environment, Lua calls a panic function and then calls exit(EXIT_FAILURE), thus exiting the host application.
	/// Your panic function can avoid this exit by never returning (e.g., doing a long jump).
	/// The panic function can access the error message at the top of the stack.
	/// # Returns
	/// The old panic function.
	pub extern "C" fn lua_atpanic(l: LuaState, panicf: LuaCFunction) -> LuaCFunction;

	/// Returns the index of the top element in the stack.
	/// Because indices start at 1, this result is equal to the number of elements in the stack (and so 0 means an empty stack).
	pub extern "C" fn lua_gettop(l: LuaState) -> c_int;

	/// Removes the element at the given valid index, shifting down the elements above this index to fill the gap.
	/// Cannot be called with a pseudo-index, because a pseudo-index is not an actual stack position.
	/// (Example of pseudoindices are LUA_GLOBALSINDEX and globals::REGISTRYINDEX)
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

	/// This function allocates a new block of memory with the given size, pushes onto the stack a new full userdata with the block address, and returns this address.
	///
	/// Userdata represent C values in Lua.
	/// A full userdata represents a block of memory.
	/// It is an object (like a table): you must create it, it can have its own metatable, and you can detect when it is being collected.
	/// A full userdata is only equal to itself (under raw equality).
	///
	/// When Lua collects a full userdata with a gc metamethod, Lua calls the metamethod and marks the userdata as finalized.
	/// When this userdata is collected again then Lua frees its corresponding memory.
	pub extern "C" fn lua_newuserdata(l: LuaState, size: SizeT) -> *mut Userdata;

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
}

// Lua Debug Library
dyn_symbols! {
	/// Returns the current hook function.
	pub extern "C" fn lua_gethook(l: LuaState) -> LuaHook;
	/// Returns the current hook count.
	pub extern "C" fn lua_gethookcount(l: LuaState) -> c_int;
	/// Returns the current hook mask.
	pub extern "C" fn lua_gethookmask(l: LuaState) -> c_int;

	/// Sets the debugging hook function.
	/// # Parameters
	///
	/// * `l` - [LuaState]
	/// * `func` [LuaHook] function
	/// * `mask` - Specifies on which events the hook will be called: it is formed by a bitwise or of the constants [MASKCALL], [MASKRET], [MASKLINE], and [MASKCOUNT]
	/// * `count` - Only meaningful when the mask includes [MASKCOUNT]. For each event, the hook is called as explained below:
	///
	/// **The call hook**: called when the interpreter calls a function. The hook is called just after Lua enters the new function, before the function gets its arguments.
	/// **The return hook**: called when the interpreter returns from a function. The hook is called just before Lua leaves the function. You have no access to the values to be returned by the function.
	/// **The line hook**: is called when the interpreter is about to start the execution of a new line of code, or when it jumps back in the code (even to the same line). (This event only happens while Lua is executing a Lua function.)
	/// **The count hook**: is called after the interpreter executes every count instructions. (This event only happens while Lua is executing a Lua function.)
	///
	/// A hook is disabled by setting ``mask`` to zero.
	pub extern "C" fn lua_sethook(l: LuaState, func: LuaHook, mask: c_int, count: c_int) -> c_int;

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

	/// Gets information about a closure's upvalue. This is basically debug.getlocal.
	/// (For Lua functions, upvalues are the external local variables that the function uses, and that are consequently included in its closure.)
	/// # Parameters
	/// * `idx` - Index of the upvalue to push the value of onto the stack and return the name of (like debug.getlocal)
	/// * `fidx` - Points to the closure in the stack.
	/// # Note
	/// Upvalues have no particular order, as they are active through the whole function.
	/// So, they are numbered in an arbitrary order.
	/// # Returns
	/// The name of the upvalue at given index `idx`, or NULL (and pushes nothing) if the index is greater than the number of upvalues.
	/// For C functions (functions not created in lua), this returns an empty string for the name of all upvalues
	pub extern "C" fn lua_getupvalue(l: LuaState, fidx: c_int, idx: c_int) -> LuaString;

	/// Sets the value of a closure's upvalue. Parameters funcindex and n are as in lua_getupvalue (see lua_getupvalue). It assigns the value at the top of the stack to the upvalue and returns its name. It also pops the value from the stack.
	pub extern "C" fn lua_setupvalue(l: LuaState, fidx: c_int, idx: c_int) -> LuaString;

	/// Sets the value of a local variable of a given activation record.
	/// Parameters ar and n are as in lua_getlocal (see lua_getlocal).
	/// lua_setlocal assigns the value at the top of the stack to the variable and returns its name.
	/// It also pops the value from the stack.
	pub extern "C" fn lua_setlocal(l: LuaState, ar: *mut LuaDebug, n: c_int) -> LuaString;
}

dyn_symbols! {
	/// Creates a copy of string 's' by replacing any occurrence of the string 'p' with the string 'r'
	/// Pushes the resulting string on the stack and returns it
	pub extern "C" fn luaL_gsub(s: LuaString, pattern: LuaString, replace: LuaString) -> LuaString;

	/// Exchange values between different threads of the same global state.
	/// This function pops `n` values from the stack `from`, and pushes them onto the stack `to`.
	pub extern "C" fn lua_xmove(from: LuaState, to: LuaState, n: c_int) -> ();
}

dyn_symbols! {
	/// Returns an unique identifier for the upvalue numbered n from the closure at index funcindex.
	/// Parameters funcindex and n are as in the lua_getupvalue (see lua_getupvalue) (but n cannot be greater than the number of upvalues).
	/// These unique identifiers allow a program to check whether different closures share upvalues.
	/// Lua closures that share an upvalue (that is, that access a same external local variable) will return identical ids for those upvalue indices.
	pub extern "C" fn lua_upvalueid(l: LuaState, fidx: c_int, n: c_int) -> *mut c_void;

	/// Make the ``n1`` upvalue of the Lua closure at index ``fidx1`` refer to the ``n2`` upvalue of the Lua closure at index ``fidx2``.
	pub extern "C" fn lua_upvaluejoin(l: LuaState, fidx1: c_int, n1: c_int, fidx2: c_int, n2: c_int) -> ();
}

// Buffer functions
dyn_symbols! {
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
}

dyn_symbols! {
	/// Returns the memory-allocation function of a given state.
	/// If ud is not NULL, Lua stores in *ud the opaque pointer passed to lua_newstate.
	pub extern "C" fn lua_getallocf(l: LuaState, ud: *mut *mut c_void) -> LuaAlloc;

	/// Changes the allocator function of a given state to f with user data ud.
	pub extern "C" fn lua_setallocf(l: LuaState, f: LuaAlloc, ud: *mut c_void) -> ();
}

// Misc
dyn_symbols! {
	/// Dumps a function as a binary chunk.
	/// Receives a Lua function on the top of the stack and produces a binary chunk that, if loaded again, results in a function equivalent to the one dumped. As it produces parts of the chunk, lua_dump calls function writer (see lua_Writer) with the given data to write them.
	pub extern "C" fn lua_dump(l: LuaState, writer: LuaWriter, data: *mut c_void) -> c_int;

	/// Grows the stack size to top + sz elements, raising an error if the stack cannot grow to that size. msg is an additional text to go into the error message.
	/// # Note
	/// You may be looking for [lua_checkstack]
	pub extern "C" fn luaL_checkstack(l: LuaState, size: c_int, msg: LuaString) -> ();
}

dyn_symbols! {
	/// Returns 1 if the value at the given acceptable index is a number or a string convertible to a number, and 0 otherwise.
	pub extern "C" fn lua_isnumber(l: LuaState, idx: c_int) -> c_int;

	/// Returns 1 if the value at the given acceptable index is a string or a number (which is always convertible to a string), and 0 otherwise.
	pub extern "C" fn lua_isstring(l: LuaState, idx: c_int) -> c_int;

	/// Returns 1 if the value at the given acceptable index is a C function, and 0 otherwise.
	pub extern "C" fn lua_iscfunction(l: LuaState, idx: c_int) -> c_int;

	/// Returns 1 if the value at the given acceptable index is a userdata (either full or light), and 0 otherwise.
	pub extern "C" fn lua_isuserdata(l: LuaState, idx: c_int) -> c_int;
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
	/// This may return None if the value at ``idx`` is not a string or a number, use [luaL_optstring] instead if you do not desire an Option<> or unwrap when you are absolutely sure of the type.
	pub fn lua_tostring(l: LuaState, idx: c_int) -> Option<LuaString> {
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
		luaL_loadfile(l, Some(filename)) == 0 || lua_pcall(l, 0, lua::MULTRET, 0) == 0
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

	/// Asserts that a string argument exists at index 'i'
	pub fn luaL_checkstring(l: LuaState, i: c_int) -> LuaString {
		luaL_checklstring(l, i, 0)
	};

	/// Like lua_tostring or luaL_checkstring, but instead of returning an invalid string / erroring,
	/// It returns the given `default` string.
	pub fn luaL_optstring(l: LuaState, i: c_int, default: LuaString) -> LuaString {
		luaL_optlstring(l, i, default, 0)
	};

	/// Sets the C function ``f`` as the value of global name ``name``.
	pub fn lua_register(l: LuaState, name: LuaString, f: LuaCFunction) -> () {
		lua_pushcfunction(l, f);
		lua_setglobal(l, name);
	};

	/// Creates a new empty table and pushes it onto the stack.
	/// It is equivalent to ``lua_createtable(l, 0, 0)``.
	pub fn lua_newtable(l: LuaState) -> () {
		lua_createtable(l, 0, 0);
	};
}

// Userdata helpers
lua_macros! {
	pub fn luaL_checkvector(l: LuaState, narg: c_int) -> Vector {
		unsafe { *( (*luaL_checkudata(l, narg, cstr!("Vector"))).data as *mut _) }
	};

	pub fn luaL_checkangle(l: LuaState, narg: c_int) -> crate::userdata::Angle {
		unsafe { *( (*luaL_checkudata(l, narg, cstr!("Angle"))).data as *mut _) }
	};
}
/// Pushes a vector onto the stack
/// # Example
/// Creates a LuaCFunction that will take three number arguments and return a glua Vector type.
/// ```rust
/// use rglua::prelude::*;
/// #[lua_function]
/// fn new_vector(l: LuaState) -> i32 {
///     let x = luaL_checknumber(l, 1) as f32;
///     let y = luaL_checknumber(l, 2) as f32;
///     let z = luaL_checknumber(l, 3) as f32;
///     lua_pushvector(l, Vector::new(x, y, z));
///     // Return one value -- the new vector
///     1
/// }
/// ```
pub fn lua_pushvector(l: LuaState, v: Vector) {
	let ptr = lua_newuserdata(l, std::mem::size_of::<Userdata>());

	// I am an actual maniac for doing this
	unsafe {
		let ty = std::ptr::addr_of_mut!((*ptr).typ);
		ty.write(LuaType::Vector);

		let data = std::ptr::addr_of_mut!((*ptr).data);
		// FIXME: This may leak memory.. need to make sure lua actually cleans it up.
		// I am assuming this will be fine since Vectors are primitive and this is lua managed userdata.
		data.write(Box::into_raw(Box::new(v)) as *mut c_void);
	}

	luaL_getmetatable(l, cstr!("Vector"));
	lua_setmetatable(l, -2);
}

/// Pushes an angle onto the stack.
pub fn lua_pushangle(l: LuaState, v: Angle) {
	let ptr = lua_newuserdata(l, std::mem::size_of::<Userdata>());

	unsafe {
		let ty = std::ptr::addr_of_mut!((*ptr).typ);
		ty.write(LuaType::Angle);

		let data = std::ptr::addr_of_mut!((*ptr).data);
		data.write(Box::into_raw(Box::new(v)) as *mut c_void);
	}

	luaL_getmetatable(l, cstr!("Angle"));
	lua_setmetatable(l, -2);
}

#[inline(always)]
#[allow(non_snake_case)]
/// Tries to see if the given value at index ``arg`` is nil or none, if so, returns ``default`` value.
/// Otherwise, runs ``func``, passing the lua state and arg indent and returns the value of that
/// # Returns
/// Type ``T`` either from the function or default value.
pub fn luaL_opt<T, F: Fn(LuaState, c_int) -> T>(l: LuaState, arg: c_int, default: T, func: F) -> T {
	if lua_isnoneornil(l, arg) {
		default
	} else {
		func(l, arg)
	}
}

/// This function works like luaL_checkudata, except that, when the test fails, it returns None instead of throwing an error.
/// Adapted from Lua 5.3, note this does not actually exist in gluajit
#[allow(non_snake_case)]
pub fn luaL_testudata(l: LuaState, arg: c_int, tname: LuaString) -> Option<*mut super::Userdata> {
	if lua_isuserdata(l, arg) == 1 {
		lua_getmetatable(l, arg); // Object metatable
		luaL_getmetatable(l, tname); // Desired global metatable
		if lua_rawequal(l, -1, -2) == 1 {
			return lua_touserdata(l, arg).map(|ud| ud as *mut super::Userdata);
		}
	}
	None
}

#[inline(always)]
#[allow(non_snake_case)]
pub fn lua_tovector<'a>(l: LuaState, i: c_int) -> Option<&'a mut Vector> {
	luaL_testudata(l, i, cstr!("Vector"))
		.map(|x: *mut Userdata| unsafe { &mut *(x as *mut Vector) })
}
