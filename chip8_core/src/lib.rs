const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const RAM_SIZE: usize = 4096; //4KB
const START_ADDRESS: u16 = 0x200; //512 decimal

const NUM_REGISTERS: usize = 16;

const STACK_SIZE: usize = 16;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    v_registers: [u8; NUM_REGISTERS], //general purpose register
    i_registers: u16,
    stack: [u16; STACK_SIZE], //hold mem address of the next instruction
    stack_pointer: u16,
    delay_timer: u8,
    sound_timer: u8,
    screen: [bool; WIDTH * HEIGHT],
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDRESS,
            ram: [0; RAM_SIZE],
            v_registers: [0; NUM_REGISTERS],
            i_registers: 0,
            stack: [0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            screen: [false; WIDTH * HEIGHT],
            stack_pointer: 0,
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    pub fn push(&mut self, val: u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    /// This  This method resets the emulator to its initial state,
    /// clearing registers, the stack, the screen,
    /// and reloading the font sprite data
    pub fn reset(&mut self) {}

    pub fn tick(&mut self) {
        //fetch opcode
        let opcode = self.fetch();
        //decode opcode and execute
        self.execute(opcode);
    }

    fn fetch(&mut self) -> u16 {
        // Example: If we have two consecutive bytes in RAM:
        // ram[pc]   = 0xA2 (10100010 in binary)
        // ram[pc+1] = 0xF0 (11110000 in binary)

        // 1. Read first byte and shift left 8 bits
        // 0xA2 << 8 = 0xA200 (1010001000000000 in binary)
        let higher_byte = (self.ram[self.pc as usize] as u16) << 8;

        // 2. Read second byte
        // 0xF0 = 11110000
        let lower_byte = self.ram[self.pc as usize + 1] as u16;

        // 3. Combine using bitwise OR
        // 0xA200 | 0xF0 = 0xA2F0
        // 1010001000000000 |
        // 0000000011110000 =
        // 1010001011110000
        let op = higher_byte | lower_byte;

        self.pc += 2; // Increment PC to next instruction
        op
    }

    fn execute(opcode: u16) {
        // todo!()
        let digit1 = (opcode & 0xF000) >> 12; // First digit
        let digit2 = (opcode & 0x0F00) >> 8; // Second digit
        let digit3 = (opcode & 0x00F0) >> 4; // Third digit
        let digit4 = opcode & 0x000F; // Fourth digit
    }
}
