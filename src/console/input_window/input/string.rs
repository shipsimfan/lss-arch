use super::Input;

pub struct StringInput {
    label: &'static str,

    bytes: Box<[u8]>,
    length: usize,
}

impl StringInput {
    pub fn new(label: &'static str, max_length: usize) -> Self {
        StringInput {
            label,
            bytes: vec![b'_'; max_length].into_boxed_slice(),
            length: 0,
        }
    }

    pub fn unwrap(self) -> String {
        let mut bytes = self.bytes.into_vec();
        unsafe { bytes.set_len(self.length) };
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

impl Input for StringInput {
    fn label(&self) -> &str {
        self.label
    }

    fn text(&self) -> &[u8] {
        &self.bytes
    }

    fn index(&self) -> usize {
        self.length.min(self.bytes.len() - 1)
    }

    fn backspace(&mut self) {
        if self.length == 0 {
            return;
        }

        self.length -= 1;
        self.bytes[self.length] = b'_';
    }

    fn char(&mut self, c: u8) {
        if self.length == self.bytes.len() {
            return;
        }

        self.bytes[self.length] = c;
        self.length += 1;
    }
}
