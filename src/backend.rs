pub(crate) mod backend_type;
pub mod llvm;

pub use backend_type::{BackendType, BACKEND_TYPES};

trait CompileBackend {
    fn compile();
}

trait JITBackend {
    fn jit(&mut self, ast: &[crate::ast::Ast]);
}
