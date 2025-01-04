use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn id(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    let name = &node.get_value().unwrap().to_string();
    let id = compiler.variables.get(name);

    if id.is_none() {
        panic!("Variable '{}' not found", name);
    }

    block.push(OPTCODE::LoadVariable { id: *id.unwrap(), target_reg: compiler.register_counter });
    compiler.stack.push_back(crate::StackValue::NUM { register: compiler.register_counter });
    compiler.register_counter += 1;
}
