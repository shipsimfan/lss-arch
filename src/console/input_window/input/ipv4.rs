use super::Input;
use net_utils::ip::v4::IPv4Address;

pub struct IPv4Input {
    label: &'static str,

    bytes: [u8; 15],

    length: usize,
}

impl IPv4Input {
    pub fn new(label: &'static str) -> Self {
        IPv4Input {
            label,
            bytes: *b"___.___.___.___",
            length: 0,
        }
    }

    fn current_group(&self) -> usize {
        self.length / 4
    }

    fn group(&self, group: usize) -> &[u8] {
        let index = group * 4;
        &self.bytes[index..index + 3]
    }

    fn group_value(&self, group: usize) -> u16 {
        let mut value = 0;
        for byte in self.group(group) {
            if *byte == b'_' {
                break;
            }

            value *= 10;
            value += (*byte - b'0') as u16;
        }

        value
    }

    fn current_group_value(&self) -> u16 {
        self.group_value(self.current_group())
    }

    pub fn unwrap(self) -> IPv4Address {
        IPv4Address::new(
            self.group_value(0) as u8,
            self.group_value(1) as u8,
            self.group_value(2) as u8,
            self.group_value(3) as u8,
        )
    }
}

impl Input for IPv4Input {
    fn label(&self) -> &str {
        self.label
    }

    fn text(&self) -> &[u8] {
        &self.bytes
    }

    fn index(&self) -> usize {
        self.length.min(14)
    }

    fn backspace(&mut self) {
        if self.length == 0 {
            return;
        }

        self.length -= 1;
        if self.length % 4 == 3 {
            self.length -= 1;
        }

        self.bytes[self.length] = b'_';
    }

    fn char(&mut self, c: u8) {
        if !c.is_ascii_digit() || self.length == 15 {
            return;
        }

        self.bytes[self.length] = c;
        if self.current_group_value() > u8::MAX as u16 {
            self.bytes[self.length] = b'_';
            return;
        }

        self.length += 1;

        if self.length != 15 && self.length % 4 == 3 {
            self.length += 1;
        }
    }
}
