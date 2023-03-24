/// Модуль оперативной памяти.

use crate::computer::{AddrBitDepth, ArcBitDepth, MAX_ADD_BIT_DEPTH};

pub struct Memory {
    data: [ArcBitDepth; MAX_ADD_BIT_DEPTH as usize]
}

impl Memory {
    //! Конструктор.
    pub fn new() -> Memory {
        Memory {
            data: [0; MAX_ADD_BIT_DEPTH as usize]
        }
    }

    pub fn read_memory(&self, address: AddrBitDepth) -> ArcBitDepth {
        self.data[address as usize]
    }

    pub fn write_memory(&mut self, address: AddrBitDepth, value: ArcBitDepth) {
        self.data[address as usize] = value;
    }

    /// Загружает программу.
    pub fn load() {

    }
}