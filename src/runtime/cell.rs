use cgc::api::{Finalizer, Traceable, Tracer};

pub enum Cell {
    String(String),
    Object(super::jsobject::JSObject),
}
impl Cell {
    pub fn unchecked_object_mut(&mut self) -> &mut super::jsobject::JSObject {
        match self {
            Cell::Object(ob) => ob,
            _ => unreachable!(),
        }
    }
}

impl Traceable for Cell {
    fn trace_with(&self, tracer: &mut Tracer) {
        match self {
            Self::Object(obj) => obj.trace_with(tracer),
            _ => (),
        }
    }
}

impl Finalizer for Cell {}
