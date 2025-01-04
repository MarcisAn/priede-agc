use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, StackValue, OPTCODE };

use super::parse_ast;

pub fn func_call(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    if node.children_count() > 1 {
        for arg in node.child(1).children() {
            parse_ast(arg, compiler, block);
        }
    }
    let func_name = node.child(0).get_value().unwrap();
    if func_name == "program" {
        let a = compiler.stack.pop_back().unwrap();
        let id_register = match a {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        block.push(crate::OPTCODE::Display { display: crate::DisplayOptions::Prog, register: id_register });
    }
}
//(number.replace(",", ".").parse::<f64>().unwrap()).to_string().replace(",", ".")
