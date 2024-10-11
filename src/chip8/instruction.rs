pub type Address = usize;

pub enum Instruction {
    CLS,
    RET,
    SYS(Address),
    JP(Address),
    CALL(Address),
    SEVxByte(usize, u8),
    SNEVxByte(usize, u8),
    SEVxVy(usize, usize),
    LDVxByte(usize, u8),
    ADDVxByte(usize, u8),
    LDVxVy(usize, usize),
    ORVxVy(usize, usize),
    ANDVxVy(usize, usize),
    XORVxVy(usize, usize),
    ADDVxVy(usize, usize),
    SUBVxVy(usize, usize),
    SHRVx(usize),
    SUBNVxVy(usize, usize),
    SHLVx(usize),
    SNEVxVy(usize, usize),
    LDI(Address),
    JP0(Address),
    RNDVxByte(usize, u8),
    DRWVxVyNibble(usize, usize, u8),
    SKPVx(usize),
    SKNPVx(usize),
    LDVxDT(usize),
    LDVxK(usize),
    LDDTVx(usize),
    LDSTVx(usize),
    ADDIVx(usize),
    LDFVx(usize),
    LDBVx(usize),
    LDIVx(usize),
    LDVxMem(usize),
    Unknown,
}

impl Instruction {
    pub fn from_opcode(opcode: u16) -> Self {
        use Instruction::*;

        let first_nibble = (opcode >> 12) & 0xF;
        let x = ((opcode >> 8) & 0xF) as usize;
        let y = ((opcode >> 4) & 0xF) as usize;
        let kk = (opcode & 0xFF) as u8;
        let nnn = (opcode & 0xFFF) as Address;

        match first_nibble {
            0x0 => match opcode {
                0x00E0 => CLS,
                0x00EE => RET,
                _ => SYS(nnn),
            },
            0x1 => JP(nnn),
            0x2 => CALL(nnn),
            0x3 => SEVxByte(x, kk),
            0x4 => SNEVxByte(x, kk),
            0x5 => SEVxVy(x, y),
            0x6 => LDVxByte(x, kk),
            0x7 => ADDVxByte(x, kk),
            0x8 => {
                let last_nibble = opcode & 0xF;
                match last_nibble {
                    0x0 => LDVxVy(x, y),
                    0x1 => ORVxVy(x, y),
                    0x2 => ANDVxVy(x, y),
                    0x3 => XORVxVy(x, y),
                    0x4 => ADDVxVy(x, y),
                    0x5 => SUBVxVy(x, y),
                    0x6 => SHRVx(x),
                    0x7 => SUBNVxVy(x, y),
                    0xE => SHLVx(x),
                    _ => Unknown,
                }
            }
            0x9 => SNEVxVy(x, y),
            0xA => LDI(nnn),
            0xB => JP0(nnn),
            0xC => RNDVxByte(x, kk),
            0xD => DRWVxVyNibble(x, y, (opcode & 0x000F) as u8),
            0xE => match kk {
                0x9E => SKPVx(x),
                0xA1 => SKNPVx(x),
                _ => Unknown,
            },
            0xF => match kk {
                0x07 => LDVxDT(x),
                0x0A => LDVxK(x),
                0x15 => LDDTVx(x),
                0x18 => LDSTVx(x),
                0x1E => ADDIVx(x),
                0x29 => LDFVx(x),
                0x33 => LDBVx(x),
                0x55 => LDIVx(x),
                0x65 => LDVxMem(x),
                _ => Unknown,
            },
            _ => Unknown,
        }
    }

    pub fn disassemble(&self) -> String {
        use Instruction::*;

        match self {
            CLS => "CLS".to_string(),
            RET => "RET".to_string(),
            SYS(address) => format!("SYS {:03X}", address),
            JP(address) => format!("JP {:03X}", address),
            CALL(address) => format!("CALL {:03X}", address),
            SEVxByte(x, byte) => format!("SE V{:X}, {:02X}", x, byte),
            SNEVxByte(x, byte) => format!("SNE V{:X}, {:02X}", x, byte),
            SEVxVy(x, y) => format!("SE V{:X}, V{:X}", x, y),
            LDVxByte(x, byte) => format!("LD V{:X}, {:02X}", x, byte),
            ADDVxByte(x, byte) => format!("ADD V{:X}, {:02X}", x, byte),
            LDVxVy(x, y) => format!("LD V{:X}, V{:X}", x, y),
            ORVxVy(x, y) => format!("OR V{:X}, V{:X}", x, y),
            ANDVxVy(x, y) => format!("AND V{:X}, V{:X}", x, y),
            XORVxVy(x, y) => format!("XOR V{:X}, V{:X}", x, y),
            ADDVxVy(x, y) => format!("ADD V{:X}, V{:X}", x, y),
            SUBVxVy(x, y) => format!("SUB V{:X}, V{:X}", x, y),
            SHRVx(x) => format!("SHR V{:X}", x),
            SUBNVxVy(x, y) => format!("SUBN V{:X}, V{:X}", x, y),
            SHLVx(x) => format!("SHL V{:X}", x),
            SNEVxVy(x, y) => format!("SNE V{:X}, V{:X}", x, y),
            LDI(address) => format!("LD I, {:03X}", address),
            JP0(address) => format!("JP V0, {:03X}", address),
            RNDVxByte(x, byte) => format!("RND V{:X}, {:02X}", x, byte),
            DRWVxVyNibble(x, y, n) => format!("DRW V{:X}, V{:X}, {:X}", x, y, n),
            SKPVx(x) => format!("SKP V{:X}", x),
            SKNPVx(x) => format!("SKNP V{:X}", x),
            LDVxDT(x) => format!("LD V{:X}, DT", x),
            LDVxK(x) => format!("LD V{:X}, K", x),
            LDDTVx(x) => format!("LD DT, V{:X}", x),
            LDSTVx(x) => format!("LD ST, V{:X}", x),
            ADDIVx(x) => format!("ADD I, V{:X}", x),
            LDFVx(x) => format!("LD F, V{:X}", x),
            LDBVx(x) => format!("LD B, V{:X}", x),
            LDIVx(x) => format!("LD [I], V{:X}", x),
            LDVxMem(x) => format!("LD V{:X}, [I]", x),
            Unknown => "Unknown".to_string(),
        }
    }
}
