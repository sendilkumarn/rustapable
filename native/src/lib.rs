#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::{JsObject, Object};
use neon::mem::Handle;

fn copy_properties(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;
    let from: Handle<JsObject> = try!(try!(call.arguments.require(scope, 0)).check::<JsObject>());
    let to: Handle<JsObject> = try!(try!(call.arguments.require(scope, 1)).check::<JsObject>());
    let keys = try!(try!(from.get_own_property_names(scope)).to_vec(scope));
    for key in keys {
        to.set(key, from.get(scope, key).unwrap());
    }
    Ok(to)
}

register_module!(m, {
    m.export("copyProperties", copy_properties)
});
