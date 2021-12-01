pub struct Intcode<'a> {
    pub memory: &'a mut Vec<usize>,
    pub counter: usize,
}

impl Intcode<'_> {
    pub fn exec_cycle(&mut self) -> bool {
        if self.counter >= self.memory.len() || self.memory[self.counter] != 99 {
            let x = self.memory[self.counter + 1];
            let y = self.memory[self.counter + 2];
            let n = self.memory[self.counter + 3];
            match self.memory[self.counter] {
                1 => {
                    self.memory[n] = self.memory[x] + self.memory[y];
                    self.counter += 4;
                }
                2 => {
                    self.memory[n] = self.memory[x] * self.memory[y];
                    self.counter += 4;
                }
                _ => panic!("unexpected opcode"),
            }
            true
        } else {
            false
        }
    }
}
