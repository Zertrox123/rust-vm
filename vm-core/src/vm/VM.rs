use std::{
    collections::HashMap,
    fmt::Debug,
    process::exit,
    time::{self, Instant, SystemTime, UNIX_EPOCH},
};

use cryptify;
use zeroize::Zeroizing;

use crate::vm::{
    OP::{
        OpCode, impl_Meow, impl_Nyaa, impl_add, impl_call, impl_dup, impl_eq, impl_input, impl_jmp,
        impl_jz, impl_load, impl_nay, impl_pop, impl_print, impl_push, impl_ret, impl_store,
        impl_sub,
    },
    RAM,
};

#[derive(Debug, Clone)]
pub struct VM {
    pub ram: RAM::RAM,
    func_table: HashMap<u8, usize>,
    pub stack: Vec<u8>,
    pub pc: usize,
    pub key: Zeroizing<u8>,
    last_op_time: Option<Instant>,
    crash_in_10: bool,
    crash_counter: u8,
    time: SystemTime,
    len: usize,
}

impl VM {
    pub fn new() -> VM {
        VM {
            ram: RAM::RAM::setup(),
            func_table: HashMap::new(),
            stack: Vec::new(),
            pc: 0,
            key: Zeroizing::new(
                std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u8,
            ),
            last_op_time: None,
            crash_in_10: false,
            crash_counter: 0,
            time: time::SystemTime::now(),
            len: 0,
        }
    }

    fn debug(&self) {
        let var = cryptify::encrypt_string!("==========DEBUG DUMP===========");
        let stack_msg = cryptify::encrypt_string!("==========STACK DUMP===========");
        println!("{}", var);
        for i in 0..self.ram.len() {
            if i % 8 == 0 {
                println!();
            }
            if i == self.pc {
                let o = self.ram.get(i).unwrap() ^ *self.key;
                let op = OpCode::iterator().find(|&i| i as u8 == o);
                if op.is_some() {
                    print!("{i}:[{:?}] ", op.unwrap());
                } else {
                    print!("{i}:[{:?}] ", o);
                }
            } else {
                print!("{i}:{} ", self.ram.get(i).unwrap() ^ *self.key);
            }
        }
        println!();
        println!("{}", stack_msg);
        for i in 0..self.stack.len() {
            print!("{} ", self.stack.get(i).unwrap());
        }
        println!();
    }

    pub fn get_raw(&mut self, id: usize) -> Option<u8> {
        Some(self.ram.get(id).unwrap() ^ *self.key)
    }

    pub fn get_op(&mut self) -> Option<OpCode> {
        let byte = self.ram.get(self.pc).unwrap() ^ *self.key;

        let high = byte >> 4;

        if high != 0xf && high != 0xE {
            return None;
        }

        if byte == 0xff {
            self.debug();
        }

        self.last_op_time = Some(Instant::now());

        OpCode::iterator().find(|&i| i as u8 == byte)
    }

    pub fn ftable(&mut self) {
        for i in 0..self.ram.len() {
            if self.ram[i] ^ *self.key == OpCode::FN as u8 {
                let id = self.ram[i + 1] ^ *self.key;
                self.func_table.insert(id, i);
            }
        }
    }

    pub fn get_ftable(&self) -> &HashMap<u8, usize> {
        &self.func_table
    }

    pub fn check(&mut self) {
        cryptify::flow_stmt!();
        let a = time::SystemTime::now().duration_since(self.time);
        cryptify::flow_stmt!();
        match a {
            Ok(d) => {
                cryptify::flow_stmt!();
                if d.as_millis() > 500 {
                    let chars = vec![
                        0x4e, 0x79, 0x61, 0x20, 0x4d, 0x65, 0x6f, 0x6f, 0x77, 0x77, 0x77,
                    ];
                    for i in chars {
                        print!("{i}");
                    }
                    println!();

                    self.corrupt_ram();
                }
            }
            Err(_) => {
                let chars = vec![
                    0x4e, 0x79, 0x61, 0x20, 0x4d, 0x65, 0x6f, 0x6f, 0x77, 0x77, 0x77,
                ];
                for i in chars {
                    print!("{i}");
                }
                println!();
            }
        }
    }

