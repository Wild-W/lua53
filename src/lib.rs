use std::cell::RefCell;

use neon::prelude::*;
extern crate lua;

fn lua_run(mut cx: FunctionContext) -> JsResult<JsUndefined>
{
    let str = cx.argument::<JsString>(0)?.value(&mut cx);
    let libs = cx.argument_opt(1);

    let mut state = lua::State::new();

    if let Some(libs) = libs
    {
        let libs_vec = libs.downcast_or_throw::<JsArray, _>(&mut cx)?.to_vec(&mut cx).unwrap();
        for lib in libs_vec
        {
            use lua::ffi::*;
    
            let lib_temp = lib.downcast::<JsString, _>(&mut cx).unwrap().value(&mut cx);
            let lib_str = lib_temp.as_str();
    
            match lib_str
            {
                "base" => state.requiref("_G", Some(luaopen_base), true),
                "bit32" => state.requiref("bit32", Some(luaopen_bit32), true),
                "coroutine" => state.requiref("coroutine", Some(luaopen_coroutine), true),
                "debug" => state.requiref("debug", Some(luaopen_debug), true),
                "io" => state.requiref("io", Some(luaopen_io), true),
                "math" => state.requiref("math", Some(luaopen_math), true),
                "os" => state.requiref("os", Some(luaopen_os), true),
                "package" => state.requiref("package", Some(luaopen_package), true),
                "string" => state.requiref("string", Some(luaopen_string), true),
                "table" => state.requiref("table", Some(luaopen_table), true),
                "utf8" => state.requiref("utf8", Some(luaopen_utf8), true),
                _ => ()
            }
        }
    }
    else
    {
        state.open_libs();
    }

    let _thread = state.do_string(&str);

    Ok(cx.undefined())
}

fn util_lua_load(state: &mut std::cell::RefMut<lua::State>, lib: &str)
{
    use lua::ffi::*;

    match lib
    {
        "base" => state.requiref("_G", Some(luaopen_base), true),
        "bit32" => state.requiref("bit32", Some(luaopen_bit32), true),
        "coroutine" => state.requiref("coroutine", Some(luaopen_coroutine), true),
        "debug" => state.requiref("debug", Some(luaopen_debug), true),
        "io" => state.requiref("io", Some(luaopen_io), true),
        "math" => state.requiref("math", Some(luaopen_math), true),
        "os" => state.requiref("os", Some(luaopen_os), true),
        "package" => state.requiref("package", Some(luaopen_package), true),
        "string" => state.requiref("string", Some(luaopen_string), true),
        "table" => state.requiref("table", Some(luaopen_table), true),
        "utf8" => state.requiref("utf8", Some(luaopen_utf8), true),
        _ => ()
    }
}

struct LuaState
{
    state: RefCell<lua::State>,
}

impl Finalize for LuaState {}

impl LuaState
{
    fn js_new(mut cx: FunctionContext) -> JsResult<JsBox<LuaState>>
    {
        Ok(cx.boxed(LuaState {state: RefCell::new(lua::State::new())}))
    }

    fn js_open_libs(mut cx: FunctionContext) -> JsResult<JsUndefined>
    {
        let libs = cx.argument_opt(0);
        let binding = cx.this()
            .downcast_or_throw::<JsBox<LuaState>, _>(&mut cx)?;
        let mut state = binding
            .state.borrow_mut();

        if let Some(libs) = libs
        {
            let libs_vec = libs.downcast_or_throw::<JsArray, _>(&mut cx)?.to_vec(&mut cx).unwrap();
            for lib in libs_vec
            {
                let lib_temp = lib.downcast::<JsString, _>(&mut cx).unwrap().value(&mut cx);
                let lib_str = lib_temp.as_str();

                util_lua_load(&mut state, lib_str);
            }
        }
        else
        {
            state.open_libs();
        }

        Ok(cx.undefined())
    }

    fn js_open_lib(mut cx: FunctionContext) -> JsResult<JsUndefined>
    {        
        let lib = cx.argument::<JsString>(0)?.value(&mut cx);
        let lib_str = lib.as_str();

        let binding = cx.this()
            .downcast_or_throw::<JsBox<LuaState>, _>(&mut cx)?;
        let mut state = binding
            .state.borrow_mut();

        util_lua_load(&mut state, lib_str);

        Ok(cx.undefined())
    }

    fn js_do_string(mut cx: FunctionContext) -> JsResult<JsUndefined>
    {
        let str = cx.argument::<JsString>(0)?.value(&mut cx);

        let _thread = cx.this()
            .downcast_or_throw::<JsBox<LuaState>, _>(&mut cx)?
            .state.borrow_mut().do_string(str.as_str());

        Ok(cx.undefined())
    }

    fn js_do_file(mut cx: FunctionContext) -> JsResult<JsUndefined>
    {
        let str = cx.argument::<JsString>(0)?.value(&mut cx);

        let _thread = cx.this()
            .downcast_or_throw::<JsBox<LuaState>, _>(&mut cx)?
            .state.borrow_mut().do_file(str.as_str());

        Ok(cx.undefined())
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()>
{
    cx.export_function("luaRun", lua_run)?;

    cx.export_function("luaNew", LuaState::js_new)?;

    cx.export_function("luaOpenLibs", LuaState::js_open_libs)?;
    cx.export_function("luaOpenLib", LuaState::js_open_lib)?;

    cx.export_function("luaDoString", LuaState::js_do_string)?;
    cx.export_function("luaDofile", LuaState::js_do_file)?;

    Ok(())
}
