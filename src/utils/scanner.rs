pub struct Scanner {
    source: String,
    tokens: Vec<super::token::Token>
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: "".to_string(),
            tokens: Vec::new(),
        }
    }
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source, ..Default::default() }
    }

    pub fn scan_for_tokens(&self) {
        println!("{}", self.source);
    }
}