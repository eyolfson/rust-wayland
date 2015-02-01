// Copyright 2015 Jonathan Eyolfson

use std::old_io::{IoError, IoResult};
use std::old_io::net::pipe::UnixStream;
use std::os;

pub type Opcode = u16;


fn u16x2_to_u32(n1: u16, n2: u16) -> u32 {
    ((n1 as u32) << 16) | (n2 as u32)
}

fn u32_to_u16x2(n: u32) -> (u16, u16) {
    (((n & 0b11111111_11111111_00000000_00000000) >> 16) as u16,
     (n & 0b00000000_00000000_11111111_11111111) as u16)
}

const DISPLAY_ID: u32 = 1;
const FIRST_ID: u32 = 2;
const MAX_ID: u32 = 0xfe_ff_ff_ff;

pub struct Display {
    stream: UnixStream,
    next_id: u32,
}

impl Display {
    pub fn connect() -> IoResult<Display> {
        let xdg_runtime_dir = match os::getenv("XDG_RUNTIME_DIR") {
            Some(d) => d,
            None => return Err(IoError::from_errno(20, false)),
        };
        let mut socket = Path::new(xdg_runtime_dir);
        socket.push("wayland-0");
        match UnixStream::connect(&socket) {
            Ok(s) => Ok(Display {stream: s, next_id: FIRST_ID}),
            Err(e) => Err(e),
        }
    }
    fn get_id(&mut self) -> u32 {
        if self.next_id > MAX_ID {
            panic!("Ran out of object IDs");
        }
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    pub fn sync(&mut self) -> IoResult<()> {
        const SYNC: Opcode = 0;
        try!(self.stream.write_le_u32(DISPLAY_ID));
        try!(self.stream.write_le_u32(u16x2_to_u32(12, SYNC)));

        let id = self.get_id();
        try!(self.stream.write_le_u32(id));

        Ok(())
    }
    pub fn get_registry(&mut self) -> IoResult<Registry> {
        const GET_REGISTRY: Opcode = 1;
        try!(self.stream.write_le_u32(DISPLAY_ID));
        try!(self.stream.write_le_u32(u16x2_to_u32(12, GET_REGISTRY)));

        let id = self.get_id();
        try!(self.stream.write_le_u32(id));

        Ok(Registry {id: id})
    }
    pub fn read_message(&mut self) -> IoResult<()> {
        let sender = try!(self.stream.read_le_u32());
        let (length, opcode) = u32_to_u16x2(try!(self.stream.read_le_u32()));
        println!("sender = {}, length = {}, opcode = {}",
                 sender, length, opcode);

        let name = try!(self.stream.read_le_u32());

        let len = try!(self.stream.read_le_u32());
        let vec = try!(self.stream.read_exact(len as usize -1));
        try!(self.stream.read_u8());
        if len % 4 != 0 {
            for _ in 0..4-(len % 4) {
                try!(self.stream.read_u8());
            }
        }

        let version = try!(self.stream.read_le_u32());
        println!("  name = {}, interface = {}, version = {}",
                 name, String::from_utf8(vec).unwrap(), version);

        Ok(())
    }
}

pub struct Registry {
    id: u32,
}

impl Registry {
    pub fn bind(&mut self, name: u32) {
        const BIND: Opcode = 0;
        println!("bind, id = {}, opcode = {}, name = {}", self.id, BIND, name);
    }
}
