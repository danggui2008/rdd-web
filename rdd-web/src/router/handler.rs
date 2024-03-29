use crate::context::Context;

pub type Handler = dyn Fn(&mut Context) + Send + Sync + 'static;
