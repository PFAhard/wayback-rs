#[derive(Debug)]
pub struct Otx {
    value: Vec<String>,
    buf: String,
    brc: bool,
    state: u8,
}

impl Otx {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&mut self, c: char) {
        if !['\n', '\t', ' '].contains(&c) {
            print!("{c}");
            match self.state {
                0 => {
                    assert_eq!(c, '[');
                    self.state += 1;
                }
                1 => {
                    assert_eq!(c, '{');
                    self.state += 1;
                }
                2 => {
                    assert_eq!(c, '"');
                    self.state += 1;
                }
                3 => {
                    if c == 'u' {
                        self.state += 1;
                    }
                }
                4 => {
                    if c == 'r' {
                        self.state += 1;
                    }
                }
                5 => {
                    if c == 'l' {
                        self.state += 1;
                    }
                }

                6 => {
                    if c == '"' {
                        self.state += 1;
                    }
                }
                7 => {
                    assert_eq!(c, ':');
                    self.state += 1;
                }
                8 => {
                    if c == '"' {
                        self.state += 1;
                    }
                }
                9 => {
                    if c != '"' {
                        self.buf.push(c);
                    } else {
                        self.value.push(self.buf.clone());
                        self.buf.clear();
                        self.state += 1;
                    }
                }
                10 => {
                    assert_eq!(c, ',');
                    self.state += 1;
                }
                11 => {
                    if c == '{' {
                        self.state += 1;
                    } else if c == '}' {
                        self.state = 13;
                    }
                }
                12 => {
                    if c == '}' {
                        self.state -= 1;
                    }
                }
                13 => {
                    if c == ',' {
                        self.state = 1;
                    } else {
                        assert_eq!(c, ']');
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
}

impl Default for Otx {
    fn default() -> Self {
        Self {
            value: Default::default(),
            buf: String::with_capacity(256),
            state: Default::default(),
        }
    }
}

impl From<Otx> for Vec<String> {
    fn from(otx: Otx) -> Self {
        otx.value
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::otx_stream::Otx;

//     use super::SAMPLE;

//     #[test]
//     fn test() {
//         let mut otx = Otx::new();
//         for c in SAMPLE.chars() {
//             otx.parse(c)
//         }
//         dbg!(otx.value);
//     }
// }
