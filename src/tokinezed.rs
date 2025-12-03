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
