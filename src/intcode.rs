use Opcode::*;
use Mode::*;
use std::fmt::{Debug, Formatter, Error, Display};
use std::io::Write;
use std::collections::VecDeque;
use std::hint::unreachable_unchecked;

//const DBG: bool = true;
const DBG: bool = false;

#[derive(Debug)]
pub struct Computer {
    pub mem: Vec<i64>,
    ptr: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    rel_base: i64,
    pub is_done: bool,
}

impl Computer {
    pub fn compute(&mut self) {
        loop {
            if DBG { print!("{}: {} ", self.ptr, self.mem[self.ptr]); }
            let orig_ptr = self.ptr;
            let opcode = Opcode::from(&self);
            if DBG { print!(" {} ", opcode); }

            if let Err(code) = Opcode::calculate(&opcode, self) {
                match code {
                    Halt => {
                        if DBG { println!() }
                        self.is_done = true;
                        return;
                    }
                    JumpNZero(_, _) | JumpZero(_, _) => {}
                    Input(_) => return,
//                    _ => unreachable!("{:?} should not error in calculation", opcode)
                    _ => unsafe { unreachable_unchecked() },
                }
            }

            if orig_ptr == self.ptr {
                self.ptr += opcode.nparams() + opcode.nwrites() + 1;
            }

            if DBG { println!(); }
        };
    }

    fn read(&self, mode: &Mode) -> i64 {
        *self.mem.get(mode.index()).unwrap_or(&0)
    }

    fn write(&mut self, mode: &Mode, val: i64) {
        let index = mode.index();
        if index >= self.mem.len() {
            self.mem.resize(index + 1, 0);
        }
        self.mem[index] = val
    }

    pub fn send(&mut self, val: i64) {
        self.input.push_back(val);
    }

    pub fn recv(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn send_all(&mut self, vals: impl Iterator<Item=i64>) {
        vals.for_each(|n| self.send(n));
    }

    pub fn recv_all(&mut self) -> impl Iterator<Item=i64> + '_ {
        self.output.drain(..)
    }

    pub fn init<I: IntoIterator<Item=i64>>(mem: &Vec<i64>, vals: I) -> Self {
        Computer {
            mem: mem.clone(),
            ptr: 0,
            input: vals.into_iter().collect(),
            output: Default::default(),
            rel_base: 0,
            is_done: false,
        }
    }

