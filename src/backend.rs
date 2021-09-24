pub(crate) mod backend_type;
pub mod llvm;

pub use backend_type::{BackendType, BACKEND_TYPES};

pub trait CompileBackend {
    fn compile(&mut self, ast: &[crate::ast::Ast]);
}

pub trait JITBackend {
    fn jit(&mut self, ast: &[crate::ast::Ast]);
}
