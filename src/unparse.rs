pub struct Printer {
    out: String,
}

impl Printer {
    pub fn new() -> Self {
        Printer { out: String::new() }
    }

    pub fn word(&mut self, string: &str) {
        self.out.push_str(string);
    }

    pub fn character(&mut self, ch: char) {
        self.out.push(ch);
    }

    pub fn hardbreak(&mut self) {
        self.out.push('\n');
    }

    pub fn eof(self) -> String {
        self.out
    }
}