    pub fn parse_mem(mem: &str) -> Vec<i64> {
        mem.lines()
            .take(1)
            .flat_map(|line| line.split(","))
            .map(|n| n.trim().parse().unwrap())
            .collect()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Opcode {
    Add(Mode, Mode, Mode),
    Mult(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpNZero(Mode, Mode),
    JumpZero(Mode, Mode),
    Less(Mode, Mode, Mode),
    Equal(Mode, Mode, Mode),
    SetRelBase(Mode),
    Halt,
}

impl Opcode {
    fn calculate(&self, com: &mut Computer) -> Result<(), Opcode> {
        if DBG { std::io::stdout().flush().unwrap(); }
        match self {
            Add(a, b, w) => {
                let res = com.read(a) + com.read(b);
                if DBG { print!("{}+{}={} @{}", com.read(a), com.read(b), res, w.index()); }
                com.write(w, res);
                Ok(())
            }
            Mult(a, b, w) => {
                let res = com.read(a) * com.read(b);
                if DBG { print!("{}*{}={} @{}", com.read(a), com.read(b), res, w.index()); }
                com.write(w, res);
                Ok(())
            }
            Input(w) => {
                let res = match com.input.pop_front() {
                    Some(inp) => inp,
                    None => return Err(Input(Mode::dummy()))
                };
                if DBG { print!("in={} @{}", res, w.index()); }
                com.write(w, res);
                Ok(())
            }
            Output(a) => {
                let res = com.read(a);
                if DBG { print!("out={}", res); }
                com.output.push_back(res);
                if com.output.len() % 3 == 0 && !com.output.is_empty() {
                    if com.output[com.output.len() - 3] == -1 {
                        println!("{:?}", com.output);
                    }
                }
                Ok(())
            }
            JumpNZero(a, j) => {
                let a = com.read(a);
                if a != 0 {
                    if DBG { print!("jnz({}!=0)->{}", a, com.read(j)); }
                    com.ptr = com.read(j) as usize;
                    Ok(())
                } else {
                    if DBG { print!("!jnz({}==0)", a); }
                    Err(JumpNZero(Mode::dummy(), Mode::dummy()))
                }
            }
            JumpZero(a, j) => {
                let a = com.read(a);
                if a == 0 {
                    if DBG { print!("jnz({}==0)->{}", a, com.read(j)); }
                    com.ptr = com.read(j) as usize;
                    Ok(())
                } else {
                    if DBG { print!("!jnz({}!=0)", a); }
                    Err(JumpZero(Mode::dummy(), Mode::dummy()))
                }
            }
            Less(a, b, w) => {
                let res = if com.read(a) < com.read(b) { 1 } else { 0 };
                if DBG { print!("{}<{}={} @{}", com.read(a), com.read(b), res, w.index()); }
                com.write(w, res);
                Ok(())
            }
            Equal(a, b, w) => {
                let res = if com.read(a) == com.read(b) { 1 } else { 0 };
                if DBG { print!("{}=={}={} @{}", com.read(a), com.read(b), res, w.index()); }
                com.write(w, res);
                Ok(())
            }
            SetRelBase(a) => {
                let a = com.read(a);
                if DBG { print!("rb={}+{}={}", com.rel_base, a, com.rel_base + a); }
                com.rel_base += a;
                Ok(())
            }
            Halt => Err(Halt),
        }
    }

    fn nparams(&self) -> usize {
        match self {
            Add(_, _, _) => 2,
            Mult(_, _, _) => 2,
            Input(_) => 0,
            Output(_) => 1,
            JumpNZero(_, _) => 2,
            JumpZero(_, _) => 2,
            Less(_, _, _) => 2,
            Equal(_, _, _) => 2,
            SetRelBase(_) => 1,
            Halt => 0,
        }
    }

    fn nwrites(&self) -> usize {
        match self {
            Add(_, _, _) => 1,
            Mult(_, _, _) => 1,
            Input(_) => 1,
            Output(_) => 0,
            JumpNZero(_, _) => 0,
            JumpZero(_, _) => 0,
            Less(_, _, _) => 1,
            Equal(_, _, _) => 1,
            SetRelBase(_) => 0,
            Halt => 0,
        }
    }

    fn from(com: &Computer) -> Self {
        let ptr = com.ptr;
        let mem = &com.mem;
        let instr = com.mem[ptr];
        let code = instr % 100;
        match code {
            1 => {
                if DBG { print!("[{}, {}, {}]", mem[ptr + 1], mem[ptr + 2], mem[ptr + 3], ); }
                Add(Mode::from(com, 1), Mode::from(com, 2), Mode::from(com, 3))
            }
            2 => {
                if DBG { print!("[{}, {}, {}]", mem[ptr + 1], mem[ptr + 2], mem[ptr + 3], ); }
                Mult(Mode::from(com, 1), Mode::from(com, 2), Mode::from(com, 3))
            }
            3 => {
                if DBG { print!("[{}]", mem[ptr + 1]); }
                Input(Mode::from(com, 1))
            }
            4 => {
                if DBG { print!("[{}]", mem[ptr + 1]); }
                Output(Mode::from(com, 1))
            }
            5 => {
                if DBG { print!("[{}, {}]", mem[ptr + 1], mem[ptr + 2]); }
                JumpNZero(Mode::from(com, 1), Mode::from(com, 2))
            }
            6 => {
                if DBG { print!("[{}, {}]", mem[ptr + 1], mem[ptr + 2]); }
                JumpZero(Mode::from(com, 1), Mode::from(com, 2))
            }
            7 => {
                if DBG { print!("[{}, {}, {}]", mem[ptr + 1], mem[ptr + 2], mem[ptr + 3], ); }
                Less(Mode::from(com, 1), Mode::from(com, 2), Mode::from(com, 3))
            }
            8 => {
                if DBG { print!("[{}, {}, {}]", mem[ptr + 1], mem[ptr + 2], mem[ptr + 3], ); }
                Equal(Mode::from(com, 1), Mode::from(com, 2), Mode::from(com, 3))
            }
            9 => {
                if DBG { print!("[{}]", mem[ptr + 1]); }
                SetRelBase(Mode::from(com, 1))
            }
            99 => Halt,
            _ => unsafe { unreachable_unchecked() },
//            _ => unreachable!("Opcode::from instr={}", instr),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self {
            Add(_, _, _) => "Add",
            Mult(_, _, _) => "Mult",
            Input(_) => "Input",
            Output(_) => "Output",
            JumpNZero(_, _) => "JumpNZero",
            JumpZero(_, _) => "JumpZero",
            Less(_, _, _) => "Less",
            Equal(_, _, _) => "Equal",
            SetRelBase(_) => "SetRelBase",
            Halt => "Halt",
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Mode {
    Pos(usize),
    Imm(usize),
    Rel(usize),
}

impl Mode {
    fn from(computer: &Computer, offset: usize) -> Self {
        let Computer { mem, ptr, input: _, output: _, rel_base, is_done: _ } = computer;
        let ptr = *ptr;
        let rel_base = *rel_base;
        let instr = mem[ptr];
        let key = (instr / 10_i64.pow((offset + 1) as u32)) % 10;
        match key {
            0 => Pos(mem[ptr + offset] as usize),
            1 => Imm(ptr + offset),
            2 => Rel((mem[ptr + offset] + rel_base) as usize),
//            fail => unreachable!("Mode::from, key={}", fail),
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn index(&self) -> usize {
        match self {
            Pos(i) => *i,
            Imm(i) => *i,
            Rel(i) => *i,
        }
    }

    fn dummy() -> Self { // thicc
        Pos(0) // should be unreachable to actually get this value
    }
}