pub struct AsciiSymbol {
    current: u8, //number
    black_list: String,
}

impl AsciiSymbol {
    pub fn new(black_list: String) -> Self {
        AsciiSymbol {
            current: 0,
            black_list: black_list,
        }
    }

    pub fn set(&mut self, black_list: String, cur: u8) {
        self.current = cur;
        self.black_list = black_list;
    }

    pub fn get(&self) -> Self {
        Self {
            current: self.current,
            black_list: self.black_list.clone(),
        }
    }
}

//O(n*m) худщий случай O(n^2)=16 384
//NOT BLAZING
impl Iterator for AsciiSymbol {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        'label1: while self.current < 128 {
            let byte: u8 = self.current;
            self.current += 1;
            for b in self.black_list.chars() {
                if byte as char == b {
                    continue 'label1;
                }
            }
            return Some(byte as char);
        }
        None
    }
}
