use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{
    BasicValue, BasicValueEnum, FloatValue, GlobalValue, IntValue, PointerValue,
};
use std::collections::HashMap;

use super::{ExternFunctions, LLVMTypes};
use crate::ast::{Ast, ExprOpKind};
use crate::value::Value;

pub(crate) struct FmtStrSpecs<'a> {
    integer: GlobalValue<'a>,
    float: GlobalValue<'a>,
}

#[allow(dead_code)]
pub(crate) struct FunctionCodeGenerator<'a, 'ctx> {
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    module: &'a Module<'ctx>,
    ty: &'a LLVMTypes<'ctx>,
    fns: &'a ExternFunctions<'ctx>,
    variables: &'a mut HashMap<String, PointerValue<'ctx>>,
    fmt_str: FmtStrSpecs<'ctx>,
}

impl<'a, 'ctx> FunctionCodeGenerator<'a, 'ctx> {
    pub(crate) fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        ty: &'a LLVMTypes<'ctx>,
        fns: &'a ExternFunctions<'ctx>,
        variables: &'a mut HashMap<String, PointerValue<'ctx>>,
    ) -> Self {
        let format_int = builder.build_global_string_ptr("%d\n", "format_int");
        let format_float = builder.build_global_string_ptr("%f\n", "format_float");
        let fmt_str = FmtStrSpecs {
            integer: format_int,
            float: format_float,
        };

        Self {
            context,
            builder,
            module,
            ty,
            fns,
            variables,
            fmt_str,
        }
    }

    pub(crate) fn generate(&self, ast: Ast) {
        match ast {
            Ast::Literal(_) => {
                let code = self.generate_code(ast);
                self.generate_print(code);
            }

            Ast::Expr {
                left: _,
                operator: _,
                right: _,
            } => {
                let code = self.generate_code(ast);
                self.generate_print(code);
            }

            _ => todo!(),
        };

        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, false)));
    }

    fn generate_print(&self, code: BasicValueEnum) {
        match code {
            BasicValueEnum::IntValue(_) => {
                self.builder.build_call(
                    self.fns.printf,
                    &[self.fmt_str.integer.as_basic_value_enum(), code],
                    "call",
                );
            }
            BasicValueEnum::FloatValue(_) => {
                self.builder.build_call(
                    self.fns.printf,
                    &[self.fmt_str.float.as_basic_value_enum(), code],
                    "call",
                );
            }
            _ => unreachable!(),
        };
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
            BasicValueEnum::IntValue(_) => self
                .generate_int_expr(left, operator, right)
                .as_basic_value_enum(),
            BasicValueEnum::FloatValue(_) => self
                .generate_float_expr(left, operator, right)
                .as_basic_value_enum(),
            _ => todo!(),
        }
    }

    fn generate_int_expr(&self, left: Ast, operator: ExprOpKind, right: Ast) -> IntValue {
        let left = self.generate_code(left);
        let right = self.generate_code(right);

        if let (BasicValueEnum::IntValue(left), BasicValueEnum::IntValue(right)) = (left, right) {
            match operator {
                ExprOpKind::EAdd => self.builder.build_int_add(left, right, ""),
                ExprOpKind::ESub => self.builder.build_int_sub(left, right, ""),
                ExprOpKind::EMul => self.builder.build_int_mul(left, right, ""),
                ExprOpKind::EDiv => self.builder.build_int_signed_div(left, right, ""),
            }
        } else {
            panic!("TypeError"); // TODO: add details
        }
    }

    fn generate_float_expr(&self, left: Ast, operator: ExprOpKind, right: Ast) -> FloatValue {
        let left = self.generate_code(left);
        let right = self.generate_code(right);

        if let (BasicValueEnum::FloatValue(left), BasicValueEnum::FloatValue(right)) = (left, right)
        {
            match operator {
                ExprOpKind::EAdd => self.builder.build_float_add(left, right, ""),
                ExprOpKind::ESub => self.builder.build_float_sub(left, right, ""),
                ExprOpKind::EMul => self.builder.build_float_mul(left, right, ""),
                ExprOpKind::EDiv => self.builder.build_float_div(left, right, ""),
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
            Value::Float(v) => self
                .context
                .f64_type() // TODO: Decide bit length
                .const_float(v as f64)
                .as_basic_value_enum(),
        }
    }
}
