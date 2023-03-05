pub struct Parser {
    pub chars: Vec<char>,
    pub index: usize,
}

impl Parser {
    pub fn new(code: String) -> Parser {
        Parser {
            chars: code.chars().collect(),
            index: 0
        }
    }
}
