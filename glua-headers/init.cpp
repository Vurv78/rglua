#define GMMODULE

// You can find the source for the header files (.h files) here:
// https://github.com/Facepunch/gmod-module-base, they were not created by me.

#include "Interface.h"
#include <stdio.h>

using namespace GarrysMod::Lua;

extern "C" {
	// Get
	const char* glua_get_string(lua_State* state, int stackPos) {
		return LUA->CheckString(stackPos);
	}
	double glua_get_number(lua_State* state, int stackPos) {
		return LUA->CheckNumber(stackPos);
	}

	void glua_get_field(lua_State* state, int stackPos, char* fieldName) {
		return LUA->GetField(stackPos,fieldName);
	}

	void* glua_get_userdata(lua_State* state, int stackPos) {
		return LUA->GetUserdata(stackPos);
	}

	// Push
	void glua_push_string(lua_State* state, char* string) {
		LUA->PushString(string);
	}
	void glua_push_number(lua_State* state, double number) {
		LUA->PushNumber(number);
	}

	void glua_push_global(lua_State* state) {
		LUA->PushSpecial(GarrysMod::Lua::SPECIAL_GLOB);
	}
	void glua_push_cfunc(lua_State* state, int (*f)(lua_State*)) {
		LUA->PushCFunction(f);
	}

	// Set
	void glua_set_table(lua_State* state, int stackPos) {
		LUA->SetTable(stackPos);
	}

	// Call
	void glua_call(lua_State* state, int nargs, int nresults) {
		LUA->Call(nargs,nresults);
	}
}