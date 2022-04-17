use std::{convert::TryInto, io::BufRead};
use std::io::{self, Write};
use cached::{proc_macro::io_cached, AsyncRedisCache};

pub struct PacketReader {
    buffer: Vec<u8>,
}

pub struct Message {
    sender: String,
    text: String,
    recipient: String,
    sender_id: i32,
}

pub struct Channel {
    name: String,
    topic: String,
    players: i32,
}

impl PacketReader {
    pub fn new(buffer: Vec<u8>) -> Self {
        PacketReader { buffer }
    }

    pub fn read_i8(&mut self) -> i8 {
        let val = self.buffer[0];
        self.buffer = self.buffer[1..].to_vec();
        i8::from_le_bytes([val])
    }

    pub fn read_u8(&mut self) -> u8 {
        let val = self.buffer[0];
        self.buffer = self.buffer[1..].to_vec();
        u8::from_le_bytes([val])
    }

    pub fn read_i16(&mut self) -> i16 {
        let val = self.buffer[0..2].to_vec();
        self.buffer = self.buffer[2..].to_vec();
        i16::from_le_bytes(val.as_slice().try_into().unwrap())
    }

    pub fn read_u16(&mut self) -> u16 {
        let val = self.buffer[0..2].to_vec();
        self.buffer = self.buffer[2..].to_vec();
        u16::from_le_bytes(val.as_slice().try_into().unwrap())
    }

    pub fn read_i32(&mut self) -> i32 {
        let val = self.buffer[0..4].to_vec();
        self.buffer = self.buffer[4..].to_vec();
        i32::from_le_bytes(val.as_slice().try_into().unwrap())
    }

    pub fn read_u32(&mut self) -> u32 {
        let val = self.buffer[0..4].to_vec();
        self.buffer = self.buffer[4..].to_vec();
        u32::from_le_bytes(val.as_slice().try_into().unwrap())
    }

    pub fn read_i64(&mut self) -> i64 {
        let val = self.buffer[0..8].to_vec();
        self.buffer = self.buffer[8..].to_vec();
        i64::from_le_bytes(val.as_slice().try_into().unwrap())
    }

    pub fn read_u64(&mut self) -> u64 {
        let val = self.buffer[0..8].to_vec();
        self.buffer = self.buffer[8..].to_vec();
        u64::from_le_bytes(val.as_slice().try_into().unwrap())
    }

    pub fn read_f16(&mut self) -> half::f16 {
        let val = self.buffer[0..2].to_vec();
        self.buffer = self.buffer[2..].to_vec();
        half::f16::from_le_bytes(val.as_slice().try_into().unwrap())
    }

    pub fn read_f32(&mut self) -> f32 {
        let val = self.buffer[0..4].to_vec();
        self.buffer = self.buffer[4..].to_vec();
        f32::from_le_bytes(val.as_slice().try_into().unwrap())
    }

    pub fn read_f64(&mut self) -> f64 {
        let val = self.buffer[0..8].to_vec();
        self.buffer = self.buffer[8..].to_vec();
        f64::from_le_bytes(val.as_slice().try_into().unwrap())
    }

    // lists
    pub fn read_i32_list_i16l(&mut self) -> Vec<i32> {
        let mut list = Vec::new();
        let mut len = self.read_i16();
        while len > 0 {
            list.push(self.read_i32());
            len -= 1;
        }
        list
    }

    pub fn read_i32_list_i32l(&mut self) -> Vec<i32> {
        let mut list = Vec::new();
        let mut len = self.read_i32();
        while len > 0 {
            list.push(self.read_i32());
            len -= 1;
        }
        list
    }

    // strings
    pub fn read_string(&mut self) -> String {
        if !self.buffer[0] == 0x0B {
            self.buffer = self.buffer[1..].to_vec();
            return "".to_string();
        }
        self.buffer = self.buffer[1..].to_vec();
        let mut shift = 0;
        let mut len = 0;
        loop {
            let b = self.buffer[0];
            self.buffer = self.buffer[1..].to_vec();
            len |= (b & 0x7F) << shift;
            if (b & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        let val = self.buffer[0..len as usize].to_vec();
        self.buffer = self.buffer[len as usize..].to_vec();
        String::from_utf8(val).unwrap()
    }

    // custom
    pub fn read_message(&mut self) -> Message {
        let sender = self.read_string();
        let text = self.read_string();
        let recipient = self.read_string();
        let sender_id = self.read_i32();
        Message {
            sender,
            text,
            recipient,
            sender_id,
        }
    }

    pub fn read_channel(&mut self) -> Channel {
        let name = self.read_string();
        let topic = self.read_string();
        let players = self.read_i32();
        Channel {
            name,
            topic,
            players,
        }
    }
}
