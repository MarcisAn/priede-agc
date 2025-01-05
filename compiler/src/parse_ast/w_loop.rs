use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn w_loop(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    block.push(OPTCODE::JumpTarget { number: compiler.jump_target_counter + 1 });

    parse_ast(node.child(0), compiler, block);
    let conditional = compiler.stack.pop_back().unwrap();
    let register_to_check = match conditional {
        crate::StackValue::NUM { register } => register,
        _ => panic!("Only numbers are supported"),
    };

    let mut body: Vec<OPTCODE> = vec![];
    parse_ast(node.child(1), compiler, &mut body);

    block.push(OPTCODE::JumpIfZero { target: compiler.jump_target_counter, register: register_to_check });
    block.append(&mut body);
    block.push(OPTCODE::Jump { target: compiler.jump_target_counter + 1 });
    block.push(OPTCODE::JumpTarget { number: compiler.jump_target_counter });
    compiler.jump_target_counter += 2;
}
