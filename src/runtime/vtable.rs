use super::*;
use jsobject::*;
use jsproperty::*;
pub struct VTable {
    pub get_own_property:
        fn(rt: &mut Runtime, object: &mut JSObject, name: &str) -> Option<JSProperty>,
    pub get_property: fn(rt: &mut Runtime, object: &mut JSObject, name: &str) -> Option<JSProperty>,
    pub get: fn(rt: &mut Runtime, object: &mut JSObject, name: &str) -> Option<JSValue>,
    pub can_put: fn(rt: &mut Runtime, object: &JSObject, name: &str) -> bool,
    pub put: fn(rt: &mut Runtime, object: &mut JSObject, name: &str, val: JSValue, s: bool),
}
