use neon::prelude::*;
//use neon::prelude::JsFunction;
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
