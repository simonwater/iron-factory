pub struct CPU {
    position_in_memory: usize,
    registers: [u8; 16],
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            position_in_memory: 0,
            registers: [0; 16],
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            let nnn = opcode & 0x0FFF;
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    pub fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
    }

    pub fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        self.registers[0xF] = if overflow { 1 } else { 0 }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack overflow!");
        }
        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.registers[3] = 10;

        let mem = &mut cpu.memory;
        mem[0] = 0x80;
        mem[1] = 0x14;
        mem[2] = 0x80;
        mem[3] = 0x24;
        mem[4] = 0x80;
        mem[5] = 0x34;

        cpu.run();
        assert_eq!(cpu.registers[0], 35);
        println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
    }

    #[test]
    fn laod_func_test() {
        let mut cpu = CPU::new();
        let add_twice: [u8; 6] = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE]; // 函数定义

        let mem = &mut cpu.memory;
        mem[0x100..0x106].copy_from_slice(&add_twice); // 函数加载到内存中
        println!("{:?}", &mem[0x100..0x106]);
    }

    #[test]
    fn call_func_test() {
        let mut cpu = CPU::new();
        let mem = &mut cpu.memory;
        // 函数定义
        let add_twice: [u8; 6] = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];
        // 函数加载到内存中
        mem[0x100..0x106].copy_from_slice(&add_twice);

        // 设置寄存器值
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        // 设置操作码0x2100：在0x100处调用函数
        mem[0x000] = 0x21;
        mem[0x001] = 0x00;
        // 设置操作码0x2100：在0x100处调用函数
        mem[0x002] = 0x21;
        mem[0x003] = 0x00;
        // 设置操作码0x0000：停止执行
        mem[0x004] = 0x00;
        mem[0x005] = 0x00;

        cpu.run();
        assert_eq!(cpu.registers[0], 45);
        println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
    }
}
