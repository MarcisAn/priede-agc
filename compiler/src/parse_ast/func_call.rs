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
    let a = compiler.stack.pop_back().unwrap();
    let id_register = match a {
        crate::StackValue::NUM { register } => register,
        _ => panic!("addition with non-number"),
    };
    if func_name == "program" {
        block.push(crate::OPTCODE::Display {
            display: crate::DisplayOptions::Prog,
            register: id_register,
        });
    } else if func_name == "verb" {
        block.push(crate::OPTCODE::Display {
            display: crate::DisplayOptions::Verb,
            register: id_register,
        });
    } else if func_name == "noun" {
        block.push(crate::OPTCODE::Display {
            display: crate::DisplayOptions::Noun,
            register: id_register,
        });
    }else if func_name == "r1" {
        block.push(crate::OPTCODE::Display {
            display: crate::DisplayOptions::R1,
            register: id_register,
        });
    }else if func_name == "r2" {
        block.push(crate::OPTCODE::Display {
            display: crate::DisplayOptions::R2,
            register: id_register,
        });
    }else if func_name == "r3" {
        block.push(crate::OPTCODE::Display {
            display: crate::DisplayOptions::R3,
            register: id_register,
        });
    }
}
//(number.replace(",", ".").parse::<f64>().unwrap()).to_string().replace(",", ".")
