use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex};

// Types are in camelcase + C prefix
pub type CVoid = std::ffi::c_void;

pub type CChar = i8;
pub type CDouble = f64; // All lua numbers are doubles in Lua 5.1 (Glua)

pub type LuaNumber = CDouble;
pub type LuaState = *mut CVoid; // Raw Lua state.

// C Functions, you shouldn't have to use these in your modules as long as we make wrappers
extern {
    // Get
    fn glua_get_string(state: LuaState, stack_pos: i32) -> *const CChar;
    fn glua_get_number(state: LuaState, stack_pos: i32) -> CDouble;

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
/// let nullptr: *mut CVoid = std::ptr::null_mut();
///
/// let rlua_state = LuaState::new( nullptr ); // Fake Lua State object made from a null mutable ptr.
/// let mut api = rlua_state.get_threadsafe();
/// for _ in 1..10 {
///     let safe_instance = api.get_clone();
///     std::thread::spawn(move || {
///         let rluastate = safe_instance.lock().unwrap();
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
/// ```
/// // This is an example of a simple gmod13_open function.
/// #[no_mangle]
/// pub extern fn gmod13_open(state: LuaState) -> c_int {
///     let mut rluastate = RLuaState::new(state);
///     printgm!(rluastate,"Hello from rust!");
/// }
/// ```
impl RLuaState {
    pub fn get_number(&mut self, stack_pos: i32) -> CDouble {
        unsafe {
            glua_get_number(self.raw_state,stack_pos)
        }
    }

    /// This is actually luaL_check_string.
    pub fn get_string(&mut self, stack_pos: i32) -> String {
        unsafe {
            let glua_chars = glua_get_string(self.raw_state,stack_pos);
            CStr::from_ptr(glua_chars).to_string_lossy().into_owned()
        }
    }

    pub fn get_field(&mut self, stack_pos: i32, key: &str) {
        unsafe {
            glua_get_field( self.raw_state, stack_pos, CString::new(key).unwrap().as_ptr() )
        }
    }

    pub fn get_global(&mut self, key: &str) {
        self.get_field(-10002,key)
    }

    pub fn get_userdata(&mut self, stack_pos: i32) {
        unsafe {
            glua_get_userdata(self.raw_state,stack_pos)
        }
    }
}

/// Lua Push Functions
impl RLuaState {
    pub fn push_number(&mut self, num: i32) {
        unsafe {
            glua_push_number(self.raw_state,num as CDouble)
        }
    }
    pub fn push_string(&mut self, string: &str) {
        unsafe {
            glua_push_string(self.raw_state,CString::new(string).unwrap().as_ptr());
        }
    }
    pub fn push_cfunc(&mut self, func: extern fn(LuaState) -> i32) {
        unsafe {
            glua_push_cfunc(self.raw_state,func)
        }
    }
    pub fn push_global(&mut self) {
        unsafe {
            glua_push_global(self.raw_state)
        }
    }
}

/// Lua Set Functions
impl RLuaState {
    pub fn set_global(&mut self, key: &str, func: extern fn(LuaState) -> i32) {
        self.push_global();
        self.push_string(key);
        self.push_cfunc(func);
        self.set_table(-3);
    }
    pub fn set_table(&mut self, stack_pos: i32) {
        unsafe {
            glua_set_table(self.raw_state,stack_pos)
        }
    }
}

/// Misc Lua Functions
impl RLuaState {
    pub fn call(&mut self, nargs: i32, nresults: i32) {
        unsafe {
            glua_call(self.raw_state,nargs,nresults)
        }
    }
}


#[allow(unused_macros)]
#[macro_export]
/// Like println!, however it prints to the gmod server's console.
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