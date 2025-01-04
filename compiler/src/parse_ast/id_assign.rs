use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn id_assign(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    parse_ast(node.child(2), compiler, block);
    let value = compiler.stack.pop_back().unwrap();

    let value_register = match value {
        crate::StackValue::NUM { register } => register,
        _ => panic!("addition with non-number"),
    };
    let varname = node.child(0).get_value().unwrap().to_string();

    let cloned_vars = compiler.variables.clone();
    let registered_id = cloned_vars.get(&varname);
    if registered_id.is_some() {
        if node.child(1).get_symbol().name == ":" {
            block.push(OPTCODE::SetVariable {
                id: *registered_id.unwrap(),
                source_reg: value_register,
            });
        } else if node.child(1).get_symbol().name == "+:" {
            block.push(OPTCODE::LoadVariable {
                id: *registered_id.unwrap(),
                target_reg: compiler.register_counter,
            });
            block.push(OPTCODE::Add {
                target_register: compiler.register_counter,
                value_register: value_register,
            });
            block.push(OPTCODE::SetVariable {
                id: *registered_id.unwrap(),
                source_reg: compiler.register_counter,
            });
        }
    } else {
        panic!("Variable not found");
    }
    compiler.register_counter += 1;
}
