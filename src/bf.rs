pub struct Instance {
    arr: [u8; 30_000],
    pointer: usize,
}

impl Instance {
    pub fn new () -> Self {
        Self {
            arr: [0_u8; 30_000],
            pointer: 0,
        }
    }

    fn move_right (&mut self) -> Result<(), String> {
        if self.pointer == self.arr.len() - 1 {
            return Err(String::from("Right fencepost error"));
        }
        self.pointer += 1;
        Ok(())
    }

    fn move_left (&mut self) -> Result<(), String> {
        if self.pointer == 0 {
            return Err(String::from("Left fencepost error"));
        }
        self.pointer -= 1;
        Ok(())
    }

    fn inc (&mut self) -> Result<(), String> {
        if self.arr[self.pointer] == 255 {
            return Err(format!("Overflow at index {}", &self.pointer))
        }
        self.arr[self.pointer] += 1;
        Ok(())
    }

    fn dec (&mut self) -> Result<(), String> {
        if self.arr[self.pointer] == 0 {
            return Err(format!("Underflow at index {}", &self.pointer));
        }
        self.arr[self.pointer] -= 1;
        Ok(())
    }

    fn out (&self) -> u8 {
        self.arr[self.pointer]
    }

    pub fn interpret_to_vec (&mut self, val: &str) -> Result<Vec<u8>, String> {
        let mut ret: Vec<u8> = Vec::new();
        let chars: Vec<char> = val.chars().collect();

        let mut idx: usize = 0;
        while idx < chars.len() {
            match chars[idx] {
                '+' => self.inc()?,
                '-' => self.dec()?,
                '>' => self.move_right()?,
                '<' => self.move_left()?,
                '.' => ret.push(self.out()),
                '[' => {
                    if self.arr[self.pointer] == 0 {
                        let mut temp = 1;
                        while temp > 0 {
                            if idx >= chars.len() - 1 { return Err(String::from("Loop was never closed")); }
                            idx += 1;
                            if chars[idx] == ']' { temp -= 1; }
                            else if chars[idx] == '[' {temp += 1; }
                        }
                    }
                },
                ']' => {
                    if self.arr[self.pointer] != 0 {
                        let mut temp = 1;
                        while temp > 0 {
                            if idx <= 0 { return Err(String::from("Empty loop close statement")); }
                            idx -= 1;
                            if chars[idx] == ']' { temp += 1; }
                            else if chars[idx] == '[' {temp -= 1; }
                        }
                    }
                }
                _ => {}
            }
            idx += 1;
        }

        Ok(ret)
    }

    pub fn interpret_to_ascii(&mut self, val: &str) -> Result<String, String> {
        let vec = self.interpret_to_vec(val)?;
        let mut ret = String::new();
        for c in vec {
            ret.push(c as char);
        }
        Ok(ret)
    }
}

