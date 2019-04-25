#[macro_use]
extern crate neon;
extern crate crypto;

use neon::prelude::*;
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn sum(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value();
    let b = cx.argument::<JsNumber>(1)?.value();
    Ok(cx.number(a + b))
}

fn sha1(mut cx: FunctionContext) -> JsResult<JsString> {
    let data = cx.argument::<JsString>(0)?.value();
    let mut hasher = Sha1::new();
    hasher.input_str(&data);
    let hex = hasher.result_str();
    Ok(cx.string(hex))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("sum", sum)?;
    cx.export_function("sha1", sha1)
});
