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

const NUM_KEYS: usize = 16;

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
    keys: [bool; NUM_KEYS],
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
            keys: [false; NUM_KEYS],
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDRESS as usize;
        let end = (START_ADDRESS as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
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

    fn execute(&mut self, opcode: u16) {
        // todo!()
        let digit1 = (opcode & 0xF000) >> 12; // First digit
        let digit2 = (opcode & 0x0F00) >> 8; // Second digit
        let digit3 = (opcode & 0x00F0) >> 4; // Third digit
        let digit4 = opcode & 0x000F; // Fourth digit

        match (digit1, digit2, digit3, digit4) {
            //NOP
            (0, 0, 0, 0) => return,
            //CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; WIDTH * HEIGHT];
            }
            //RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }
            //JMP NNN
            (1, _, _, _) => {
                let nnn = opcode & 0x0FFF; // 12-bits address
                self.pc = nnn; // Jump to address
            }
            //2NNN - CALL addr
            (2, _, _, _) => {
                let nnn = opcode & 0x0FFF; // 12-bits address
                self.push(self.pc); // add current program counter to stack
                self.pc = nnn; // Jump to address
            }
            //3XNN
            (3, _, _, _) => {
                let nn = (opcode & 0x0FF) as u8; // 12-bits address
                let x = digit2 as usize;
                if self.v_registers[x] == nn {
                    self.pc += 2;
                }
            }
            //4XNN
            (4, _, _, _) => {
                let nn = (opcode & 0x0FF) as u8; // 12-bits address
                let x = digit2 as usize;
                if self.v_registers[x] != nn {
                    self.pc += 2;
                }
            }
            //5XY0
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_registers[x] == self.v_registers[y] {
                    //skip next instruction
                    self.pc += 2;
                }
            }
            //6XNN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (opcode & 0x0FF) as u8; // 12-bits address
                self.v_registers[x] = nn;
            }
            //7XNN
            //Adds the value NN to register VX.
            //If the addition results in an overflow,
            //the value wraps around, and the carry flag (VF) is not affected
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (opcode & 0x0FF) as u8; // 12-bits address
                self.v_registers[x] = self.v_registers[x].wrapping_add(nn);
            }
            //8XY0
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_registers[y] = self.v_registers[x];
            }
            //8XY1
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_registers[y] |= self.v_registers[x];
            }
            //8XY2
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_registers[y] &= self.v_registers[x];
            }
            //8XY3
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_registers[y] ^= self.v_registers[x];
            }
            // 8XY4
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.v_registers[x].overflowing_add(self.v_registers[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.v_registers[x] = new_vx;
                self.v_registers[0xF] = new_vf;
            }
            // 8XY5
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_registers[x].overflowing_sub(self.v_registers[y]);
                let new_vf = if borrow { 1 } else { 0 };

                self.v_registers[x] = new_vx;
                self.v_registers[0xF] = new_vf;
            }
            // 8XY6
            (8, _, _, 6) => {
                // This operation performs a single right shift on the value in VX, with the bit that was dropped off being stored into the VF register.
                let x = digit2 as usize;
                let lsb = self.v_registers[x] & 1;
                self.v_registers[x] >>= 1;
                self.v_registers[0xF] = lsb;
            }
            // 8XY7
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_registers[y].overflowing_sub(self.v_registers[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_registers[x] = new_vx;
                self.v_registers[0xF] = new_vf;
            }
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                let msb = (self.v_registers[x] >> 7) & 1;
                self.v_registers[x] <<= 1;
                self.v_registers[0xF] = msb;
            }
            //9XY0
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_registers[x] != self.v_registers[y] {
                    self.pc += 2;
                }
            }
            //ANNN
            (0xA, _, _, _) => {
                let nnn = opcode & 0x0FFF; // 12-bits address
                self.i_registers = nnn;
            }
            //BNNN
            (0xB, _, _, _) => {
                let nnn = opcode & 0x0FFF; // 12-bits address
                self.pc = nnn + self.v_registers[0] as u16;
            }
            //CXNN
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (opcode & 0x0FF) as u8; // 12-bits address
                let random_byte: u8 = rand::random();
                self.v_registers[x] = random_byte & nn;
            }
            //DXYN
            // Draws a sprite at the coordinates specified by the values
            // in registers VX and VY.
            //  The sprite is 0xN pixels tall,
            // and its data is read from memory starting at the address stored
            // in the I register.
            // The VF flag register is set if any pixels are flipped from
            // set to unset or unset to set during the drawing process.
            (0xD, _, _, _) => {
                let x = digit2 as usize;
                let x_coord = self.v_registers[x] as u16;
                let y = digit3 as usize;
                let y_coord = self.v_registers[y] as u16;

                let num_of_rows = digit4;
            }
            //EX9E
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_registers[x];
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 1) => {
                //skip key if not pressed
                let x = digit2 as usize;
                let vx = self.v_registers[x];
                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
            }
            (0xF, _, 0, 7) => {
                self.v_registers[digit2 as usize] = self.delay_timer;
            }
            (0xF, _, 0, A) => {
                let x = digit2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_registers[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.pc -= 2; //redo opcode
                }
            }
            (0xF, _, 1, 5) => {
                self.delay_timer = self.v_registers[digit2 as usize];
            }
            (0xF, _, 1, 8) => {
                self.sound_timer = self.v_registers[digit2 as usize];
            }
            (0xF, _, 1, 0xE) => {
                self.i_registers = self
                    .i_registers
                    .wrapping_add(self.v_registers[digit2 as usize] as u16);
            }
            (0xF, _, 2, 9) => {
                //set i to font address
                let c = self.v_registers[digit2 as usize] as u16;
                self.i_registers = c * 5;
            }
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let vx = self.v_registers[x] as f32;

                let digit_100 = (vx / 100.0).floor() as u8;
                let digit_10 = ((vx / 10.0) % 10.0).floor() as u8;
                let digit_1 = (vx % 10.0) as u8;

                self.ram[self.i_registers as usize] = digit_100;
                self.ram[(self.i_registers + 1) as usize] = digit_10;
                self.ram[(self.i_registers + 2) as usize] = digit_1;
            }
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let i = self.i_registers as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_registers[idx];
                }
            }
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.i_registers as usize;
                for idx in 0..=x {
                    self.v_registers[i + idx] = self.ram[idx];
                }
            }
            _ => {
                todo!();
            }
        }
    }
}
