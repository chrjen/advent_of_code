use std::fmt::Write;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Cpu represents the main processor for the chronospatial computer and
/// can be used to emulate the processor in software. After initialising the
/// struct you can call the [`Cpu::tick`] method to emulate a single clock
/// cycle. By calling the method over and over you can run entire programs.
pub struct Cpu {
    pub pc: usize,
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

impl Cpu {
    /// Creates and returns a new cpu with the registers A, B and C initialised to
    /// the given values. The program count is always initialised to 0.
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        Self { pc: 0, a, b, c }
    }

    /// Forwards the cpu state a single clock cycle (i.e. a tick). The cpu will always
    /// execute a whole instruction every clock cycle. The cpu may optionally produce an
    /// output as the `Ok()` variant of the result if an OUT instruction was executed, if
    /// no output `Ok(None)` is returned instead. If the cpu encounters an error such as
    /// malformed input or trying to access memory outside the available memory region
    /// (indicating a halt) then the error is returned instead.
    pub fn tick(&mut self, memory: &[u8]) -> Result<Option<u8>, CpuError> {
        let opcode: Opcode = (*memory.get(self.pc).ok_or(CpuError::Halt)?).try_into()?;
        let operand: u8 = *memory.get(self.pc + 1).ok_or(CpuError::NoOperand)?;
        self.pc += 2;

        #[allow(clippy::assign_op_pattern)]
        match (opcode, operand) {
            (Opcode::Adv, 0..=3) => self.a = self.a / u64::pow(2, operand as u32),
            (Opcode::Adv, 4) => self.a = 0,
            (Opcode::Adv, 5) => self.a = self.a / u64::pow(2, self.b as u32),
            (Opcode::Adv, 6) => self.a = self.a / u64::pow(2, self.c as u32),
            (Opcode::Adv, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
            (Opcode::Bxl, _) => self.b = self.b ^ operand as u64,
            (Opcode::Bst, 0..=3) => self.b = operand as u64,
            (Opcode::Bst, 4) => self.b = self.a % 8,
            (Opcode::Bst, 5) => self.b = self.b % 8,
            (Opcode::Bst, 6) => self.b = self.c % 8,
            (Opcode::Bst, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
            (Opcode::Jnz, _) => {
                if self.a != 0 {
                    self.pc = operand as usize;
                }
            }
            (Opcode::Bxc, _) => self.b = self.b ^ self.c,
            (Opcode::Out, 0..=3) => return Ok(Some(operand)),
            (Opcode::Out, 4) => return Ok(Some((self.a % 8) as u8)),
            (Opcode::Out, 5) => return Ok(Some((self.b % 8) as u8)),
            (Opcode::Out, 6) => return Ok(Some((self.c % 8) as u8)),
            (Opcode::Out, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
            (Opcode::Bdv, 0..=3) => self.b = self.a / u64::pow(2, operand as u32),
            (Opcode::Bdv, 4) => self.b = 0,
            (Opcode::Bdv, 5) => self.b = self.a / u64::pow(2, self.b as u32),
            (Opcode::Bdv, 6) => self.b = self.a / u64::pow(2, self.c as u32),
            (Opcode::Bdv, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
            (Opcode::Cdv, 0..=3) => self.c = self.a / u64::pow(2, operand as u32),
            (Opcode::Cdv, 4) => self.c = 0,
            (Opcode::Cdv, 5) => self.c = self.a / u64::pow(2, self.b as u32),
            (Opcode::Cdv, 6) => self.c = self.a / u64::pow(2, self.c as u32),
            (Opcode::Cdv, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
        }

        Ok(None)
    }

    #[allow(dead_code)]
    /// Reads the program in memory and produces the assembly for the program.
    /// May return an error instead if the program is malformed.
    pub fn disassemble(memory: &[u8], comments: bool) -> Result<String, CpuError> {
        let mut assembly = String::new();
        let mut pc = 0;

        loop {
            let opcode: Opcode = match memory.get(pc).ok_or(CpuError::Halt) {
                Ok(&byte) => byte.try_into()?,
                Err(CpuError::Halt) => break,
                Err(e) => return Err(e),
            };
            let operand: u8 = *memory.get(pc + 1).ok_or(CpuError::NoOperand)?;
            pc += 2;

            if comments {
                let _ = match (opcode, operand) {
                    (Opcode::Adv, 0..=3) => writeln!(assembly, "; A = A / {operand}"),
                    (Opcode::Adv, 4) => writeln!(assembly, "; A = 0"),
                    (Opcode::Adv, 5) => writeln!(assembly, "; A = A / 2^B"),
                    (Opcode::Adv, 6) => writeln!(assembly, "; A = A / 2^C"),
                    (Opcode::Adv, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
                    (Opcode::Bxl, _) => writeln!(assembly, "; B = B xor {operand}"),
                    (Opcode::Bst, 0..=3) => writeln!(assembly, "; B = {operand}"),
                    (Opcode::Bst, 4) => writeln!(assembly, "; B = A mod 8"),
                    (Opcode::Bst, 5) => writeln!(assembly, "; B = B mod 8"),
                    (Opcode::Bst, 6) => writeln!(assembly, "; B = C mod 8"),
                    (Opcode::Bst, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
                    (Opcode::Jnz, _) => writeln!(assembly, "; Jump to {operand} (if A != 0)"),

                    (Opcode::Bxc, _) => writeln!(assembly, "; B = B xor C"),
                    (Opcode::Out, 0..=3) => writeln!(assembly, "; Print {operand}"),
                    (Opcode::Out, 4) => writeln!(assembly, "; Print (A mod 8)"),
                    (Opcode::Out, 5) => writeln!(assembly, "; Print (B mod 8)"),
                    (Opcode::Out, 6) => writeln!(assembly, "; Print (C mod 8)"),
                    (Opcode::Out, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
                    (Opcode::Bdv, 0..=3) => writeln!(assembly, "; B = A / {operand}"),
                    (Opcode::Bdv, 4) => writeln!(assembly, "; B = 0"),
                    (Opcode::Bdv, 5) => writeln!(assembly, "; B = A / 2^B"),
                    (Opcode::Bdv, 6) => writeln!(assembly, "; B = A / 2^C"),
                    (Opcode::Bdv, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
                    (Opcode::Cdv, 0..=3) => writeln!(assembly, "; C = A / {operand}"),
                    (Opcode::Cdv, 4) => writeln!(assembly, "; C = 0"),
                    (Opcode::Cdv, 5) => writeln!(assembly, "; C = A / 2^B"),
                    (Opcode::Cdv, 6) => writeln!(assembly, "; C = A / 2^C"),
                    (Opcode::Cdv, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
                };
            }

            let _ = match (opcode, operand) {
                (Opcode::Adv, 0..=3) => writeln!(assembly, "ADV {operand}"),
                (Opcode::Adv, 4) => writeln!(assembly, "ADV A"),
                (Opcode::Adv, 5) => writeln!(assembly, "ADV B"),
                (Opcode::Adv, 6) => writeln!(assembly, "ADV C"),
                (Opcode::Adv, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
                (Opcode::Bxl, _) => writeln!(assembly, "BXL {operand}"),
                (Opcode::Bst, 0..=3) => writeln!(assembly, "BST {operand}"),
                (Opcode::Bst, 4) => writeln!(assembly, "BST A"),
                (Opcode::Bst, 5) => writeln!(assembly, "BST B"),
                (Opcode::Bst, 6) => writeln!(assembly, "BST C"),
                (Opcode::Bst, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
                (Opcode::Jnz, _) => writeln!(assembly, "JNZ {operand}"),

                (Opcode::Bxc, _) => writeln!(assembly, "BXC {operand}"),
                (Opcode::Out, 0..=3) => writeln!(assembly, "OUT {operand}"),
                (Opcode::Out, 4) => writeln!(assembly, "OUT A"),
                (Opcode::Out, 5) => writeln!(assembly, "OUT B"),
                (Opcode::Out, 6) => writeln!(assembly, "OUT C"),
                (Opcode::Out, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
                (Opcode::Bdv, 0..=3) => writeln!(assembly, "BDV {operand}"),
                (Opcode::Bdv, 4) => writeln!(assembly, "BDV A"),
                (Opcode::Bdv, 5) => writeln!(assembly, "BDV B"),
                (Opcode::Bdv, 6) => writeln!(assembly, "BDV C"),
                (Opcode::Bdv, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
                (Opcode::Cdv, 0..=3) => writeln!(assembly, "CDV {operand}"),
                (Opcode::Cdv, 4) => writeln!(assembly, "CDV A"),
                (Opcode::Cdv, 5) => writeln!(assembly, "CDV B"),
                (Opcode::Cdv, 6) => writeln!(assembly, "CDV C"),
                (Opcode::Cdv, _) => return Err(CpuError::IllegalOperand(opcode, operand)),
            };
        }

        Ok(assembly)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuError {
    Halt,
    IllegalOpcode(u8),
    NoOperand,
    IllegalOperand(Opcode, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl TryFrom<u8> for Opcode {
    type Error = CpuError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Adv),
            1 => Ok(Opcode::Bxl),
            2 => Ok(Opcode::Bst),
            3 => Ok(Opcode::Jnz),
            4 => Ok(Opcode::Bxc),
            5 => Ok(Opcode::Out),
            6 => Ok(Opcode::Bdv),
            7 => Ok(Opcode::Cdv),
            _ => Err(CpuError::IllegalOpcode(value)),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(val: Opcode) -> Self {
        val as u8
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use rstest::rstest;

    use super::*;

    #[test]
    fn test_halt() {
        let mut cpu = Cpu::default();
        assert_eq!(cpu.tick(&mut []), Err(CpuError::Halt));
    }

    #[test]
    fn test_illegal_opcode() {
        let mut cpu = Cpu::default();
        assert_eq!(cpu.tick(&mut [8, 99]), Err(CpuError::IllegalOpcode(8)));
    }

    #[test]
    fn test_no_operand() {
        let mut cpu = Cpu::default();
        assert_eq!(cpu.tick(&mut [0]), Err(CpuError::NoOperand));
    }

    #[rstest]
    #[case(0, 5, 5)]
    #[case(1, 10, 5)]
    #[case(1, 11, 5)]
    fn test_adv_instr_combo(#[case] operand: u8, #[case] a_in: u64, #[case] a_out: u64) {
        let mut cpu = Cpu::new(a_in, 0, 0);
        assert!(cpu.tick(&mut [0, operand]).is_ok());
        assert_eq!(cpu.a, a_out);
    }

    #[test]
    fn test_adv_instr_combo_a() {
        let mut cpu = Cpu::new(2, 0, 0);
        assert!(cpu.tick(&mut [0, 4]).is_ok());
        assert_eq!(cpu.a, 0);
    }

    #[test]
    fn test_adv_instr_combo_b() {
        let mut cpu = Cpu::new(12, 2, 0);
        assert!(cpu.tick(&mut [0, 5]).is_ok());
        assert_eq!(cpu.a, 3);
    }

    #[test]
    fn test_adv_instr_combo_c() {
        let mut cpu = Cpu::new(12, 0, 2);
        assert!(cpu.tick(&mut [0, 6]).is_ok());
        assert_eq!(cpu.a, 3);
    }

    #[rstest]
    #[case(0, 5, 5)]
    #[case(7, 5, 2)]
    #[case(2, 7, 5)]
    fn test_bxl_instr(#[case] operand: u8, #[case] b_in: u64, #[case] b_out: u64) {
        let mut cpu = Cpu::new(0, b_in, 0);
        assert!(cpu.tick(&mut [1, operand]).is_ok());
        assert_eq!(cpu.b, b_out);
    }

    #[test]
    fn test_bst_instr_combo() {
        let mut cpu = Cpu::new(0, 0, 0);
        assert!(cpu.tick(&mut [2, 2]).is_ok());
        assert_eq!(cpu.b, 2);
    }

    #[test]
    fn test_bst_instr_combo_a() {
        let mut cpu = Cpu::new(12, 0, 0);
        assert!(cpu.tick(&mut [2, 4]).is_ok());
        assert_eq!(cpu.b, 4);
    }

    #[test]
    fn test_bst_instr_combo_b() {
        let mut cpu = Cpu::new(0, 12, 0);
        assert!(cpu.tick(&mut [2, 5]).is_ok());
        assert_eq!(cpu.b, 4);
    }

    #[test]
    fn test_bst_instr_combo_c() {
        let mut cpu = Cpu::new(0, 0, 12);
        assert!(cpu.tick(&mut [2, 6]).is_ok());
        assert_eq!(cpu.b, 4);
    }

    #[test]
    fn test_jnz_instr_a_zero() {
        let mut cpu = Cpu::new(0, 0, 0);
        assert!(cpu.tick(&mut [3, 6]).is_ok());
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_jnz_instr_a_not_zero() {
        let mut cpu = Cpu::new(5, 0, 0);
        assert!(cpu.tick(&mut [3, 6]).is_ok());
        assert_eq!(cpu.pc, 6);
    }

    #[rstest]
    #[case(0, 5, 5)]
    #[case(7, 5, 2)]
    #[case(2, 7, 5)]
    fn test_bxc_instr(#[case] b_in: u64, #[case] c_in: u64, #[case] b_out: u64) {
        let mut cpu = Cpu::new(0, b_in, c_in);
        assert!(cpu.tick(&mut [4, 0]).is_ok());
        assert_eq!(cpu.b, b_out);
    }

    #[test]
    fn test_out_instr_combo() {
        let mut cpu = Cpu::new(0, 0, 0);
        assert!(
            cpu.tick(&mut [5, 2])
                .is_ok_and(|v| v.is_some_and(|v| v == 2))
        );
    }

    #[test]
    fn test_out_instr_combo_a() {
        let mut cpu = Cpu::new(12, 0, 0);
        assert!(
            cpu.tick(&mut [5, 4])
                .is_ok_and(|v| v.is_some_and(|v| v == 4))
        );
    }

    #[test]
    fn test_out_instr_combo_b() {
        let mut cpu = Cpu::new(0, 12, 0);
        assert!(
            cpu.tick(&mut [5, 5])
                .is_ok_and(|v| v.is_some_and(|v| v == 4))
        );
    }

    #[test]
    fn test_out_instr_combo_c() {
        let mut cpu = Cpu::new(0, 0, 12);
        assert!(
            cpu.tick(&mut [5, 6])
                .is_ok_and(|v| v.is_some_and(|v| v == 4))
        );
    }

    #[rstest]
    #[case(0, 5, 5)]
    #[case(1, 10, 5)]
    #[case(1, 11, 5)]
    fn test_bdv_instr(#[case] operand: u8, #[case] a_in: u64, #[case] b_out: u64) {
        let mut cpu = Cpu::new(a_in, 0, 0);
        assert!(cpu.tick(&mut [6, operand]).is_ok());
        assert_eq!(cpu.b, b_out);
    }

    #[test]
    fn test_bdv_instr_combo_a() {
        let mut cpu = Cpu::new(2, 0, 0);
        assert!(cpu.tick(&mut [6, 4]).is_ok());
        assert_eq!(cpu.b, 0);
    }

    #[test]
    fn test_bdv_instr_combo_b() {
        let mut cpu = Cpu::new(12, 2, 0);
        assert!(cpu.tick(&mut [6, 5]).is_ok());
        assert_eq!(cpu.b, 3);
    }

    #[test]
    fn test_bdv_instr_combo_c() {
        let mut cpu = Cpu::new(12, 0, 2);
        assert!(cpu.tick(&mut [6, 6]).is_ok());
        assert_eq!(cpu.b, 3);
    }

    #[rstest]
    #[case(0, 5, 5)]
    #[case(1, 10, 5)]
    #[case(1, 11, 5)]
    fn test_cdv_instr(#[case] operand: u8, #[case] a_in: u64, #[case] c_out: u64) {
        let mut cpu = Cpu::new(a_in, 0, 0);
        assert!(cpu.tick(&mut [7, operand]).is_ok());
        assert_eq!(cpu.c, c_out);
    }

    #[test]
    fn test_cdv_instr_combo_a() {
        let mut cpu = Cpu::new(2, 0, 0);
        assert!(cpu.tick(&mut [7, 4]).is_ok());
        assert_eq!(cpu.c, 0);
    }

    #[test]
    fn test_cdv_instr_combo_b() {
        let mut cpu = Cpu::new(12, 2, 0);
        assert!(cpu.tick(&mut [7, 5]).is_ok());
        assert_eq!(cpu.c, 3);
    }

    #[test]
    fn test_cdv_instr_combo_c() {
        let mut cpu = Cpu::new(12, 0, 2);
        assert!(cpu.tick(&mut [7, 6]).is_ok());
        assert_eq!(cpu.c, 3);
    }

    fn run_until_halt(cpu: &mut Cpu, memory: &mut [u8], max_ticks: usize) -> Vec<u8> {
        let mut output = Vec::new();

        for ticks in 0.. {
            if ticks == max_ticks {
                panic!("cpu took too long");
            }
            match cpu.tick(memory) {
                Ok(Some(out)) => output.push(out),
                Ok(None) => { /* Do nothing. */ }
                Err(CpuError::Halt) => break,
                Err(e) => panic!("got cpu error {e:?}"),
            }
        }

        output
    }

    #[test]
    fn p1_example_1() {
        let mut cpu = Cpu::new(0, 0, 9);
        run_until_halt(&mut cpu, &mut [2, 6], 1000);
        assert_eq!(cpu.b, 1);
    }

    #[test]
    fn p1_example_2() {
        let mut cpu = Cpu::new(10, 0, 0);
        let output = run_until_halt(&mut cpu, &mut [5, 0, 5, 1, 5, 4], 1000);
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn p1_example_3() {
        let mut cpu = Cpu::new(2024, 0, 0);
        let output = run_until_halt(&mut cpu, &mut [0, 1, 5, 4, 3, 0], 1000);
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(cpu.a, 0);
    }

    #[test]
    fn p1_example_4() {
        let mut cpu = Cpu::new(0, 29, 0);
        run_until_halt(&mut cpu, &mut [1, 7], 1000);
        assert_eq!(cpu.b, 26);
    }

    #[test]
    fn p1_example_5() {
        let mut cpu = Cpu::new(0, 2024, 43690);
        run_until_halt(&mut cpu, &mut [4, 0], 1000);
        assert_eq!(cpu.b, 44354);
    }
}
