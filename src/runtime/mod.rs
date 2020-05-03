pub mod cell;
pub mod environment;
pub mod jsobject;
pub mod jsproperty;
pub mod jsvalue;
pub mod pure_nan;

use cgc::heap::Heap;
use jsobject::*;
use jsproperty::*;
use jsvalue::*;
pub struct Runtime {
    pub heap: Heap,
    pub number: JSValue,
    pub boolean: JSValue,
    pub math: JSValue,
    pub object: JSValue,
    pub string: JSValue,
    pub symbol: JSValue,
    pub global: JSValue,
}

impl Runtime {
    pub fn execute(&mut self, _: JSValue, _: JSValue, _: &[JSValue]) -> Result<JSValue, JSValue> {
        Ok(JSValue::undefined())
    }
}
