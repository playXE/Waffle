use super::cell::*;
use super::jsproperty::*;
use super::jsvalue::*;
use cgc::api::{Finalizer, Traceable, Tracer};
use fxhash::*;
use std::collections::HashMap;
pub trait ObjectTrait {
    fn get_own_property(&self, key: &JSValue) -> JSProperty;
    fn define_own_property(&mut self, key: String, prop: JSProperty) -> bool;
    fn put_property(&mut self, name: String, prop: JSProperty);
    fn remove_property(&mut self, name: &str);
    fn get_internal(&self, _: &str) -> JSValue;
    fn set_internal(&mut self, _: &str, value: JSValue);
    /// [[[GetPrototypeOf]]](https://tc39.es/ecma262/#sec-ordinary-object-internal-methods-and-internal-slots-getprototypeof)
    ///
    /// If object has prototype returns prototype otherwise returns JSValue::null()
    fn get_prototype_of(&self) -> JSValue {
        self.get_internal("prototype")
    }

    fn set_prototype_of(&mut self, to: JSValue) -> bool;
}

/// Representation of JS object.
///
/// TODO: Implement hidden classes for properties.
pub struct JSObject {
    pub kind: JSObjectKind,
    pub internal: FxHashMap<String, JSValue>,
    pub property: hashlink::LinkedHashMap<String, JSProperty>,
    pub prototype: Option<JSValue>,
    pub extensible: bool,
    pub class: String,
    pub class_object: &'static super::vtable::VTable,
}

impl JSObject {
    pub fn is_array(&self) -> bool {
        match self.kind {
            JSObjectKind::Array(_) => true,
            _ => false,
        }
    }

    pub(crate) fn _read(&self, name: &str) -> Option<JSProperty> {
        self.property.get(name).cloned()
    }
    pub(crate) fn _write(&mut self, name: &str, prop: JSProperty) {
        if self.property.contains_key(name) {
            self.property[name] = prop;
        } else {
            self.property.insert(name.to_owned(), prop);
        }
    }

    pub(crate) fn _exist(&self, name: &str) -> bool {
        self.property.contains_key(name)
    }
}

pub enum JSObjectKind {
    Array(Vec<JSValue>),
    String,
    Symbol,
    Error,
    Boolean,
    Number,
    Normal,
}

impl Traceable for JSObject {
    fn trace_with(&self, tracer: &mut Tracer) {
        match &self.kind {
            JSObjectKind::Array(array) => {
                array.trace_with(tracer);
            }
            _ => (),
        }
    }
}
impl Finalizer for JSObject {}
