#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::{JsArray, JsNull, JsUndefined, JsObject, Object, JsFunction, JsValue, JsNumber, JsBoolean};
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


fn fast_filter(mut call: Call) -> JsResult<JsArray>{
    let fun: Handle<JsFunction> = try!(try!(call.arguments.require(call.scope, 0)).check::<JsFunction>());
    let this: Handle<JsArray> = try!(try!(call.arguments.require(call.scope, 1)).check::<JsArray>());
    let arguments: Handle<JsValue> = try!(try!(call.arguments.require(call.scope, 2)).check::<JsValue>());

    let args: Vec<Handle<JsValue>> = vec![arguments];
    let this_arg = match args.len() {
        0...2 => JsUndefined::new().upcast::<JsValue>(),
        _ =>  args[1],
    };

    let len = this.len();
    let res: Handle<JsArray> = JsArray::new(call.scope, len);
    let mut index = 0;
    for i in 0..len {
      if  i as f64 <= args.len() as f64 {
        let val = this.get(call.scope, i).unwrap();
        let mut argument_list: Vec<_> = vec![];
        argument_list.push(this_arg);
        argument_list.push(val);
        argument_list.push(JsNumber::new(call.scope, (i-1) as f64).upcast::<JsValue>());
        argument_list.push(this.upcast::<JsValue>());
        let func_call = try!(fun.call(call.scope,
                                    JsNull::new(),
                                    argument_list.clone()))
                                    .check::<JsBoolean>().unwrap().value();
        if func_call  {
            res.set(index, val);
            index += 1;
        }
      }
    }

    Ok(res)
}

register_module!(m, {
    m.export("copyProperties", copy_properties)?;
    m.export("fastFilter", fast_filter)?;
    Ok(())
});
