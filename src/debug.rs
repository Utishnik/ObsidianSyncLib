pub mod display_utils {
    use crate::debug;
    use crate::debug::debug_and_test_utils::reset_color_print;
    use crate::debug::debug_and_test_utils::set_color_print;
    use crate::debug_println;
    use core::fmt;
    use std::sync::LazyLock;
    pub const DEFAULT_OUTLINE: (char, char) = ('[', ']');
    pub struct OutLineFormatSymbols(char, char);
    pub fn set_scobes_format_syms(l_scobe: char, r_scobe: char) -> OutLineFormatSymbols {
        OutLineFormatSymbols(l_scobe, r_scobe)
    }
    impl Default for OutLineFormatSymbols {
        fn default() -> Self {
            Self(DEFAULT_OUTLINE.0, DEFAULT_OUTLINE.1)
        }
    }
    #[doc = "separator -> при записи как разделять элементы,
    outline -> как обоводить элементы(OutLineFormatSymbols),
    separator_width -> через сколько добавлять separator
    transfer -> через сколько переносы (writeln)
    lifetime separator и separator различны"]
    pub struct FormaterSliceFmt<'a, 'b> {
        separator: &'a str,
        outline: Option<&'b OutLineFormatSymbols>,
        separator_width: usize,
        transfer: usize,
    }
    static DEFAULT_FORMATER_SLICE_FMT: LazyLock<FormaterSliceFmt<'static, 'static>> =
        LazyLock::new(|| {
            let formater_default: FormaterSliceFmt<'_, '_> = FormaterSliceFmt::default();
            formater_default
        });
    pub struct PrintSliceFmt<'sep_lt, 'outln_lt, 'fmtslice_lt> {
        formater_slice_fmt: &'fmtslice_lt FormaterSliceFmt<'sep_lt, 'outln_lt>,
        color: Option<debug::debug_and_test_utils::Colors>,
    }

    impl<'sep_lt, 'outln_lt, 'fmtslice_lt> Default for PrintSliceFmt<'sep_lt, 'outln_lt, 'fmtslice_lt> {
        fn default() -> Self {
            Self {
                formater_slice_fmt: &DEFAULT_FORMATER_SLICE_FMT,
                color: None,
            }
        }
    }

    impl<'sep_lt, 'outln_lt, 'fmtslice_lt> PrintSliceFmt<'sep_lt, 'outln_lt, 'fmtslice_lt> {
        fn set(
            &mut self,
            formater_slice_fmt: &'fmtslice_lt FormaterSliceFmt<'sep_lt, 'outln_lt>,
            color: Option<debug::debug_and_test_utils::Colors>,
        ) {
            self.formater_slice_fmt = formater_slice_fmt;
            self.color = color;
        }
    }

    impl Default for FormaterSliceFmt<'_, '_> {
        fn default() -> Self {
            Self {
                separator: " ,",
                outline: Some(&OutLineFormatSymbols(DEFAULT_OUTLINE.0, DEFAULT_OUTLINE.1)),
                separator_width: 1,
                transfer: 0,
            }
        }
    }

    impl<'a, 'b> FormaterSliceFmt<'a, 'b> {
        fn set(
            &mut self,
            separator: &'a str,
            outline: Option<&'b OutLineFormatSymbols>,
            separator_width: usize,
            transfer: usize,
        ) {
            self.separator = separator;
            self.outline = outline;
            self.separator_width = separator_width;
            self.transfer = transfer;
        }
    }
    #[must_use]
    pub fn display_vec<T>(vec: &[T], fsf: &FormaterSliceFmt<'_, '_>) -> String
    where
        T: core::fmt::Display,
    {
        let items: Vec<String> = vec.iter().map(|x| x.to_string()).collect();
        if fsf.outline.is_none() {
            format!("{}", items.join(fsf.separator))
        } else {
            let scobes_unwrap: &OutLineFormatSymbols = unsafe { fsf.outline.unwrap_unchecked() };
            let l_scobe: char = scobes_unwrap.0;
            let r_scobe: char = scobes_unwrap.1;
            format!("{l_scobe}{}{r_scobe}", items.join(fsf.separator))
        }
    }

    #[doc = "нужно для \"правила сироты\" и для правил хранения правил 
    форматирования когда если T реализует trait Display мы реализуем
    для Vec<T> этот trait"]
    pub struct DisplayableVec<'a, T> {
        pub vec: Vec<T>,
        pub separator: &'a str,
    }

    #[doc = "если T реализует trait Display мы реализуем для Vec<T> этот trait"]
    impl<T> core::fmt::Display for DisplayableVec<'_, T>
    where
        T: core::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
            Ok(write!(f, "a")?) //загулшка
        }
    }

    #[doc = "для T у которого Vec<&T> есть Display trait"]
    pub fn write_slice_ref<T>(
        f: &mut core::fmt::Formatter<'_>,
        slice: &[T],
        start: usize,
        end: usize,
    ) -> fmt::Result
    where
        for<'a> Vec<&'a T>: core::fmt::Display,
    {
        Ok(write!(
            f,
            "{}",
            slice.iter().skip(start - 1).take(end).collect::<Vec<&T>>()
        )?)
    }
    #[doc = "для T у которого Vec<T> есть Display trait"]
    #[must_use]
    pub fn write_slice<T>(
        f: &mut core::fmt::Formatter<'_>,
        slice: &[T],
        start: usize,
        end: usize,
    ) -> fmt::Result
    where
        Vec<T>: core::fmt::Display,
        T: core::clone::Clone,
    {
        Ok(write!(
            f,
            "{}",
            slice
                .iter()
                .skip(start - 1)
                .take(end)
                .cloned()
                .collect::<Vec<T>>()
        )?)
    }
    #[doc = "почему в dispay_vec нужно FormaterSliceFmt а здесь передовать просто параметры которыми потом заполнится 
    он, ну так банально удобнее ну нужно если тебе просто вывести нужно значение каждый раз создавать а просто ввел нужное"]
    pub fn print_vec<T>(
        vec: &[T],
        separator: &str,
        outline: Option<OutLineFormatSymbols>,
        color: Option<debug::debug_and_test_utils::Colors>,
        separator_width: usize,
        transfer: usize,
    ) where
        T: std::fmt::Display,
    {
        let mut fsf: FormaterSliceFmt = FormaterSliceFmt::default();
        fsf.set(separator, outline.as_ref(), separator_width, transfer);
        if color.is_none() {
            let display: String = display_vec(vec, &fsf);
            debug_println!("{}", display);
        } else {
            let display: String = display_vec(vec, &fsf);
            let unwrap_color: debug::debug_and_test_utils::Colors =
                unsafe { color.unwrap_unchecked() };
            set_color_print(unwrap_color);
            debug_println!("{}", display);
            reset_color_print();
        }
    }
    #[must_use]
    pub fn display_vec_range<T>(
        vec: &[T],
        separator: &str,
        scobes: Option<OutLineFormatSymbols>,
        start: usize,
        end: usize,
    ) -> String
    where
        T: std::fmt::Display,
    {
        if start == end {
            return String::default();
        }
        let items: Vec<String> = vec
            .iter()
            .skip(start)
            .take(end)
            .map(|x| x.to_string())
            .collect();
        let vec_len: usize = vec.len();
        if scobes.is_none() {
            format!("{}", items.join(separator))
        } else {
            let scobes_unwrap: OutLineFormatSymbols = unsafe { scobes.unwrap_unchecked() };
            let l_scobe: char = scobes_unwrap.0;
            let r_scobe: char = scobes_unwrap.1;
            if start != 0 && end != vec_len {
                format!("..{l_scobe}{}{r_scobe}..", items.join(separator))
            } else if start == 0 {
                format!("{l_scobe}{}{r_scobe}..", items.join(separator))
            } else if start != 0 && end == vec_len {
                format!("..{l_scobe}{}{r_scobe}", items.join(separator))
            } else {
                set_color_print(debug::debug_and_test_utils::Colors::Blue);
                debug_println!("display_vec_range -> full range");
                reset_color_print();
                format!("{l_scobe}{}{r_scobe}", items.join(separator))
            }
        }
    }
    pub fn print_vec_range<T>(
        vec: &[T],
        separator: &str,
        scobes: Option<OutLineFormatSymbols>,
        start: usize,
        end: usize,
        color: Option<debug::debug_and_test_utils::Colors>,
    ) where
        T: std::fmt::Display,
    {
        if color.is_none() {
            let display: String = display_vec_range(vec, separator, scobes, start, end);
            debug_println!("{}", display);
        } else {
            let display: String = display_vec_range(vec, separator, scobes, start, end);
            let unwrap_color: debug::debug_and_test_utils::Colors =
                unsafe { color.unwrap_unchecked() };
            set_color_print(unwrap_color);
            debug_println!("{}", display);
            reset_color_print();
        }
    }
}

