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

    pub fn space(&mut self) {
        self.word(" ");
    }

    pub fn eof(self) -> String {
        self.out
    }
}
