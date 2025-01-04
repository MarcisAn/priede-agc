use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, StackValue, OPTCODE };

use super::parse_ast;

pub fn comp_s(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    parse_ast(node.child(0), compiler, block);
    parse_ast(node.child(2), compiler, block);
    let a = compiler.stack.pop_back().unwrap();
    let b = compiler.stack.pop_back().unwrap();

    let a_register = match a {
        crate::StackValue::NUM { register } => register,
        _ => panic!("addition with non-number"),
    };
    let b_register = match b {
        crate::StackValue::NUM { register } => register,
        _ => panic!("addition with non-number"),
    };
    if node.child(1).get_symbol().name == "=" {
        block.push(crate::OPTCODE::AreEqual {
            target_a_register: a_register,
            b_register: b_register,
            comparison_id: compiler.comparison_counter,
        });
    } else if node.child(1).get_symbol().name == ">=" {
        block.push(crate::OPTCODE::LargerEqual {
            target_a_register: a_register,
            b_register: b_register,
            comparison_id: compiler.comparison_counter,
        });
    } else if node.child(1).get_symbol().name == "<=" {
        block.push(crate::OPTCODE::LessEqual {
            target_a_register: a_register,
            b_register: b_register,
            comparison_id: compiler.comparison_counter,
        });
    } else if node.child(1).get_symbol().name == ">" {
        block.push(crate::OPTCODE::LargerThan {
            target_a_register: a_register,
            b_register: b_register,
            comparison_id: compiler.comparison_counter,
        });
    } else if node.child(1).get_symbol().name == "<" {
       block.push(crate::OPTCODE::LessThan {
            target_a_register: a_register,
            b_register: b_register,
            comparison_id: compiler.comparison_counter,
        });
    }
    compiler.comparison_counter += 1;
    compiler.stack.push_back(StackValue::NUM { register: a_register });
}
