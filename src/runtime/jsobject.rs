use super::cell::*;
use super::jsproperty::*;
use super::jsvalue::*;

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