pub mod debug_trait_utils {}

pub mod debug_and_test_utils {
    use crate::bits_utils::size_bits;
    use crate::bits_utils::size_bytes;
    use crate::debug_println;
    use crate::debug_println_fileinfo;
    use std::fmt;
    use std::fs::File;
    use std::io::Write;
    use std::num::ParseIntError;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::OnceLock;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    pub struct ArgsFmt {
        args: Vec<String>,
        cnt_args: usize,
    }

    pub enum StreamPrint {
        StdOut,
        StdErr,
    }

    impl StreamPrint {
        pub fn as_str(&self) -> String {
            match self {
                StreamPrint::StdErr => "stderr".to_string(),
                StreamPrint::StdOut => "stdout".to_string(),
            }
        }
    }

    impl StreamPrint {
        pub fn str_to_self(&self, str: &str) -> Result<StreamPrint, ()> {
            let res: StreamPrint = match str {
                "stderr" => StreamPrint::StdErr,
                "stdout" => StreamPrint::StdOut,
                _ => {
                    return Err(());
                }
            };
            Ok(res)
        }
    }

    impl ArgsFmt {
        pub fn new() -> Self {
            Self {
                args: Vec::new(),
                cnt_args: 0,
            }
        }

        pub fn add<T: fmt::Display>(&mut self, arg: T) {
            self.args.push(format!("{}", arg));
            self.cnt_args += 1;
        }

