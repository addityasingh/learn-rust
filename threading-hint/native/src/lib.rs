#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::vm::{Call, JsResult};
use neon::js::JsString;
use neon::js::JsNumber;

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}

fn threading_hint(call: Call) -> JsResult<JsNumber> {
    Ok(JsNumber::new(call.scope, num_cpus::get() as f64))
}

register_module!(m, {
    m.export("threading_hint", threading_hint)
});
