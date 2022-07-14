#[derive(Clone)]
pub struct Source {
    pub src: Vec<char>,
    pub pos: usize,
}

impl Source {
    pub fn get_char(&mut self, check: impl Fn(char) -> bool) -> Result<char, String> {
        match self.get_next() {
            Ok(c) => {
                if check(c) {
                    Ok(c)
                } else {
                    self.pos -= 1;
                    Err(String::from("Not found."))
                }
            }

            Err(text) => Err(text),
        }
    }

    pub fn get_string(&mut self, string: String) -> Result<String, String> {
        let first_pos = self.pos;
        for i in 0..string.chars().count() {
            if self.has_next() {
                match self.get_next() {
                    Ok(c) => {
                        if c == string.chars().nth(i).unwrap() {
                            continue;
                        } else {
                            self.pos = first_pos;
                            return Err(String::from(""));
                        }
                    }
                    Err(_) => {}
                }
            } else {
                self.pos = first_pos;
                return Err(String::from(""));
            }
        }

        Ok(string)
    }

    pub fn get_chars(&mut self, check: impl Fn(char) -> bool) -> Result<String, String> {
        let mut buffer = String::from("");
        while self.has_next() {
            match self.get_next() {
                Ok(c) => {
                    if check(c) {
                        buffer += &c.to_string();
                    } else {
                        self.pos -= 1;
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }

        if buffer == "" {
            Err(String::from("Not found."))
        } else {
            Ok(buffer)
        }
    }

    pub fn get_next(&mut self) -> Result<char, String> {
        self.pos += 1;
        if self.src.len() > self.pos - 1 {
            Ok(self.src[self.pos - 1])
        } else {
            Err(String::from("EOF"))
        }
    }

    pub fn new(src: String) -> Source {
        Source {
            src: src.chars().collect(),
            pos: 0,
        }
    }

    pub fn has_next(&self) -> bool {
        self.src.len() > self.pos
    }
}
