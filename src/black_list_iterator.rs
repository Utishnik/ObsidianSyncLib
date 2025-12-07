use crate::black_list_iterator;

pub struct AsciiSymbol
{
    current: u8,//number
    black_list: String,
}

impl AsciiSymbol
{
    pub fn new(black_list: String) -> Self
    {
        AsciiSymbol { current: 0 , black_list: black_list}
    }
}

//O(n*m) худщий случай O(n^2)=16 384
//NOT BLAZING
impl Iterator for AsciiSymbol
{
    type Item = char;
    fn next(&mut self) -> Option<char> 
    {
        'label1:
        while self.current < 128
        {
            let byte: u8 = self.current;
            self.current+=1;
            for b in self.black_list.chars()
            {
                if byte as char == b
                {
                    continue 'label1;
                }
            }
            return Some(byte as char); 
        }
        None
    }
}