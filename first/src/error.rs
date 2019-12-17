pub enum RError {
    Nothing,
    Tokenize(usize, String),
}

impl RError {
    pub fn build_error_message<'a>(&self, input: &'a str) -> String {
        match self {
            Self::Tokenize(i, s) => {
                let mut out = format!("{}\n", input);
                for _ in 0..*i {
                    out += " ";
                }
                out += format!("^ {}\n", s).as_str();
                out
            }
            _ => "Unexpected Error".to_string(),
        }
    }
}
