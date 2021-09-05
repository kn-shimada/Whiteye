use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::PointerValue;
use inkwell::OptimizationLevel;
use log::info;
use std::collections::HashMap;

use crate::ast::Ast;
use crate::backend::llvm::function_code_generator::FunctionCodeGenerator;

mod function_code_generator;

pub struct LLVMBackend<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> LLVMBackend<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let variables = HashMap::new();

        Self {
            context,
            module,
            builder,
            variables,
        }
    }

    pub fn run_jit(&mut self, ast: &[Ast]) {
        self.generate(ast);

        let execution_engine = self
            .module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .unwrap();

        unsafe {
            let jit_fn = execution_engine
                .get_function::<unsafe extern "C" fn() -> i64>("main")
                .unwrap();

            println!("Return: {}", jit_fn.call());
        };
    }

    fn generate(&mut self, ast: &[Ast]) {
        let main_fn_type = self.context.i32_type().fn_type(&[], false);
        let main_function = self.module.add_function("main", main_fn_type, None);

        let entry_basic_block = self.context.append_basic_block(main_function, "entry");
        self.builder.position_at_end(entry_basic_block);

        for a in ast {
            let function_code_generator = FunctionCodeGenerator::new(
                self.context,
                &self.builder,
                &self.module,
                &mut self.variables,
            );

            function_code_generator.generate(a.clone());
        }

        info!("IR:\n{}", self.module.print_to_string().to_string());
    }
}
