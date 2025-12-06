    use crate::debug;
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

    fn check_construction(ignore_symbol_list: &str,construction: &str) -> Option<Vec<usize>>
    {
        let mut idx: usize=0;
        let mut collision: Vec<usize> = Vec::new();
        for items in construction.chars().enumerate()
        {
            let (s_i,s): (usize, char) = items;
            for c in ignore_symbol_list.chars()
            {
                if c == s
                {
                    collision[idx]=s_i;
                    idx+=1;
                }
            }
        }
        if collision.is_empty()
        {
            return None;
        }
        else 
        {
            return Some(collision);
        }
    }

    pub struct construction
    {
        start: usize,
        end: usize
    }

    //todo функция скип символом пока не дойдет до определенного
    pub fn skip_construction(str: &str,index: &mut usize,ignore_symbol_list: &str,construction: &str) -> bool
    {
        let mut idx: usize=0;
        let mut iter: usize=0;
        let len_str: usize = str.len();
        let len_construction: usize = construction.len();
        let option_collision_constuction: Option<Vec<usize>> = check_construction(ignore_symbol_list,construction);
        let mut collision_construction: Vec<usize>;
        let mut constuction_skiping: construction = construction { start: 0, end: 0 };//todo доделать щас нужно пофиксить основной loop
        match option_collision_constuction
        {
            None => {},
            Some(x) =>
            {
                collision_construction=x;
            }
        }
        if len_str < len_construction
        {
            return false;
        }
        crate::debug_println!("\n\n\n");
        loop 
        {
            if !skip_symbol(str, index, ignore_symbol_list.to_string()) //todo нужно соглосовать с collision_construction
            {
                if *index > len_str-1
                {
                    return false;
                }
                let option_str: Option<char> = get_symbol(str, *index);
                let mut give_sym_str: char='a';
                match  option_str
                {
                    None => {},//невзможно из за проверки переполнения
                    Some(x) =>
                    {
                        give_sym_str=x;
                    }
                }
                let option_construction: Option<char> = get_symbol(&construction, iter);
                crate::debug_println_fileinfo!("iter:   {}     index:    {}",iter,index);
                let mut give_sym_construction: char='b';
                match option_construction
                {
                    None => {},//невозможно из за проверки переполнения
                    Some(x) =>
                    {
                        give_sym_construction=x;
                    }
                }

                print!("construct: {} src: {}  index: {}\n",give_sym_construction,give_sym_str,iter);
                if give_sym_construction == give_sym_str
                {
                    if iter == len_construction-1
                    {
                        return true;
                    }
                }
                *index+=1;
                iter+=1;
            }
        }
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
