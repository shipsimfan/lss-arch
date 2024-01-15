use super::Input;

pub struct U8Input {
    label: &'static str,

    bytes: [u8; 3],

    length: usize,
}

impl U8Input {
    pub fn new(label: &'static str) -> Self {
        U8Input {
            label,
            bytes: [b'_'; 3],
            length: 0,
        }
    }

    fn value(&self) -> u16 {
        let mut value = 0;
        for i in 0..self.length {
            value *= 10;
            value += (self.bytes[i] - b'0') as u16;
        }

        value
    }

    pub fn unwrap(self) -> Option<u8> {
        if self.length == 0 {
            None
        } else {
            Some(self.value() as u8)
        }
    }
}

impl Input for U8Input {
    fn label(&self) -> &str {
        self.label
    }

    fn text(&self) -> &[u8] {
        &self.bytes
    }

    fn index(&self) -> usize {
        self.length.min(2)
    }

    fn backspace(&mut self) {
        if self.length == 0 {
            return;
        }

        self.length -= 1;
        self.bytes[self.length] = b'_';
    }

    fn char(&mut self, c: u8) {
        if self.length == 3 || !c.is_ascii_digit() {
            return;
        }

        self.bytes[self.index()] = c;
        self.length += 1;

        if self.value() > u8::MAX as u16 {
            self.backspace();
        }
    }
}
