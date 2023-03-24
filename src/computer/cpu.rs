/// Модуль процессора.

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

pub enum OperationType {
    /// Нет операции (ничего не делать).
    None,
    /// Операция чтения данных.
    Read,
    /// Операция записи данных.
    Write,
}

use std::collections::HashMap;
use crate::computer::{AddrBitDepth, ArcBitDepth, PC_START};

pub struct CPU {
    registers: HashMap<Registers, ArcBitDepth>,
}

impl CPU {
    /// Конструктор.
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
        registers.insert(Registers::RegPC, PC_START);
        registers.insert(Registers::RegCond, 0);

        CPU { registers }
    }

    /// Сброс состояния процессора.
    pub fn reset(&mut self) {

    }

    /// Один тактовый импульс.
    pub fn tick(&mut self) {

    }

    /// Возвращает значение, показывающее необходимо ли считать данные из памяти, записать,
    /// или ничего не предпринимать.
    pub fn memory_request(&mut self) -> OperationType {
        OperationType::None
    }

    /// Адрес памяти с которым выполняются действия (чтение или запись).
    pub fn address(&self) -> Option<AddrBitDepth> {
        None
    }

    /// Возвращает данные которые следует поместить в ячейку памяти или устройство вывода.
    pub fn data(&self) -> Option<ArcBitDepth> {
        None
    }

    /// Передает данные процессору (из памяти или периферийного устройства).
    pub fn set_data(&mut self, data: ArcBitDepth) {

    }

    /// Закрытый метод. Читает значение регистра.
    fn read_register(&self, reg: &Registers) -> Option<&ArcBitDepth> {
        match self.registers.get(reg) {
            None => None,
            Some(val) => Some(val)
        }
    }
}