    #[derive(Debug, Clone, PartialEq)]
    pub enum Token 
    {
        Email,
        AccTok,
        RemoteRepAddr,
        SetVal,
        PathObsidian,
        TimeCommit,
        TextCommit, 
        UserName,
        IteratorStart,
        IteratorEnd,
    }

    pub fn get_symbol(str: &str,index: usize) -> Option<char>
    {
        return str.chars().nth(index);
    }

    pub fn skip_symbol(str: &str,index: &mut usize,symbol_list: String) -> bool
    { 
        let chr: Option<char> = get_symbol(str, *index);
        let value: char;
        match chr
        {
            None =>
            {
                return false;
            }
            Some(x) =>
            {
                value=x;
            }
        }

        for s in symbol_list.chars()
        {
            if value == s
            {
                *index+=1;
                return true;
            }
        }        
    false
    }

    //todo функция скип символом пока не дойдет до определенного
    pub fn skip_construction(str: &str,index: &mut usize,ignore_symbol_list: &str,construction: String) -> bool
    {
        let mut idx: usize=0;
        let mut iter: usize=0;
        let len_str: usize = str.len();
        let len_construction: usize = construction.len();
        loop 
        {
            if !skip_symbol(str, index, ignore_symbol_list.to_string())  
            {
                if iter > len_str-1
                {
                    return false;
                }
                let option_str: Option<char> = get_symbol(str, iter);
                let give_sym_str: char;
                match  option_str
                {
                    None => {},//невзможно из за проверки переполнения
                    Some(x) =>
                    {
                        give_sym_str=x;
                    }
                }
                if idx > len_construction-1
                {
                    return false;
                }
                let option_construction: Option<char> = get_symbol(str, idx);
                let give_sym_construction: char;
                match  option_construction
                {
                    None => {},//невзможно из за проверки переполнения
                    Some(x) =>
                    {
                        give_sym_construction=x;
                    }
                }
            }
            iter+=1;
        }
        false
    }

    pub struct Token_String
    {
        pub tok_start : usize,
        pub tok_end : usize,
        pub tok_val : String,
    }

    pub struct TokenStruct
    {
        pub tok_values : Vec<String>,
        pub tok_lines_number : Vec<u64>,
    }

    impl TokenStruct 
    {
        pub fn new(size:usize) -> Self 
        {
            Self 
            {
                tok_values: vec![String::new(); size], 
                tok_lines_number: Vec::<u64>::with_capacity(size),
            }
        }

        pub fn get_size(&self) -> usize
        {
           self.tok_values.len()
        }
    }

    impl Token {
        pub fn as_str(&self) -> &'static str {
            match self {
                Token::Email => "Email",
                Token::AccTok => "Token",
                Token::UserName => "UserName",
                Token::RemoteRepAddr => "Remote",
                Token::SetVal => "=",
                Token::PathObsidian => "Path",
                Token::TimeCommit => "Time",
                Token::TextCommit => "Text",
                Token::IteratorStart => "{{",
                Token::IteratorEnd => "}}",
            }
        }
    }
