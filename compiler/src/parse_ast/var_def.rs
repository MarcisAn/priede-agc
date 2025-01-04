use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn var_def(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    if node.child(0).get_value().unwrap() != "sk" {
        panic!("Not supported");
    }
    parse_ast(node.child(2), compiler, block);
    let value = compiler.stack.pop_back().unwrap();
    let name = node.child(1).get_value().unwrap().to_string();
    let value_register = match value {
        crate::StackValue::NUM { register } => register,
        _ => panic!("non-number"),
    };
    let cloned_vars = compiler.variables.clone();
    let registered_id = cloned_vars.get(&name);
    if registered_id.is_some() {
        block.push(OPTCODE::SetVariable { id: *registered_id.unwrap(), source_reg: value_register });
    }
    else {
        let new_var_id = compiler.variables.len();
        compiler.variables.insert(name, new_var_id);
        block.push(OPTCODE::SetVariable { id: new_var_id, source_reg: value_register });
    }
}
