use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

pub fn number(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    let number= node.get_value().unwrap().replace(",", ".");
    compiler.stack.push_back(crate::StackValue::NUM {
        register: compiler.register_counter,
    });
    compiler.constants.push(number);
    block.push(OPTCODE::LoadNumber { const_num: compiler.constants.len() - 1, register: compiler.register_counter });
    compiler.register_counter += 1;
}
