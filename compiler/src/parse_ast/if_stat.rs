use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn if_stat(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    let is_ifelse = node.children_count() > 2;
    if is_ifelse {
        parse_ast(node.child(0), compiler, block);
        let mut if_bytecode: Vec<OPTCODE> = vec![];
        let mut else_bytecode: Vec<OPTCODE> = vec![];

        parse_ast(node.child(1), compiler, &mut if_bytecode);
        
        parse_ast(node.child(3), compiler, &mut else_bytecode);

        let conditional = compiler.stack.pop_back().unwrap();
        let register_to_check = match conditional {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        block.push(OPTCODE::JumpIfZero {
            target: compiler.jump_target_counter,
            register: register_to_check,
        });
        block.append(&mut if_bytecode);
        block.push(crate::OPTCODE::Jump {
            target: compiler.jump_target_counter + 1,
        });
        block.push(OPTCODE::JumpTarget { number: compiler.jump_target_counter });
        block.append(&mut else_bytecode);
        block.push(OPTCODE::JumpTarget {
            number: compiler.jump_target_counter + 1,
        });
        compiler.jump_target_counter += 2;
    } else {
        parse_ast(node.child(0), compiler, block);
        let conditional = compiler.stack.pop_back().unwrap();
        let mut body: Vec<OPTCODE> = vec![];
        parse_ast(node.child(1), compiler, &mut body);
        let register_to_check = match conditional {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        block.push(OPTCODE::JumpIfZero {
            target: compiler.jump_target_counter,
            register: register_to_check,
        });
        block.append(&mut body);
        block.push(crate::OPTCODE::JumpTarget { number: compiler.jump_target_counter });
        compiler.jump_target_counter += 1;
    }
}

/*
 */
