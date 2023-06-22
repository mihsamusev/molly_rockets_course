use std::{io::Result, fs, env};

use simulate::{cpu::Cpu, instruction::Instruction};

fn parse_instructions(instrution_str: &str) -> Vec<Instruction> {
    instrution_str.lines()
        .filter_map(|line| line.parse::<Instruction>().ok())
        .collect()
}

fn main() -> Result<()> {
    let filename = env::args().skip(1).next().expect("bs");
    let instruction_string = fs::read_to_string(filename)?;
    let instructions = parse_instructions(&instruction_string);
    let mut cpu = Cpu::default();
    cpu.exec(instructions);
    println!("{}", cpu);
    Ok(())
}