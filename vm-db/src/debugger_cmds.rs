use vm_core::vm::OP;
use vm_core::vm::OP::*;
use crate::debugger::Debugger;
use cryptify;

pub const OPCODE_ARG_SIZES: &[(OpCode, usize)] = &[
    (OpCode::FN,    1),
    (OpCode::Nyaa,  1),
    (OpCode::Meow,  1),
    (OpCode::Nay,   0),
    (OpCode::Push,  1),
    (OpCode::Pop,   0),
    (OpCode::Add,   0),
    (OpCode::Sub,   0),
    (OpCode::Jmp,   1),
    (OpCode::Jz,    1),
    (OpCode::Call,  1),
    (OpCode::Ret,   0),
    (OpCode::Load,  0),
    (OpCode::Store, 2),
    (OpCode::Print, 1),
    (OpCode::Input, 0),
    (OpCode::Eq,    0),
    (OpCode::Check, 0),
    (OpCode::Debug, 0),
];

impl Debugger {
    fn cmd_next(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        if let Some(op) = self.vm.get_op() {
            self.vm.exec_op(op);
        }
        Ok(())
    }
    fn cmd_break(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        if cmds.len() < 2 {
            //TODO: do the pc but im sleepy rn
            println!("usage: break <fn_id>\nor: break *<pc>");
            return Ok(());
        }
        if let Ok(id) = cmds.get(1).unwrap().parse() {
            for i in self.vm.get_ftable().keys() {
                if i == &id {
                    let ppc = self.vm.get_ftable().get(i).unwrap();
                    println!("added breakpoint on {} pc {}", i, ppc);
                    self.breakpoints.push(id);
                }
            }
        }
        Ok(())
    }

    fn cmd_continue(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        loop {
            if let Some(op) = self.vm.get_op() {
                if op as u8 == OP::OpCode::Call as u8 {
                    let fn_id = self.vm.get_raw(self.vm.pc + 1).unwrap();
                    let mut a: bool = false;
                    for bp in &self.breakpoints {
                        if *bp == fn_id {
                            println!("hit breakpoint {}", fn_id);
                            a = true;
                            break;
                        }
                    }
                    if a {
                        break;
                    }
                }
                self.vm.exec_op(op);
            }
        }
        Ok(())
    }

    fn cmd_exit(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        Err(())
    }

    fn cmd_debug(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        let var = cryptify::encrypt_string!("==========DEBUG DUMP===========");
        let stack_msg = cryptify::encrypt_string!("==========STACK DUMP===========");
        let start = self.vm.pc.saturating_sub(3);
        let mut end = self.vm.pc + 4;
        let mut arg_size = 0;
        println!("{}", var);
        let mut i = start;
        loop {
            if i >= end || i >= self.vm.ram_len()  {
                break;
            }
            let o = self.vm.get_raw(i).unwrap_or(0);
            let op = OpCode::iterator().find(|&op_enum| op_enum as u8 == o);
            let display_value = match op {
                Some(op_found) => format!("{:?}", op_found),
                None => {
                    i += 1;
                    end += 1;
                    continue;
                },
            };
            if let Some(op_found) = op {
                for (opcode, size) in OPCODE_ARG_SIZES {
                    if *opcode as u8 == op_found as u8 {
                        arg_size = *size;
                        break;
                    }
                }
            }
            if i == self.vm.pc {
                print!("-> {i}: {}(", display_value);
            } else {
                print!("   {i}: {}(", display_value);
            }
            for o in 0..arg_size {
                print!("{}", self.vm.get_raw(i + (o+ 1)).unwrap_or(0));
                if o != arg_size - 1 {
                print!(" ");

                }
            }
                println!(")");

            i += arg_size;
            end += arg_size;
            i+=1;
        }
        println!();
        println!("{}", stack_msg);
        for i in 0..self.vm.stack.len() {
            println!("{i}: {}", self.vm.stack.get(i).unwrap());
        }
        println!();
        Ok(())
    }

    pub fn exec_cmd(&mut self, cmds: &Vec<String>) -> Result<(), ()> {
        match cmds.get(0).unwrap().to_lowercase().as_str() {
            "continue" | "c" => {
                return self.cmd_continue(&cmds);
            }
            "next" | "n" => {
                return self.cmd_next(&cmds);
            }
            "break" | "breakpoint" | "b" => {
                return self.cmd_break(&cmds);
            }
            "exit" => {
                return self.cmd_exit(&cmds);
            }
            "debug" | "d" => {
                return self.cmd_debug(&cmds);
            }
            _ => {}
        }
        Ok(())
    }
}
