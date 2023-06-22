use std::fmt::Display;

use itertools;

use crate::{flags::Flags, instruction::Instruction, register::Reg};

#[derive(Debug, Default)]
pub struct Cpu {
    instruction_pointer: usize,
    instruction_count: usize,
    registers: [i16; 8],
    flags: [bool; 4],
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "insruction count: {}", self.instruction_count)?;
        writeln!(f, "instruction pointer: {}", self.instruction_pointer)?;

        let register_names = Reg::names();
        let registers_values = register_names
            .iter()
            .enumerate()
            .map(|(idx, name)| format!("{} {}", name, self.registers[idx]));
        let joined_registers = itertools::join(registers_values, "\n");
        writeln!(f, "registers:\n{}", joined_registers)?;


        let flag_names = Flags::names();
        let flag_values = flag_names
            .iter()
            .enumerate()
            .map(|(idx, name)| format!("{} {}", name, self.flags[idx]));
        let joined_flags = itertools::join(flag_values, "\n");
        writeln!(f, "flags:\n{}", joined_flags)?;
        Ok(())
    }
}
impl Cpu {
    pub fn exec(&mut self, instructions: Vec<Instruction>) {
        while self.instruction_pointer != instructions.len() {
            let next = instructions[self.instruction_pointer];
            println!("IP: {}, instruction: {:?}", self.instruction_pointer, next);
            self.exec_one(next);
        }
    }
    pub fn exec_one(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            MovImmToReg(dest, value) => {
                self.copy_to_register(dest, value);
            }
            MovRegToReg(dest, src) => self.copy_to_register(dest, self.read_register(src)),
            AddImmToReg(dest, value) => self.add_to_regsiter(dest, value),
            AddRegToReg(dest, src) => self.add_to_regsiter(dest, self.read_register(src)),
            SubImmToReg(dest, value) => self.sub_to_regsiter(dest, value),
            SubRegToReg(dest, src) => self.sub_to_regsiter(dest, self.read_register(src)),
            CmpImmToReg(dest, value) => self.cmp_to_regsiter(dest, value),
            CmpRegToReg(dest, src) => self.cmp_to_regsiter(dest, self.read_register(src)),
            Jnz(offset) => self.jump_if_nonzero(offset),
            _ => ()
        }
        self.instruction_count += 1;
    }

    fn jump_if_nonzero(&mut self, offset: i16) {
        self.instruction_pointer += 1;
        if !self.flags[Flags::Zero.index()] {
            self.instruction_pointer = (self.instruction_pointer as i16 + offset) as usize;
        }
    }
        
    fn copy_to_register(&mut self, register: Reg, value: i16) {
        self.instruction_pointer += 1;
        self.registers[register.index()] = value;
    }

    fn add_to_regsiter(&mut self, register: Reg, value: i16) {
        self.instruction_pointer += 1;
        self.registers[register.index()] += value;
        self.update_flags(self.read_register(register))
    }

    fn sub_to_regsiter(&mut self, register: Reg, value: i16) {
        self.instruction_pointer += 1;
        self.registers[register.index()] -= value;
        self.update_flags(self.read_register(register))
    }

    fn cmp_to_regsiter(&mut self, register: Reg, value: i16) {
        self.instruction_pointer += 1;
        let difference = self.registers[register.index()] - value;
        self.update_flags(difference)
    }

    fn update_flags(&mut self, register_value: i16) {
        self.flags[Flags::Zero.index()] = register_value == 0;
        self.flags[Flags::Sign.index()] = register_value < 0;
    }

    fn read_register(&self, register: Reg) -> i16 {
        self.registers[register.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let instruction = Instruction::MovImmToReg(Reg::BX, 50);
        let mut cpu = Cpu::default();
        cpu.exec_one(instruction);
        assert_eq!(cpu.read_register(Reg::BX), 50)
    }
}
