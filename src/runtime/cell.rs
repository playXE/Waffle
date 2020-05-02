use cgc::api::{Finalizer, Traceable, Tracer};

pub enum Cell {
    String(String),
    Object(super::jsobject::JSObject),
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