    pub fn exec_op(&mut self, op: OpCode) {
        self.time = time::SystemTime::now();
        let mut skip: bool = false;
        cryptify::flow_stmt!();
        cryptify::flow_stmt!();
        match op {
            OpCode::FN => {
                self.pc += 2;
            }
            OpCode::Nyaa => {
                cryptify::flow_stmt!();
                impl_Nyaa(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Meow => {
                cryptify::flow_stmt!();
                impl_Meow(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Nay => {
                cryptify::flow_stmt!();
                impl_nay(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Push => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                if let Some(last) = self.last_op_time
                    && last.elapsed() > std::time::Duration::from_secs(1)
                {
                    self.corrupt_ram();
                }
                if impl_push(&mut self.pc, &mut self.ram, &mut self.stack, *self.key) {
                    cryptify::flow_stmt!();
                    cryptify::flow_stmt!();
                    return;
                }
                cryptify::flow_stmt!();
            }
            OpCode::Pop => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_pop(&mut self.pc, &mut self.ram, &mut self.stack);
                cryptify::flow_stmt!();
            }
            OpCode::Add => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_add(&mut self.pc, &mut self.ram, &mut self.stack);
                cryptify::flow_stmt!();
            }
            OpCode::Sub => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_sub(&mut self.pc, &mut self.ram, &mut self.stack);
                cryptify::flow_stmt!();
            }
            OpCode::Jmp => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_jmp(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Jz => {
                cryptify::flow_stmt!();
                impl_jz(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Call => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_call(
                    &self.func_table,
                    &mut self.pc,
                    &mut self.ram,
                    &mut self.stack,
                    *self.key,
                );
                cryptify::flow_stmt!();
            }
            OpCode::Ret => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_ret(&mut self.pc, &mut self.ram, &mut self.stack);
            }
            OpCode::Load => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_load(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Store => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_store(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Print => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_print(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
                cryptify::flow_stmt!();
            }
            OpCode::Input => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_input(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
                skip = true;
            }
            OpCode::Eq => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_eq(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
            }
            OpCode::Check => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                impl_dup(&mut self.pc, &mut self.ram, &mut self.stack, *self.key);
            }
            OpCode::Debug => {
                cryptify::flow_stmt!();
                cryptify::flow_stmt!();
                self.pc += 1;
            }
        }
        cryptify::flow_stmt!();
        cryptify::flow_stmt!();
        if let Some(last) = self.last_op_time
            && last.elapsed() > std::time::Duration::from_secs(1)
            && !skip
        {
            self.corrupt_ram();
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.ram.push(byte as u8 ^ *self.key);
        self.len += 1
    }

    pub fn ram_len(&self) -> usize {
        return self.len
    }

    pub fn corrupt_ram(&mut self) {
        for i in 0..self.ram.len() {
            self.ram
                .set(i, (0xEE + (i as u8 % (255 - 0xEE))) ^ *self.key);
        }
    }

    pub fn run(&mut self) {
        self.check();
        cryptify::flow_stmt!();
        self.ftable();
        while self.pc < self.ram.len() {
            cryptify::flow_stmt!();
            if let Some(op) = self.get_op() {
                cryptify::flow_stmt!();
                self.exec_op(op);
            } else {
                cryptify::flow_stmt!();
                let chars: Vec<u8> = vec![
                    0x4e, 0x79, 0x61, 0x20, 0x4d, 0x65, 0x6f, 0x6f, 0x77, 0x77, 0x77,
                ];
                cryptify::flow_stmt!();
                for i in chars {
                    print!("{}", i as char);
                }
                print!(" ");

                self.corrupt_ram();
                self.pc += 2;
            }
        }
    }
}

pub const NONCE_B: [u8; 9] = *b"0aXXBv4ZZ";
