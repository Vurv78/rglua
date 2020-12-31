use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex};

// Types are in camelcase + C prefix
pub type CVoid = std::ffi::c_void;

pub type CChar = i8;
pub type CDouble = f64; // All lua numbers are doubles in Lua 5.1 (Glua)

pub type LuaNumber = CDouble;
pub type LuaState = *mut CVoid; // Raw Lua state.

// Extern raw functions that shouldn't need to be used by the lib user.
extern {
    // Get (Throws errors if given incorrect type)
    fn glua_check_string(state: LuaState, stack_pos: i32) -> *const CChar;
    fn glua_check_number(state: LuaState, stack_pos: i32) -> CDouble;

    // Get (Misc)
    fn glua_get_field(state: LuaState, stack_pos: i32, string: *const CChar);

    fn glua_get_userdata(state: LuaState, stack_pos: i32);

    // Push
    fn glua_push_string(state: LuaState, string: *const CChar);
    fn glua_push_number(state: LuaState, n: CDouble);

    // Push (Special)
    fn glua_push_global(state: LuaState);
    fn glua_push_cfunc(state: LuaState, func: extern fn(LuaState) -> i32);

    // Set
    fn glua_set_table(state: LuaState, stack_pos: i32);

    fn glua_call(state: LuaState, nargs: i32, nresults: i32);
}

pub struct RLuaState {
    raw_state: LuaState,
}

pub struct RLSThreadable {
    rls_object: Arc< Mutex< RLuaState > >
}

/// This is the wrapper for the LuaState null ptr that we keep and use functions with.
/// Here is a basic example of how to use the wrapper in a binary module
/// ```
/// use rglua::{RLuaState,LuaState};
/// #[no_mangle]
/// unsafe extern fn gmod13_open(state: LuaState) -> i32 {
///     let mut wrapped = RLuaState::new(state);
///     // This is the same as doing 'printgm!(wrapped,"Hello from rust!")'
///     wrapped.get_global(&"print");
///     wrapped.push_string(&"Hello from rust!");
///     wrapped.call(1,0);
///     //printgm!(wrapped,"Also hello!");
///     0
/// }
/// #[no_mangle]
/// unsafe extern fn gmod13_close(state: LuaState) -> i32 {
///    let mut _wrapped = RLuaState::new(state);
///    //printgm!(_wrapped,"Goodbye!");
///    0
/// }
/// ```

impl RLuaState {
    pub fn new(state: LuaState) -> RLuaState {
        RLuaState {
            raw_state: state,
        }
    }

    pub fn get_threadsafe(self) -> RLSThreadable {
        RLSThreadable {
            rls_object: Arc::new(Mutex::new(self))
        }
    }
}

/// This is a struct that is returned from calling get_threadsafe on an RLuaState.
/// ```
/// use rglua::{RLuaState};
/// let nullptr = std::ptr::null_mut();
///
/// let rlua_state = RLuaState::new( nullptr ); // Fake Lua State object made from a null mutable ptr.
/// let mut api = rlua_state.get_threadsafe();
/// for _ in 1..10 {
///     let safe_instance = api.get_clone();
///     std::thread::spawn(move || {
///         let mut rluastate = safe_instance.lock().unwrap();
///         unsafe {
///             rluastate.get_global(&"print");
///             rluastate.push_string(&"Hello from a thread!");
///             rluastate.call(1,0);
///         };
///         // Do your multi-threaded stuff here
///     });
/// }
/// ```
impl RLSThreadable {
    pub fn get_clone(&mut self) -> Arc< Mutex< RLuaState > > {
        Arc::clone(&self.rls_object)
    }
}

unsafe impl Send for RLuaState {}

/// This is a wrapper for a traditional lua state that will allow easy access to rglua's library.
impl RLuaState {
    /// This is actually luaL_checknumber, which automatically throws an error if they don't provide the number.
    pub unsafe fn get_number(&mut self, stack_pos: i32) -> CDouble {
        glua_check_number(self.raw_state,stack_pos)
    }

    /// This is actually luaL_checkstring, which automatically throws an error if they don't provide the number.
    pub unsafe fn get_string(&mut self, stack_pos: i32) -> String {
        let glua_chars = glua_check_string(self.raw_state,stack_pos);
        CStr::from_ptr(glua_chars).to_string_lossy().into_owned()
    }

    pub unsafe fn get_field(&mut self, stack_pos: i32, key: &str) {
        glua_get_field( self.raw_state, stack_pos, CString::new(key).unwrap().as_ptr() )
    }

    pub unsafe fn get_global(&mut self, key: &str) {
        self.get_field(-10002,key)
    }

    pub unsafe fn get_userdata(&mut self, stack_pos: i32) {
        glua_get_userdata(self.raw_state,stack_pos)
    }
}

/// Lua Push Functions
impl RLuaState {
    pub unsafe fn push_number(&mut self, num: i32) {
        glua_push_number(self.raw_state,num as CDouble)
    }
    pub unsafe fn push_string(&mut self, string: &str) {
        glua_push_string(self.raw_state,CString::new(string).unwrap().as_ptr());
    }
    pub unsafe fn push_cfunc(&mut self, func: extern fn(LuaState) -> i32) {
        glua_push_cfunc(self.raw_state,func)
    }
    pub unsafe fn push_global(&mut self) {
        glua_push_global(self.raw_state)
    }
}

/// Lua Set Functions
impl RLuaState {
    pub unsafe fn set_global(&mut self, key: &str, func: extern fn(LuaState) -> i32) {
        self.push_global();
        self.push_string(key);
        self.push_cfunc(func);
        self.set_table(-3);
    }
    pub unsafe fn set_table(&mut self, stack_pos: i32) {
        glua_set_table(self.raw_state,stack_pos)
    }
}

/// Misc Lua Functions
impl RLuaState {
    pub unsafe fn call(&mut self, nargs: i32, nresults: i32) {
        glua_call(self.raw_state,nargs,nresults)
    }
}



#[allow(unused_macros)]
#[macro_export]
macro_rules! printgm {
    // First arg is the lua state.
    // Rest are varargs.
    // Can be either a variable storing a str literal, or a referenced String / str variable
    ($state:expr, $($x:expr),*) => {
        {
            let mut s = String::new();
            $( s.push_str($x); )* // Push every arg to the end string
            $state.get_global(&"print");
            $state.push_string(&s);
            // 1 arg, 0 results
            $state.call(1, 0);
        }
    };
}