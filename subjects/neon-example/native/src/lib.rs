#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn sum(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value();
    let b = cx.argument::<JsNumber>(1)?.value();
    Ok(cx.number(a + b))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("sum", sum)
});
