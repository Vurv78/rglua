extern crate rglua;

#[test]
fn thread_test() {
    use rglua::{RLuaState,LuaState};

    #[no_mangle]
    pub unsafe extern fn gmod13_open(state: LuaState) -> i32 {
        let wrapped = RLuaState::new(state);
        let mut ts = wrapped.get_threadsafe();

        let copy = ts.get_clone();
        std::thread::spawn(move || {
            let mut state = copy.lock().unwrap();
            state.get_global(&"print");
            state.push_string(&"This is from a thread!");
            state.call(1,0);
        });

        let copy = ts.get_clone();
        let mut state = copy.lock().unwrap();
        state.get_global(&"print");
        state.push_string(&"This is outside the thread!");
        state.call(1,0);
        0
    }

    #[no_mangle]
    pub unsafe extern fn gmod13_close(state: LuaState) -> i32 {
        let _wrapped = RLuaState::new(state);
        0
    }
}