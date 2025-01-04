use std::{ collections::{HashMap, LinkedList}, fs, process };
mod agc_output;
use agc_output::format_asm;
use hime_redist::ast::AstNode;
mod parse_ast;
use parse_ast::parse_ast;
mod hime;

#[derive(Debug, Clone)]
pub enum StackValue {
    NUM {
        register: usize,
    },
    STRING {
        register: usize,
    },
}

#[derive(Debug, Clone)]
pub enum DisplayOptions {
    Prog,
    Verb,
    Noun,
    R1,
    R2,
    R3,
}

#[derive(Debug, Clone)]
pub enum OPTCODE {
    Display {
        display: DisplayOptions,
        register: usize,
    },
    LoadNumber {
        const_num: usize,
        register: usize,
    },
    Add {
        target_register: usize,
        value_register: usize,
    },
    Subtract {
        target_register: usize,
        value_register: usize,
    },
    Multiply {
        target_register: usize,
        value_register: usize,
    },
    Divide {
        target_register: usize,
        value_register: usize,
    },
    Jump {
        target: usize,
    },
    JumpIfZero {
        target: usize,
        register: usize
    },
    JumpTarget {
        number: usize
    },
    LoadInA {
        register: usize
    },
    AreEqual {
        target_a_register: usize,
        b_register: usize,
        comparison_id: usize
    },
    LargerEqual {
        target_a_register: usize,
        b_register: usize,
        comparison_id: usize
    },
    LargerThan {
        target_a_register: usize,
        b_register: usize,
        comparison_id: usize
    },
    LessEqual {
        target_a_register: usize,
        b_register: usize,
        comparison_id: usize
    },
    LessThan {
        target_a_register: usize,
        b_register: usize,
        comparison_id: usize
    },
    LoadVariable {
        id: usize,
        target_reg: usize
    },
    SetVariable {
        id: usize,
        source_reg: usize
    },

}

#[derive(Debug, Clone)]
pub struct Constant {
    register: usize,
    value: String,
}

#[derive(Debug, Clone)]
pub struct Compiler {
    register_counter: usize,
    jump_target_counter: usize,
    stack: LinkedList<StackValue>,
    constants: Vec<String>,
    comparison_counter: usize,
    variables: HashMap<String, usize>
}

impl Compiler {
    pub fn new() -> Compiler {
        return Compiler {
            register_counter: 0,
            stack: LinkedList::new(),
            constants: vec![],
            jump_target_counter: 0,
            comparison_counter: 0,
            variables: HashMap::new(),
        };
    }
    pub fn add(&mut self, block: &mut Vec<OPTCODE>) {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();

        let a_register = match a {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        let b_register = match b {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };

        block.push(crate::OPTCODE::Add {
            target_register: a_register,
            value_register: b_register,
        });
        self.stack.push_back(crate::StackValue::NUM { register: a_register });
    }
    pub fn subtract(&mut self, block: &mut Vec<OPTCODE>) {
        let b = self.stack.pop_back().unwrap();
        let a = self.stack.pop_back().unwrap();

        let a_register = match a {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        let b_register = match b {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };

        block.push(crate::OPTCODE::Subtract {
            target_register: a_register,
            value_register: b_register,
        });
        self.stack.push_back(crate::StackValue::NUM { register: a_register });
        self.register_counter += 1;
    }
    pub fn divide(&mut self, block: &mut Vec<OPTCODE>) {
        let b = self.stack.pop_back().unwrap();
        let a = self.stack.pop_back().unwrap();

        let a_register = match a {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        let b_register = match b {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };

        block.push(crate::OPTCODE::Divide {
            target_register: a_register,
            value_register: b_register,
        });
        self.stack.push_back(crate::StackValue::NUM { register: a_register });
        self.register_counter += 1;
    }
}

pub fn compile(path: String) {
    let file_content = read_file(path.clone());

    let parse_res = hime::priede::parse_string(file_content.clone());
    let ast = parse_res.get_ast();
    print!("{:?}", parse_res.errors.errors);
    let root = ast.get_root();
    print_ast(root);
    let mut compiler = Compiler::new();
    let mut main_block: Vec<OPTCODE> = vec![];
    parse_ast(root, &mut compiler, &mut main_block);

    for optcode in &main_block {
        println!("{:?}", optcode);
    }

    println!("{:?}", compiler.constants);

    let asm_file = format_asm(main_block, compiler.constants, compiler.register_counter, compiler.variables.len());
    // println!("{}", asm_file);
    let _ = fs::write("E:/Dev/priede-agc/dist/Validation.agc", asm_file);
    //C:\ProgramData\MA Lighting Technologies\grandma\gma2_V_3.9.60\macros
}

pub fn read_file(path: String) -> String {
    let file_read = fs::read_to_string(&path);
    if file_read.is_err() {
        println!("{}", file_read.err().unwrap());
        println!("Neizdevās nolasīt failu {} \nPārlicinies, ka faila adrese ir pareiza!", path);
        process::exit(1);
    }
    file_read.unwrap()
}
fn print<'a>(node: AstNode, crossings: Vec<bool>) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", if crossings[i] { "|   " } else { "    " });
            i += 1;
        }
        print!("+-> ");
    }
    println!("{:}", node);
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        print(children.at(i), child_crossings);
        i += 1;
    }
}
pub fn print_ast(node: AstNode) {
    print(node, Vec::<bool>::new());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        compile("../examples/test.pr".to_string());
    }
}
