use super::cell::Cell;
use super::jsvalue::*;
use cgc::api::Handle;
/// The property descriptor.
pub struct JSProperty {
    /// Location inside inline slots of an object.
    ///
    /// To change property use `property.base.store_direct(property.slot,your_value)`.
    /// And to load property use `property.base.direct(property.slot)`.
    pub slot: u32,
    /// Base object that contains this property.
    pub base: Option<Handle<Cell>>,
    pub getter: Option<JSValue>,
    pub setter: Option<JSValue>,
    pub configurable: Option<bool>,
    pub enumerable: Option<bool>,
    pub writable: Option<bool>,
}

impl JSProperty {
    pub fn new() -> Self {
        Self {
            slot: u32::max_value(),
            base: None,
            getter: None,
            setter: None,
            configurable: None,
            enumerable: None,
            writable: None,
        }
    }

    pub fn enumerable(mut self, enumerable: bool) -> Self {
        self.enumerable = Some(enumerable);
        self
    }

    pub fn writable(mut self, writable: bool) -> Self {
        self.writable = Some(writable);
        self
    }

    pub fn configurable(mut self, configurable: bool) -> Self {
        self.configurable = Some(configurable);
        self
    }

    pub fn slot(mut self, slot: u32) -> Self {
        self.slot = slot;
        self
    }

    pub fn base(mut self, base: Handle<Cell>) -> Self {
        self.base = Some(base);
        self
    }

    pub fn getter(mut self, get: JSValue) -> Self {
        self.getter = Some(get);
        self
    }

    pub fn setter(mut self, set: JSValue) -> Self {
        self.setter = Some(set);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.getter.is_none()
            && self.setter.is_none()
            && self.enumerable.is_none()
            && self.writable.is_none()
            && self.configurable.is_none()
    }

    pub fn is_accessor(&self) -> bool {
        self.getter.is_some() || self.setter.is_some()
    }

    pub fn is_data(&self) -> bool {
        self.slot != u32::max_value() || self.writable.is_some()
    }

    pub fn is_generic(&self) -> bool {
        !self.is_accessor() && !self.is_data()
    }
}

impl Default for JSProperty {
    fn default() -> Self {
        Self::new()
    }
}
