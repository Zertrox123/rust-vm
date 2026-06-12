use std::collections::HashMap;
use std::process::exit;
use std::{
    io::{Read, Write},
    usize,
};

use self::OpCode::*;
use crate::vm::RAM;
use crate::vm::VM::VM;
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    FN = 0xED,
    Nyaa = 0xEE,
    Meow = 0xEF,
    Nay = 0xf0,
    Push = 0xf1,
    Pop = 0xf2,
    Add = 0xf3,
    Sub = 0xf4,
    Jmp = 0xf5,
    Jz = 0xf6,
    Call = 0xf7,
    Ret = 0xf8,
    Load = 0xf9,
    Store = 0xfa,
    Print = 0xfb,
    Input = 0xfc,
    Eq = 0xfd,
    Check = 0xfe,
    Debug = 0xff,
}

pub const KEY_B: [u8; 20] = *b"AS6lXXVAd1oXXg6q#fm1"; // junk at indices 4,5,11,12
impl OpCode {
    pub fn iterator() -> impl Iterator<Item = OpCode> {
        [
            Push, Pop, Add, Sub, Jmp, Jz, Call, Ret, Load, Store, Debug, Print, Input, Eq, Check,
            Nay, Meow, Nyaa, FN,
        ]
        .iter()
        .copied()
    }

    pub fn from_u8(val: u8) -> Option<OpCode> {
        match val {
            0xED => Some(FN),
            0xEE => Some(Nyaa),
            0xEF => Some(Meow),
            0xf0 => Some(Nay),
            0xf1 => Some(Push),
            0xf2 => Some(Pop),
            0xf3 => Some(Add),
            0xf4 => Some(Sub),
            0xf5 => Some(Jmp),
            0xf6 => Some(Jz),
            0xf7 => Some(Call),
            0xf8 => Some(Ret),
            0xf9 => Some(Load),
            0xfa => Some(Store),
            0xfb => Some(Print),
            0xfc => Some(Input),
            0xfd => Some(Eq),
            0xfe => Some(Check),
            0xff => Some(Debug),
            _ => None,
        }
    }
}

pub fn impl_Nyaa(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>, key: u8) -> bool {
    stack.push(b'N');
    stack.push(b'y');
    stack.push(b'a');
    stack.push(b'a');
    for i in stack {
        print!("{}", *i as char);
    }
    println!("Nyaaaa");
    *pc += 2;
    false
}

pub fn impl_Meow(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>, key: u8) -> bool {
    stack.push(b'M');
    stack.push(b'e');
    stack.push(b'o');
    stack.push(b'w');
    stack.push(b'w');
    stack.push(b'w');
    for i in stack {
        print!("{}", *i as char);
    }
    *pc += 2;
    false
}

#[inline(never)]
pub fn impl_nay(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>, key: u8) -> bool {
    exit(1);
}

#[inline(never)]
pub fn impl_print(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>, key: u8) -> bool {
    let dest = ram.get(*pc + 1).ok().map(|b| b ^ key);
    let char = stack.get(dest.unwrap() as usize).copied();
    print!("{}", char.unwrap_or_default() as char);
    let _ = std::io::stdout().flush();
    *pc += 2;
    false
}

#[inline(never)]
pub fn impl_push(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>, key: u8) -> bool {
    let dest = ram.get(*pc + 1).ok().map(|b| b ^ key);
    if let Some(dest) = dest {
        stack.push(dest);
    }
    *pc += 2;
    false
}

#[inline(never)]
pub fn impl_pop(pc: &mut usize, _ram: &mut RAM::RAM, stack: &mut Vec<u8>) -> bool {
    stack.pop();
    *pc += 1;
    true
}

#[inline(never)]
pub fn impl_add(pc: &mut usize, _ram: &mut RAM::RAM, stack: &mut Vec<u8>) -> bool {
    let a = stack.pop().unwrap_or(0);
    let b = stack.pop().unwrap_or(0);
    stack.push(b.wrapping_add(a));
    *pc += 1;
    true
}

#[inline(never)]
pub fn impl_sub(pc: &mut usize, _ram: &mut RAM::RAM, stack: &mut Vec<u8>) -> bool {
    let a = stack.pop().unwrap_or(0);
    let b = stack.pop().unwrap_or(0);
    stack.push(b.wrapping_sub(a));
    *pc += 1;
    true
}

#[inline(never)]
pub fn impl_jmp(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>, key: u8) -> bool {
    let dest = ram.get(*pc + 1).ok().map(|b| b ^ key);
    if let Some(dest) = dest {
        *pc = dest as usize;
    }
    true
}

#[inline(never)]
pub fn impl_jz(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>, key: u8) -> bool {
    let val = stack.pop().unwrap();
    let dest = ram.get(*pc + 1).ok().map(|b| b ^ key);
    if val == 0 {
        if let Some(dest) = dest {
            *pc = dest as usize;
            return true;
        }
    }
    *pc += 2;
    true
}

#[inline(never)]
pub fn impl_call(
    ftable: &HashMap<u8, usize>,
    pc: &mut usize,
    ram: &mut RAM::RAM,
    stack: &mut Vec<u8>,
    key: u8,
) -> bool {
    let dest = ram.get(*pc + 1).ok().map(|b| b ^ key);
    if let Some(dest) = dest
        && let Some(idx) = ftable.get(&dest)
    {
        *pc = *idx;
        return true;
    }
    false
}

#[inline(never)]
pub fn impl_ret(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>) -> bool {
    if let Some(addr) = stack.pop() {
        *pc = addr as usize;
        return true;
    }
    false
}

#[inline(never)]
pub fn impl_load(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>, key: u8) -> bool {
    if let Some(addr) = stack.pop() {
        if let Ok(val) = ram.get(addr as usize) {
            stack.push(val ^ key);
        }
    }
    *pc += 1;
    true
}

#[inline(never)]
pub fn impl_store(pc: &mut usize, ram: &mut RAM::RAM, stack: &mut Vec<u8>, key: u8) -> bool {
    let dest = ram.get(*pc + 1).ok().map(|b| b ^ key);
    let value = stack.first().copied();

    if let (Some(dest), Some(value)) = (dest, value) {
        stack.remove(0);
        ram[dest as usize] = value ^ key;
    }
    *pc += 3;
    true
}

#[inline(never)]
pub fn impl_input(pc: &mut usize, _ram: &mut RAM::RAM, stack: &mut Vec<u8>, _key: u8) -> bool {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    for byte in input.bytes() {
        stack.push(byte);
    }
    stack.push(0);
    *pc += 1;
    true
}

#[inline(never)]
pub fn impl_eq(pc: &mut usize, _ram: &mut RAM::RAM, stack: &mut Vec<u8>, _key: u8) -> bool {
    if stack.len() >= 2 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(if a == b { 1 } else { 0 });
    } else {
        stack.push(0);
    }
    *pc += 1;
    true
}

#[inline(never)]
fn calc(last: usize, stack: &mut Vec<u8>) -> usize {
    if let Some(value) = stack.pop() {
        calc(last + value as usize, stack)
    } else {
        last
    }
}

#[inline(never)]
pub fn impl_dup(pc: &mut usize, _ram: &mut RAM::RAM, stack: &mut Vec<u8>, _key: u8) -> bool {
    // UwUSuperPasswordTguezTuLauraJmsTfacon
    if stack.len() != 37 {
        let a = calc(0, stack);
        if a == 3797 {
            *pc = 85;
        } else {
            *pc = 0;
        }
    }
    *pc += 2;
    false
}