        pub fn get_len(&self) -> usize {
            self.cnt_args
        }

        pub fn debug_print(&self) {
            debug_println!("ArgsFmt args: ");
            for i in self.args.iter() {
                debug_println!("{};", i);
            }
        }

        pub fn first(&self) -> Option<String> {
            self.args.first().cloned()
        } //todo использовать не только first но и другие значение например следуйщиее целое число
        //это сколько выводов показать или каких их цветом
        //patterns(несколько их может быть не только один вывод) после них количество выводов каждого патерна
        //I64SIZEMAX значит все i64 используется так как + значит первый n выводов - последние n выводов
        //и потом идут цвета каждого вывода
        //first будет означать кого типа выводить обычных stdout или поток ошибок

        pub fn get_args_owned(&self) -> Vec<String> {
            self.args.clone()
        }

        pub fn get_args_ref(&self) -> &[String] {
            &self.args
        }
    }

    impl Iterator for ArgsFmt {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            let len: usize = self.get_len();
            let mut i: usize = 0;
            'out: while i < len {
                let ret: Option<&String> = self.get_args_ref().get(i).clone();
                if ret.is_none() {
                    i += 1;
                    continue 'out;
                } else {
                    return Some(ret.map_or("zxc", |v| v).to_string());
                }
            }
            None
        }
    }

    impl Default for ArgsFmt {
        fn default() -> Self {
            Self::new()
        }
    }

    #[macro_export]
    macro_rules! argsfmt
    {
        ($($arg:expr),* $(,)?) =>
        {{
            use $crate::debug::debug_and_test_utils::ArgsFmt;
            let mut args = ArgsFmt::new();
            $(args.add($arg);)*
            args
        }};
    }

    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const RESET: &str = "\x1b[0m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";

    pub enum Colors {
        Red,
        Yellow,
        Green,
        Blue,
        Reset,
    }

    impl Colors {
        pub fn as_str(&self) -> String {
            match self {
                Self::Red => RED.to_string(),
                Self::Green => GREEN.to_string(),
                Self::Reset => RESET.to_string(),
                Self::Yellow => YELLOW.to_string(),
                Self::Blue => BLUE.to_string(),
                _ => todo!(),
            }
        }

        pub fn str_to_color(str: &str) -> Option<Self> {
            let result: Self = match str {
                RED => Self::Red,
                GREEN => Self::Green,
                RESET => Self::Reset,
                YELLOW => Self::Yellow,
                BLUE => Self::Blue,
                _ => return None,
            };
            Some(result)
        }
    }

    pub fn set_color_print(color: Colors) {
        print!("{}", color.as_str());
    }

    pub fn reset_color_print() {
        print!("\x1b[0m");
    }

    pub fn set_color_eprint(color: Colors) {
        eprint!("{}", color.as_str());
    }

    pub fn reset_color_eprint() {
        eprint!("\x1b[0m");
    }

    pub struct ParseCntColor {
        //todo инкапсуляция
        cnt: Vec<i64>,
        color: Vec<Colors>,
    }

    impl ParseCntColor {
        fn new() -> Self {
            Self {
                cnt: Vec::new(),
                color: Vec::new(),
            }
        }

        fn push_cnt(&mut self, val: i64) {
            self.cnt.push(val);
        }

        fn push_color(&mut self, val: Colors) {
            self.color.push(val);
        }
    }

    pub enum IterateParseError {
        PatternErr(String),
        ColorErr(String),
        CntErr(String),
    }

    pub struct IterateParse {
        end_pattern: Option<usize>,
        cnt_end: Option<usize>,
        color_end: Option<usize>,
    }

    impl IterateParse {
        pub fn new() -> Self {
            Self {
                end_pattern: None,
                cnt_end: None,
                color_end: None,
            }
        }

        pub fn set_end_pattern(&mut self, idx: Option<usize>) {
            self.end_pattern = idx;
        }

        pub fn set_cnt_end(&mut self, idx: Option<usize>) {
            self.cnt_end = idx;
        }

        pub fn set_color_end(&mut self, idx: Option<usize>) {
            self.color_end = idx;
        }

        pub fn get_end_pattern_idx(&self) -> Option<usize> {
            self.end_pattern
        }

        pub fn get_cnt_end_idx(&self) -> Option<usize> {
            self.cnt_end
        }

        pub fn get_color_end_idx(&self) -> Option<usize> {
            self.color_end
        }
    }

    impl Default for IterateParse {
        fn default() -> Self {
            Self::new()
        }
    }

    fn iterate_cnt_color_parse(args: &[String], color_err: &mut String) -> Option<ParseCntColor> {
        let mut result: ParseCntColor = ParseCntColor::new();
        let mut break_index: usize = 0;
        for i in args.iter().enumerate() {
            let (iter, str): (usize, &String) = i;
            let item: Result<i64, ParseIntError> = str.parse();
            if item.is_err() {
                break_index = iter;
                break;
            }
            let val: i64 = item.unwrap(); //safe
            result.push_cnt(val);
        }
        let mut parse_color: Colors;
        if break_index < 1 || args.len() < (break_index - 1) {
            debug_println!("break_index = {}\targs.len() = {}", break_index, args.len());
            return None; // тут надежда на обработку ошибки в функции которая
            //эту вызывает CntErr
        }
        let mut cnt_parse_color: usize = 0;
        'skip_col: for i in args.iter().skip(break_index - 1) {
            match Colors::str_to_color(i) {
                Some(color) => parse_color = color,
                None => {
                    if args.len() >= (break_index - 1) {
                        if args.len() == (break_index - 1) {
                            break 'skip_col; //цвета необязательны
                        }
                        if (args.len() - break_index + 1) != cnt_parse_color {
                            *color_err = format!(
                                "args.len() = {}  break_index = {}  cnt_parse_color = {}",
                                args.len(),
                                break_index,
                                cnt_parse_color
                            );
                            debug_println_fileinfo!("{}", color_err);
                        }
                    }
                    break;
                }
            }
            cnt_parse_color += 1;
            result.push_color(parse_color);
        } //тут чтоб обработать надо чекнуть
        Some(result)
    }

    fn iterate_pattern_parse(args: &[String]) -> Option<Vec<String>> {
        let mut result: Vec<String> = Vec::new();
        let mut index_break: usize = 0; //нужно для того чтобы знать сколько скипнуть при парсинг цветов и количества
        for item in args.iter().enumerate() {
            let (i, str): (usize, &String) = item;
            let item: Result<i64, ParseIntError> = str.parse();
            if item.is_ok() {
                index_break = i;
                break;
            }
            let val: &String = str;
            result.push(val.to_string());
        }
        if !result.is_empty() {
            return Some(result);
        }
        None
    }

    pub fn iterate_parse(args: &[String]) -> Result<(), IterateParseError> {
        let patter_result: Option<Vec<String>> = iterate_pattern_parse(args);
        let _match_pattern: Vec<String> = match patter_result {
            Some(x) => x,
            None => {
                return Err(IterateParseError::PatternErr(
                    "Empty pattern Vec".to_string(),
                ));
            }
        };
        Ok(())
    }

    //todo для всех debug print/eprint метод сделать ввод в фаил и возможность воспроизведения

    //todo функции серелизации и десерелизации stdout stderr

    pub fn fmt_args_parse(args: &[String]) -> Result<(), String> {
        let mut stream_type: StreamPrint = StreamPrint::StdOut;
        for item in args.to_vec().iter().enumerate() {
            let (i, str): (usize, &String) = item;
            if i == 0 {
                let state: Result<StreamPrint, ()> = stream_type.str_to_self(str);
                match state {
                    Ok(v) => stream_type = v,
                    Err(_) => {
                        return Err("parse stream error".to_string());
                    }
                }
            } else {
            }
        }
        Ok(())
    }

    fn private_printing_manage(args: fmt::Arguments) {
        //let patern: String = args.as_str().unwrap().to_string();//contains
        let argsfmt: ArgsFmt = argsfmt!(args);
        let _args: Vec<String> = argsfmt.get_args_owned();
        let patern: String = " ".to_string();
        let _str_args: String = patern.to_string();
        let _find_state: bool = false;
    }

    #[macro_export]
    macro_rules! printing_manage {
    ($($arg:tt)*) => {
        private_printing_manage(format_args!($($arg)*));
    };
}

    #[macro_export]
    macro_rules! debug_println {
    ($($arg:tt)*) =>
    {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}

    #[macro_export]
    macro_rules! debug_eprintln {
    ($($arg:tt)*) =>
    {
        #[cfg(debug_assertions)]
        {
            eprintln!($($arg)*);
        }
    };
}

    #[macro_export]
    macro_rules! debug_println_fileinfo {
    ($($arg:tt)*) =>
    {
        #[cfg(debug_assertions)]
        {
            println!("[ {} : {} ]",file!(),line!());
            println!($($arg)*);
        }
    };
}

    #[macro_export]
    macro_rules! debug_eprintln_fileinfo {
    ($($arg:tt)*) =>
    {
        #[cfg(debug_assertions)]
        {
            eprintln!("[ {} : {} ]",file!(),line!());
            eprintln!($($arg)*);
        }
    };
}
    //думаю клонирование дешевле либо +- также чем удерживать блокировку
    #[derive(Clone)]
    pub struct Test {
        pub result: bool,
        pub number: usize,
        pub typetest: usize,
        //pub cnt_types: usize,
    }

    //maybe многопоточное тестирования хз нужно?
    pub static TESTS: OnceLock<Arc<Mutex<Vec<Test>>>> = OnceLock::new();
    pub static type_test: AtomicUsize = AtomicUsize::new(0);

    fn get_tests() -> &'static Arc<Mutex<Vec<Test>>> {
        TESTS.get_or_init(|| Arc::new(Mutex::new(Vec::new())))
    }

    pub fn get_count_tests() -> usize {
        let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
        let guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
        let cnt: usize = guard.len();
        cnt
    }

    pub fn clear_console() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn successes_beh() {
        let mut i: usize = 0;
        let cnt_test: usize = get_count_tests();
        let mut suc_test: usize = 0;
        let mut not_suc_test: usize = 0;
        //c like
        while i < cnt_test {
            let g_test: Option<Test> = get_test(i);
            match g_test {
                None => {}
                Some(x) => {
                    if x.result {
                        suc_test += 1;
                    } else {
                        not_suc_test += 1;
                    }
                }
            }
            i += 1;
        }
        let mut diff: usize = 0;
        if suc_test > not_suc_test {
            diff = suc_test - not_suc_test;
        } else {
            diff = not_suc_test - suc_test;
        }
        if not_suc_test > 0 {
            std::process::exit(diff.try_into().unwrap_or(1));
        }
    }

    pub fn add_test(test: Test) {
        let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
        let mut guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
        guard.push(test);
    }

    pub fn add_type_test() {
        type_test.fetch_add(1, Ordering::Release);
    }

    pub fn get_type_test() -> usize {
        type_test.load(Ordering::Acquire)
    }

    pub fn get_test(index: usize) -> Option<Test> {
        let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
        let guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
        let test: Option<Test> = guard.get(index).cloned();
        test
    }

    //todo get_test_slice

    pub fn get_last_test() -> Option<Test> {
        let tests: &Arc<Mutex<Vec<Test>>> = get_tests();
        let guard: std::sync::MutexGuard<'_, Vec<Test>> = tests.lock().unwrap();
        guard.last().cloned()
    }

    pub fn result_list() -> bool {
        let cnt_test: usize = get_count_tests();
        let mut i: usize = 0;
        const RED: &str = "\x1b[31m"; //todo по идеи можно заменить на константы глобальные в самом фаиле
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";
        let mut successfully: bool = true;
        //c like
        while i < cnt_test {
            let g_test: Option<Test> = get_test(i);
            match g_test {
                None => {}
                Some(x) => {
                    if x.result {
                        println!("{}", GREEN);
                    } else {
                        println!("{}", RED);
                        successfully = false;
                    }
                    println!("index: {} --- status: {}", x.number, x.result);
                    print!("{}", RESET);
                }
            }
            i += 1;
        }
        successfully
    }

    pub fn dump_result_list(path: String) -> Result<(), std::io::Error> //todo: сделать асинхронной? типо тестов может быть очень много
