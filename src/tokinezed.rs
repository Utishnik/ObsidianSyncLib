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

    pub fn skip_symbol(str: String,index: usize,symbol_list: String) -> bool
    {
        let mut is_skip_sym = |sym: char| -> bool
        {
            for s in symbol_list.chars()
            {
                let chr: Option<char> = get_symbol(&str, index);
            }
            true
        };
        true
    }
    //todo функция скип символом пока не дойдет до определенного

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
