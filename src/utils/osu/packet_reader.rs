use std::{convert::TryInto, io::BufRead};

pub struct PacketReader {
    buffer: Vec<u8>,
}

/*
   # integral types

    def read_i8(self) -> int:
        val = self.body_view[0]
        self.body_view = self.body_view[1:]
        return val - 256 if val > 127 else val

    def read_u8(self) -> int:
        val = self.body_view[0]
        self.body_view = self.body_view[1:]
        return val

    def read_i16(self) -> int:
        val = int.from_bytes(self.body_view[:2], "little", signed=True)
        self.body_view = self.body_view[2:]
        return val

    def read_u16(self) -> int:
        val = int.from_bytes(self.body_view[:2], "little", signed=False)
        self.body_view = self.body_view[2:]
        return val

    def read_i32(self) -> int:
        val = int.from_bytes(self.body_view[:4], "little", signed=True)
        self.body_view = self.body_view[4:]
        return val

    def read_u32(self) -> int:
        val = int.from_bytes(self.body_view[:4], "little", signed=False)
        self.body_view = self.body_view[4:]
        return val

    def read_i64(self) -> int:
        val = int.from_bytes(self.body_view[:8], "little", signed=True)
        self.body_view = self.body_view[8:]
        return val

    def read_u64(self) -> int:
        val = int.from_bytes(self.body_view[:8], "little", signed=False)
        self.body_view = self.body_view[8:]
        return val

    # floating-point types

    def read_f16(self) -> float:
        (val,) = struct.unpack_from("<e", self.body_view[:2])
        self.body_view = self.body_view[2:]
        return val

    def read_f32(self) -> float:
        (val,) = struct.unpack_from("<f", self.body_view[:4])
        self.body_view = self.body_view[4:]
        return val

    def read_f64(self) -> float:
        (val,) = struct.unpack_from("<d", self.body_view[:8])
        self.body_view = self.body_view[8:]
        return val

    # complex types

    # XXX: some osu! packets use i16 for
    # array length, while others use i32
    def read_i32_list_i16l(self) -> tuple[int]:
        length = int.from_bytes(self.body_view[:2], "little")
        self.body_view = self.body_view[2:]

        val = struct.unpack(f'<{"I" * length}', self.body_view[: length * 4])
        self.body_view = self.body_view[length * 4 :]
        return val

    def read_i32_list_i32l(self) -> tuple[int]:
        length = int.from_bytes(self.body_view[:4], "little")
        self.body_view = self.body_view[4:]

        val = struct.unpack(f'<{"I" * length}', self.body_view[: length * 4])
        self.body_view = self.body_view[length * 4 :]
        return val

    def read_string(self) -> str:
        exists = self.body_view[0] == 0x0B
        self.body_view = self.body_view[1:]

        if not exists:
            # no string sent.
            return ""

        # non-empty string, decode str length (uleb128)
        length = shift = 0

        while True:
            byte = self.body_view[0]
            self.body_view = self.body_view[1:]

            length |= (byte & 0x7F) << shift
            if (byte & 0x80) == 0:
                break

            shift += 7

        val = self.body_view[:length].tobytes().decode()  # copy
        self.body_view = self.body_view[length:]
        return val

    # custom osu! types

    def read_message(self) -> Message:
        """Read an osu! message from the internal buffer."""
        return Message(
            sender=self.read_string(),
            text=self.read_string(),
            recipient=self.read_string(),
            sender_id=self.read_i32(),
        )

    def read_channel(self) -> Channel:
        """Read an osu! channel from the internal buffer."""
        return Channel(
            name=self.read_string(),
            topic=self.read_string(),
            players=self.read_i32(),
        )

    def read_match(self) -> MultiplayerMatch:
        """Read an osu! match from the internal buffer."""
        m = MultiplayerMatch(
            id=self.read_i16(),
            in_progress=self.read_i8() == 1,
            powerplay=self.read_i8(),
            mods=self.read_i32(),
            name=self.read_string(),
            passwd=self.read_string(),
            map_name=self.read_string(),
            map_id=self.read_i32(),
            map_md5=self.read_string(),
            slot_statuses=[self.read_i8() for _ in range(16)],
            slot_teams=[self.read_i8() for _ in range(16)],
            # ^^ up to slot_ids, as it relies on slot_statuses ^^
        )

        for status in m.slot_statuses:
            if status & 124 != 0:  # slot has a player
                m.slot_ids.append(self.read_i32())

        m.host_id = self.read_i32()
        m.mode = self.read_i8()
        m.win_condition = self.read_i8()
        m.team_type = self.read_i8()
        m.freemods = self.read_i8() == 1

        if m.freemods:
            m.slot_mods = [self.read_i32() for _ in range(16)]

        m.seed = self.read_i32()  # used for mania random mod

        return m

    def read_scoreframe(self) -> ScoreFrame:
        sf = ScoreFrame(*SCOREFRAME_FMT.unpack_from(self.body_view[:29]))
        self.body_view = self.body_view[29:]

        if sf.score_v2:
            sf.combo_portion = self.read_f64()
            sf.bonus_portion = self.read_f64()

        return sf

    def read_replayframe(self) -> ReplayFrame:
        return ReplayFrame(
            button_state=self.read_u8(),
            taiko_byte=self.read_u8(),  # pre-taiko support (<=2008)
            x=self.read_f32(),
            y=self.read_f32(),
            time=self.read_i32(),
        )

    def read_replayframe_bundle(self) -> ReplayFrameBundle:
        # save raw format to distribute to the other clients
        raw_data = self.body_view[: self.current_len]

        extra = self.read_i32()  # bancho proto >= 18
        framecount = self.read_u16()
        frames = [self.read_replayframe() for _ in range(framecount)]
        action = ReplayAction(self.read_u8())
        scoreframe = self.read_scoreframe()
        sequence = self.read_u16()

        return ReplayFrameBundle(frames, scoreframe, action, extra, sequence, raw_data)

*/

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
