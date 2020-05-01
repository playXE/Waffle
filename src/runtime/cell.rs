use cgc::api::{Finalizer, Traceable, Tracer};

pub enum Cell {
    String(String),
}

impl Traceable for Cell {
    fn trace_with(&self, _: &mut Tracer) {}
}

impl Finalizer for Cell {}
