#[macro_use]
extern crate neon;
extern crate crypto;

use neon::prelude::*;
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;

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

fn fibonacci(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value() as usize;
    let (mut a, mut b) = (1, 1);
    for _i in 0..n {
        let sum = a + b;
        a = b;
        b = sum;
    }
    Ok(cx.number(b))
}

register_module!(mut cx, {
    cx.export_function("sum", sum)?;
    cx.export_function("sha1", sha1)?;
    cx.export_function("fibonacci", fibonacci)
});
