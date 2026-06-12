use std::{
    io::Result, ops::{self, Index}, process::id, time::UNIX_EPOCH, u8
};

use zeroize::Zeroizing;

#[derive(Debug, Clone)]
pub struct RAM {
    ram: Zeroizing<Vec<u8>>,
    count: usize,
    key: Zeroizing<usize>,
}

impl RAM {
    pub fn setup() -> RAM {
        RAM {
            ram: Zeroizing::new(Vec::new()),
            count: 0,
            key: Zeroizing::new(
                (std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u16) as usize,
            ),
        }
    }

    pub fn push(&mut self, byte: u8) {
        let idx = self.count ^ *self.key;
        if idx >= self.ram.len() {
            for i in self.ram.len()..idx + 1 {
                self.ram.push(0x0);
            }
        }
        self.ram[idx] = byte;
        self.count += 1;
    }

    pub fn add_byte(&mut self, byte: u8) {
        let idx = self.count ^ *self.key;
        if idx >= self.ram.len() {
            for i in self.ram.len()..idx + 1 {
                self.ram.push(0x0);
            }
        }
        self.ram[idx] = byte;
        self.count += 1;
    }

    pub fn get_value(&mut self, pc: usize) -> u8 {
        let idx = pc ^ *self.key;
        self.ram[idx]
    }

    pub fn get(&self, pc: usize) -> Result<u8> {
        let idx = pc ^ *self.key;
        if idx >= self.ram.len() {
            return Ok(0)
        }
        Ok(self.ram[idx])
    }
    

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn debug(&mut self) {
        println!("{:?}", self.ram);
    }

    pub fn set(&mut self, pc: usize, value: u8) {
        let idx = pc ^ *self.key;
        if idx >= self.ram.len() {
            for i in self.ram.len()..idx + 1 {
                self.ram.push(0x0);
            }
        }
        self.ram[idx] = value;
    }
}
impl ops::Index<usize> for RAM {
    type Output = u8;

    fn index(&self, _rhs: usize) -> &Self::Output {
        let idx = _rhs ^ *self.key;
        &self.ram[idx]
    }
}

impl ops::IndexMut<usize> for RAM {
    fn index_mut(&mut self, _rhs: usize) -> &mut Self::Output {
        let idx = _rhs ^ *self.key;
        &mut self.ram[idx]
    }
}
