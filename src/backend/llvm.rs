use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::IntValue;
use inkwell::values::PointerValue;
use inkwell::OptimizationLevel;
use std::collections::HashMap;

use crate::ast::Ast;
use crate::ast::Value;

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

    pub fn run_jit(&self, ast: &[Ast]) {
        self.generate_code(ast);

        let execution_engine = self
            .module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .unwrap();

        unsafe {
            execution_engine
                .get_function::<unsafe extern "C" fn()>("main")
                .unwrap()
                .call();
        };
    }

    fn generate_code(&self, ast: &[Ast]) {
        let printf_fn_type = self
            .context
            .i32_type()
            .fn_type(&[self.context.i8_type().into()], true);
        let printf_function = self.module.add_function("printf", printf_fn_type, None);

        let main_fn_type = self.context.i32_type().fn_type(&[], false);
        let main_function = self.module.add_function("main", main_fn_type, None);

        let entry_basic_block = self.context.append_basic_block(main_function, "entry");
        self.builder.position_at_end(entry_basic_block);

        for a in ast {
            match a {
                Ast::Literal(value) => match value {
                    &Value::Integer(v) => {
                        let hw_string_ptr =
                            self.builder.build_global_string_ptr(&v.to_string(), "");

                        self.builder.build_call(
                            printf_function,
                            &[hw_string_ptr.as_pointer_value().into()],
                            "call",
                        );
                    }
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }

        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, false)));
    }

    // TODO: Decide bit length
    fn generate_int_code(&self, v: isize) -> IntValue {
        self.context
            .i64_type()
            .const_int(v.abs() as u64, v.is_positive())
    }
}
