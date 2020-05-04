use super::*;
use jsobject::*;
use jsproperty::*;
pub struct VTable {
    pub get_own_property: fn(rt: &mut Runtime, _: JSValue, name: &str) -> Option<JSProperty>,
    pub get_property: fn(rt: &mut Runtime, _: JSValue, name: &str) -> Option<JSProperty>,
    pub get: fn(rt: &mut Runtime, _: JSValue, name: &str) -> Result<JSValue, JSValue>,
    pub can_put: fn(rt: &mut Runtime, _: JSValue, name: &str) -> bool,
    pub put: fn(rt: &mut Runtime, _: JSValue, name: &str, val: JSValue, s: bool),
    pub has_property: fn(rt: &mut Runtime, _: JSValue, name: &str) -> bool,
    pub has_own_property: fn(rt: &mut Runtime, _: JSValue, name: &str) -> bool,
    pub define_own_property:
        fn(rt: &mut Runtime, _: JSValue, _: &str, _: JSProperty, _: bool) -> bool,
    pub delete: fn(rt: &mut Runtime, _: JSValue, _: &str, _: bool) -> bool,
    pub enumerate: fn(rt: &mut Runtime, _: JSValue, _: bool, _: fn(&mut Runtime, &str) -> bool),
}

pub fn object_enumerate(
    rt: &mut Runtime,
    this: &mut JSObject,
    all: bool,
    each: fn(&mut Runtime, &str) -> bool,
) {
    for (name, prop) in this.property.iter() {
        if all || prop.enumerable.unwrap_or(false) {
            if !each(rt, name) {
                return;
            }
        }
    }
}

pub fn object_get_own_property(
    _rt: &mut Runtime,
    object: &mut JSObject,
    name: &str,
) -> Option<JSProperty> {
    object._read(name)
}

pub fn object_get_property(rt: &mut Runtime, object: JSValue, name: &str) -> Option<JSProperty> {
    let mut c = object.get_object(rt).as_cell();
    let obj = c.unchecked_object_mut();
    if let Some(p) = (obj.class_object.get_own_property)(rt, object, name) {
        return Some(p);
    } else if let Some(proto) = obj.prototype {
        return (proto
            .get_object(rt)
            .as_cell()
            .get_mut()
            .unchecked_object_mut()
            .class_object
            .get_property)(rt, proto.get_object(rt), name);
    } else {
        None
    }
}

pub fn object_get(rt: &mut Runtime, object: JSValue, name: &str) -> Result<JSValue, JSValue> {
    let mut c = object.get_object(rt).as_cell();
    let obj = c.unchecked_object_mut();
    let prop = (obj.class_object.get_property)(rt, object, name);
    if let Some(prop) = prop {
        if prop.is_accessor() {
            return rt.execute(prop.getter.unwrap(), object, &[]);
        } else {
            Ok(prop.value.unwrap_or(JSValue::undefined()))
        }
    } else {
        Ok(JSValue::undefined())
    }
}