//и результаты всех нужно дампать
//с другой стороны тут большая логическая нагрзука а не просто запросить данные
    {
        let mut file: File = File::create(path)?;
        let cnt_test: usize = get_count_tests();
        const RED: &str = "\x1b[31m"; //todo по идеи можно заменить на константы глобальные в самом фаиле
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";
        let mut i: usize = 0;

        while i < cnt_test {
            let g_test: Option<Test> = get_test(i);
            match g_test {
                None => {}
                Some(x) => {
                    if x.result {
                        file.write_fmt(format_args!("{}", GREEN))?;
                    } else {
                        file.write_fmt(format_args!("{}", RED))?;
                    }
                    file.write_fmt(format_args!("index: {} --- status: {}", x.number, x.result))?;
                    file.write_fmt(format_args!("{}", RESET))?;
                }
            }
            i += 1;
        }
        Ok(()) //TODO доделать там нумерацию фаилов пред проверки всякие и тд и наверное чтоб он автоматически взависомти от
        //теста в нужную директорию
    }

    #[derive(Clone, PartialEq)]
    pub enum BitsDisplay {
        All,
        DEC,
        HEX,
        BIN,
    }

    pub fn print_bits_detailed<T>(value: T, label: &str, filter: BitsDisplay)
    where
        T: std::fmt::Binary + std::fmt::Display + Copy + std::fmt::UpperHex,
    {
        let size_bytes: usize = size_bytes::<T>();
        let size_bits: usize = size_bits::<T>();

        let mut print_dec: bool = filter == BitsDisplay::DEC;
        let mut print_hex: bool = filter == BitsDisplay::HEX;
        let mut print_bin: bool = filter == BitsDisplay::BIN;
        if filter == BitsDisplay::All {
            print_dec = true;
            print_hex = true;
            print_bin = true;
        }

        debug_println!("{}:", label);
        if print_dec {
            debug_println!("  DEC: {}", value);
        }
        if print_hex {
            debug_println!("  HEX: 0x{:X}", value);
        }
        if print_bin {
            debug_println!("  BIN: {:0width$b}", value, width = size_bits);
        }
    }

    pub fn print_bits_formating<T>(value: T, label: &str, split_width: usize, separator: &str)
    where
        T: std::fmt::Binary + std::fmt::Display + Copy + std::fmt::UpperHex,
    {
        let size_bytes: usize = size_bytes::<T>();
        let size_bits: usize = size_bits::<T>();

        if split_width >= size_bits {
            set_color_eprint(Colors::Red);
            debug_eprintln_fileinfo!("print_bits_detailed  split_width >= size_bits");
            reset_color_eprint();
            return;
        }
        let binary_str: String = format!("{:0width$b}", value, width = size_bits);
        let with_spaces: String = binary_str
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i > 0 && i % split_width == 0 {
                    format!("{separator}{}", c)
                } else {
                    c.to_string()
                }
            })
            .collect();

        debug_println!("  С разделителями: {}", with_spaces);
        debug_println!("  Размер: {} байт ({} бит)", size_bytes, size_bits);
    }

    fn format_hex_with_separators<T>(value: T, chunk_size: usize, separator: &str) -> String
    where
        T: std::fmt::LowerHex,
    {
        let hex_string: String = format!("{:x}", value);
        let mut result: String = String::with_capacity(
            hex_string.len() + (hex_string.len() / chunk_size) * separator.len(),
        );

        let first_chunk_len: usize = hex_string.len() % chunk_size;
        let first_chunk_len: usize = if first_chunk_len == 0 {
            chunk_size
        } else {
            first_chunk_len
        };

        result.push_str(&hex_string[..first_chunk_len]);

        for chunk in hex_string[first_chunk_len..].as_bytes().chunks(chunk_size) {
            result.push_str(separator);
            result.push_str(&String::from_utf8_lossy(chunk));
        }
        result
    }

    pub fn print_hex_with_separators<T>(value: T, chunk_size: usize, separator: &str)
    where
        T: std::fmt::LowerHex,
    {
        let size_bytes: usize = size_bytes::<T>();
        let size_bits: usize = size_bits::<T>();
        let format_hex_res: String = format_hex_with_separators(value, chunk_size, separator);
        debug_println!("  С разделителями: {}", format_hex_res);
        debug_println!("  Размер: {} байт ({} бит)", size_bytes, size_bits);
    }
    //todo: pub fn dump_list_print

    //ебаный раст засирает assert_eq поэтому пишем свой
    #[macro_export]
    macro_rules! test_assert {
    ($left:expr,$right:expr) => {
        match (&$left, &$right)
        {
            (left_val, right_val) =>
            {
                const RED: &str = "\x1b[31m";
                const GREEN: &str = "\x1b[32m";
                const YELLOW: &str = "\x1b[33m";
                const RESET: &str = "\x1b[0m";
                let mut result_test: bool = false;
                use $crate::debug::debug_and_test_utils::Test;
                use $crate::debug::debug_and_test_utils::get_last_test;
                use $crate::debug::debug_and_test_utils::get_type_test;
                use $crate::debug::debug_and_test_utils::add_test;
                use std::sync::atomic::Ordering;
                //$crate::debug::* может так лучше?
                let last_test_option: Option<Test> = get_last_test();
                let mut test: Test;
                match last_test_option
                {
                    None =>
                    {
                        test = Test {result: false, number: 1,typetest: 0};
                    },
                    Some(x) =>
                    {
                        test=x.clone();
                        test.number=x.number+1;
                    }
                }

                if !(*left_val == *right_val)
                {
                    println!();
                    println!("{}══════════════════════════════════════════════════════════{}", RED, RESET);
                    println!("{} ✗ ТЕСТ НЕ ПРОЙДЕН{}", RED, RESET);
                    println!("{} Ожидалось: {:?}{}", YELLOW, right_val, RESET);
                    println!("{} Получено:  {:?}{}", RED, left_val, RESET);
                    println!("{}══════════════════════════════════════════════════════════{}\n", RED, RESET);
                    result_test=false;
                    //todo чекнуть почему при eprintln! может ломматься вывод
                }
                else
                {
                    println!();
                    println!("{}══════════════════════════════════════════════════════════{}", GREEN, RESET);
                    println!("{} ✔ ТЕСТ ПРОЙДЕН{}", GREEN, RESET);
                    println!("{} Ожидалось: {:?}{}", YELLOW, right_val, RESET);
                    println!("{} Получено:  {:?}{}", GREEN, left_val, RESET);
                    println!("{}══════════════════════════════════════════════════════════{}\n", GREEN, RESET);
                    result_test=true;
                }
                test.result=result_test;
                test.typetest=get_type_test();
                add_test(test);
            }
        }
    };
}
}
