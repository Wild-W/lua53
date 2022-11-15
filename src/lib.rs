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

    fn js_load_libs(mut cx: FunctionContext) -> JsResult<JsUndefined>
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

    fn js_load_lib(mut cx: FunctionContext) -> JsResult<JsUndefined>
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

    fn js_get_top(mut cx: FunctionContext) -> JsResult<JsNumber>
    {
        let index = cx.this().downcast_or_throw::<JsBox<LuaState>, _>(&mut cx)?
            .state.borrow_mut().get_top();

        Ok(cx.number(index))
    }

    fn js_push_number(mut cx: FunctionContext) -> JsResult<JsUndefined>
    {
        let num = cx.argument::<JsNumber>(0)?.value(&mut cx);

        cx.this().downcast_or_throw::<JsBox<LuaState>, _>(&mut cx)?
            .state.borrow_mut().push_number(num);
        
        Ok(cx.undefined())
    }

    fn js_set_global(mut cx: FunctionContext) -> JsResult<JsUndefined>
    {
        let name = cx.argument::<JsString>(0)?.value(&mut cx);

        cx.this().downcast_or_throw::<JsBox<LuaState>, _>(&mut cx)?
            .state.borrow_mut().set_global(name.as_str());

        Ok(cx.undefined())
    }

    fn js_create_global(mut cx: FunctionContext) -> JsResult<JsUndefined>
    {
        let name = cx.argument::<JsString>(0)?.value(&mut cx);
        let value = cx.argument::<JsValue>(1)?;

        let temp = cx.this().downcast_or_throw::<JsBox<LuaState>, _>(&mut cx)?;
        let mut mut_state = temp.state.borrow_mut();

        if value.is_a::<JsBoolean, _>(&mut cx)
        {
            mut_state.push_bool(value.downcast_or_throw::<JsBoolean, _>(&mut cx)?.value(&mut cx));
        }
        else if value.is_a::<JsNumber, _>(&mut cx)
        {
            mut_state.push_number(value.downcast_or_throw::<JsNumber, _>(&mut cx)?.value(&mut cx));
        }
        else if value.is_a::<JsString, _>(&mut cx)
        {
            mut_state.push_string(value.downcast_or_throw::<JsString, _>(&mut cx)?.value(&mut cx).as_str());
        }
        // else if value.is_a::<JsFunction, _>(&mut cx)
        // {
        //     let func = value.downcast_or_throw::<JsFunction, _>(&mut cx)?;
        // }
        else { return Ok(cx.undefined()); }

        mut_state.set_global(name.as_str());

        Ok(cx.undefined())
    }

    // fn js_get_global(mut cx: FunctionContext) -> JsResult<JsUndefined>
    // {
    //     let name = cx.argument::<JsString>(0)?.value(&mut cx);
    //     let binding = cx.this().downcast_or_throw::<JsBox<LuaState>, _>(&mut cx)?;
    //     let mut state = binding.state.borrow_mut();

    //     let lua_val = state.get_global(name.as_str());
    //     lua::ffi::lua_get
    //     println!("{:?}", lua_val);

    //     Ok(cx.undefined())
    // }

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

    cx.export_function("luaOpenLibs", LuaState::js_load_libs)?;
    cx.export_function("luaOpenLib", LuaState::js_load_lib)?;

    cx.export_function("luaGetTop", LuaState::js_get_top)?;
    //cx.export_function("luaGetGlobal", LuaState::js_get_global)?;

    cx.export_function("luaPushNumber", LuaState::js_push_number)?;
    cx.export_function("luaSetGlobal", LuaState::js_set_global)?;
    cx.export_function("luaCreateGlobal", LuaState::js_create_global)?;

    cx.export_function("luaDoString", LuaState::js_do_string)?;
    cx.export_function("luaDofile", LuaState::js_do_file)?;

    Ok(())
}
