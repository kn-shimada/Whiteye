use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::BasicValue;
use inkwell::values::BasicValueEnum;
use inkwell::values::PointerValue;
use std::collections::HashMap;

use crate::ast::Ast;
use crate::ast::ExprOpKind;
use crate::value::Value;

#[allow(dead_code)]
pub(crate) struct FunctionCodeGenerator<'a, 'ctx> {
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    module: &'a Module<'ctx>,
    variables: &'a mut HashMap<String, PointerValue<'ctx>>,
}

impl<'a, 'ctx> FunctionCodeGenerator<'a, 'ctx> {
    pub(crate) fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        variables: &'a mut HashMap<String, PointerValue<'ctx>>,
    ) -> Self {
        Self {
            context,
            builder,
            module,
            variables,
        }
    }

    pub(crate) fn generate(&self, ast: Ast) {
        match ast {
            Ast::Literal(_) => {
                let code = self.generate_code(ast);
                self.builder.build_return(Some(&code));
            }
            Ast::Expr {
                left: _,
                operator: _,
                right: _,
            } => {
                let code = self.generate_code(ast);
                self.builder.build_return(Some(&code));
            }
            _ => {
                self.builder
                    .build_return(Some(&self.context.i32_type().const_int(0, false)));
            }
        }
    }

    fn generate_code(&self, ast: Ast) -> BasicValueEnum {
        match ast {
            Ast::Literal(value) => self.generate_value(value),
            Ast::Expr {
                left,
                operator,
                right,
            } => self.generate_expr(*left, operator, *right),
            _ => todo!(),
        }
    }

    fn generate_expr(&self, left: Ast, operator: ExprOpKind, right: Ast) -> BasicValueEnum {
        let left_value = self.generate_code(left.clone());

        match left_value {
            BasicValueEnum::IntValue(_) => self.generate_int_expr(left, operator, right),
            _ => todo!(),
        }
    }

    fn generate_int_expr(&self, left: Ast, operator: ExprOpKind, right: Ast) -> BasicValueEnum {
        let left = self.generate_code(left);
        let right = self.generate_code(right);

        if let (BasicValueEnum::IntValue(left), BasicValueEnum::IntValue(right)) = (left, right) {
            match operator {
                ExprOpKind::EAdd => self
                    .builder
                    .build_int_add(left, right, "what")
                    .as_basic_value_enum(),

                _ => todo!(),
            }
        } else {
            panic!("TypeError"); // TODO: add details
        }
    }

    fn generate_value(&self, value: Value) -> BasicValueEnum {
        match value {
            Value::Integer(v) => self
                .context
                .i64_type() // TODO: Decide bit length
                .const_int(v.abs() as u64, v.is_positive())
                .as_basic_value_enum(),
            _ => todo!(),
        }
    }
}
