pub struct Instance {
    arr: [u8; 30_000],
    pointer: usize,
    out: Vec<u8>
}

impl Instance {
    pub fn new () -> Self {
        Self {
            arr: [0_u8; 30_000],
            pointer: 0,
            out: Vec::new() 
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

    fn enter_loop (&mut self, idx: &mut usize, chars: &Vec<char>) -> Result<(), String> {
        if self.arr[self.pointer] == 0 {
            // Skip over the loop
            let mut temp = 1;
            while temp > 0 {
                if *idx >= chars.len() - 1 {
                    return Err(String::from("Loop was never closed")); 
                }
                *idx += 1;
                if chars[*idx] == ']' { temp -= 1; }
                else if chars[*idx] == '[' {temp += 1; }
            }
        }
        Ok(())
    }

    fn exit_loop (&mut self, idx: &mut usize, chars: &Vec<char>) -> Result<(), String> {
        if self.arr[self.pointer] != 0 {
            // Find where the loop started
            let mut temp = 1;
            while temp > 0 {
                if *idx == 0 {
                    return Err(String::from("Empty loop close statement")); 
                }
                *idx -= 1;
                if chars[*idx] == ']' { temp += 1; }
                else if chars[*idx] == '[' { temp -= 1; }
            }
        }
        Ok(())
    }

    fn out (&self) -> u8 {
        self.arr[self.pointer]
    }

    pub fn update (&mut self, val: &String) -> Result<(), String> {
        let mut new: Vec<u8> = self.interpret_to_vec(val)?;
        self.out.append(&mut new);
        Ok(())
    }

    fn interpret_to_vec (&mut self, val: &String) -> Result<Vec<u8>, String> {
        let mut ret: Vec<u8> = Vec::new();
        let chars: Vec<char> = val.chars().collect();

        let mut parsing_idx: usize = 0;
        while parsing_idx < chars.len() {
            match chars[parsing_idx] {
                '+' => self.inc()?,
                '-' => self.dec()?,
                '>' => self.move_right()?,
                '<' => self.move_left()?,
                '.' => ret.push(self.out()),
                '[' => self.enter_loop(&mut parsing_idx, &chars)?,
                ']' => self.exit_loop(&mut parsing_idx, &chars)?,
                _ => {}
            }
            parsing_idx += 1;
        }

        Ok(ret)
    }

    pub fn get_ascii (&mut self) -> String {
        let mut ret = String::new();
        for c in &self.out {
            ret.push(*c as char);
        }
        ret
    }
}

