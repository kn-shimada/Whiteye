use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, FloatType, IntType, PointerType};
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::{AddressSpace, OptimizationLevel};
use log::info;
use std::collections::HashMap;

use crate::ast::Ast;
use crate::backend::llvm::function_code_generator::FunctionCodeGenerator;

mod function_code_generator;

#[allow(dead_code)]
pub struct LLVMTypes<'a> {
    i8_type: IntType<'a>,
    i32_type: IntType<'a>,
    i64_type: IntType<'a>,
    f64_type: FloatType<'a>,
    i8_ptr_type: PointerType<'a>,
}

pub(crate) struct ExternFunctions<'a> {
    printf: FunctionValue<'a>,
}

pub struct LLVMBackend<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    ty: LLVMTypes<'ctx>,
    fns: ExternFunctions<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> LLVMBackend<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let variables = HashMap::new();

        let ty = LLVMTypes {
            i8_type: context.i8_type(),
            i32_type: context.i32_type(),
            i64_type: context.i64_type(),
            f64_type: context.f64_type(),
            i8_ptr_type: context.i8_type().ptr_type(AddressSpace::Generic),
        };

        let printf_fn_type = ty.i32_type.fn_type(
            &[context
                .i8_type()
                .ptr_type(inkwell::AddressSpace::Generic)
                .as_basic_type_enum()],
            false,
        );
        let printf_fn = module.add_function("printf", printf_fn_type, None);

        let fns = ExternFunctions { printf: printf_fn };

        Self {
            context,
            module,
            builder,
            ty,
            fns,
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
                .get_function::<unsafe extern "C" fn() -> ()>("main")
                .unwrap();

            jit_fn.call();
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
                &self.ty,
                &self.fns,
                &mut self.variables,
            );

            function_code_generator.generate(a.clone());
        }

        info!("IR:\n{}", self.module.print_to_string().to_string());
    }
}
