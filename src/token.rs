// Deefines the Token struct
struct Token {
    kind: String,
    value: String,
}

// Implement methods for the Token struct
impl Token {
    fn new(kind: &str, value: &str) -> Token {
        Token {
            kind: kind.to_string(),
            value: value.to_int(),
        }
    }

    fn display(&self) {
        println!("Token kind: {}, value: {}", self.kind, self.value);
    }
}