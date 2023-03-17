use std::collections::HashMap;

// Тип определяющий разрядность нашей архитектуры.
type ArcBit = u16;

/// Регистры процессора.
/// RegR0 - RegR7 регистры общего назначения.
/// RegPC регистр счетчика команд (program counter, хранит адрес памяти, где находится следующая
/// исполняемая инструкция).
/// RegCond регистр флагов условий.
#[derive(Eq, Hash, PartialEq)]
pub enum Registers {
    RegR0,
    RegR1,
    RegR2,
    RegR3,
    RegR4,
    RegR5,
    RegR6,
    RegR7,
    RegPC,
    RegCond,
}

pub struct CPU {
    registers: HashMap<Registers, ArcBit>
}

impl CPU {
    pub fn new() -> CPU {
        let mut registers = HashMap::new();
        registers.insert(Registers::RegR0, 0);
        registers.insert(Registers::RegR1, 0);
        registers.insert(Registers::RegR2, 0);
        registers.insert(Registers::RegR3, 0);
        registers.insert(Registers::RegR4, 0);
        registers.insert(Registers::RegR5, 0);
        registers.insert(Registers::RegR6, 0);
        registers.insert(Registers::RegR7, 0);
        registers.insert(Registers::RegPC, 0);
        registers.insert(Registers::RegCond, 0);

        CPU { registers }
    }

    pub fn reg_get_val(&self, reg: &Registers) -> Option<&ArcBit> {
        match self.registers.get(reg) {
            None => None,
            Some(val) => Some(val)
        }
    }

    pub fn tick(&mut self) {
        // Пока ничего.
    }
}