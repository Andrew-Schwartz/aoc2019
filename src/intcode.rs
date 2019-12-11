use Opcode::*;
use Mode::*;
use std::fmt::{Debug, Formatter, Error, Display};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::io::Write;

//const DBG: bool = true;
const DBG: bool = false;

#[derive(Debug)]
pub struct Computer {
    mem: Vec<i64>,
    ptr: usize,
    inputs: Option<Receiver<i64>>,
    outputs: Option<Sender<i64>>,
    rel_base: i64,
    pub is_done: bool,
}

pub type Txin = Sender<i64>;
pub type Rxout = Receiver<i64>;

impl Computer {
    pub fn compute(self) -> Self {
        let mut com = self;
        loop {
            if DBG { print!("{}: {} ", com.ptr, com.mem[com.ptr]); }
            let orig_ptr = com.ptr;
            let opcode = Opcode::from(&com);
            if DBG { print!(" {} ", opcode); }

            if let Err(code) = Opcode::calculate(&opcode, &mut com) {
                match code {
                    Halt => {
                        if DBG { println!() }
                        com.is_done = true;
                        return com;
                    }
                    JumpNZero(_, _) | JumpZero(_, _) => {}
                    Input(_) => return com,
                    _ => unreachable!("{:?} should not error in calculation", opcode)
                }
            }

            if orig_ptr == com.ptr {
                com.ptr += opcode.nparams() + opcode.nwrites() + 1;
            }

            if DBG { println!(); }
        }
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

    pub fn clone_rx_input(&self, input: Receiver<i64>) -> Self {
        let Computer { mem, ptr, inputs: _, outputs, rel_base, is_done } = self;
        let mem = mem.clone();
        let ptr = ptr.clone();
        let outputs = outputs.clone();
        let rel_base = rel_base.clone();
        let is_done = is_done.clone();
        Computer {
            mem,
            ptr,
            inputs: Some(input),
            outputs,
            rel_base,
            is_done,
        }
    }

    pub fn tx_output(&mut self, output: Sender<i64>) {
        self.outputs = Some(output);
    }

    pub fn new(mem: Vec<i64>) -> Self {
        Computer {
            mem,
            ptr: 0,
            inputs: None,
            outputs: None,
            rel_base: 0,
            is_done: false,
        }
    }

    pub fn init<I>(&self, inputs: I) -> (Computer, Txin, Rxout) where I: IntoIterator<Item=i64> {
        let (txin, rxin) = channel();
        let (txout, rxout) = channel();

        let mut com = self.clone_rx_input(rxin);
        com.tx_output(txout);
        inputs.into_iter().for_each(|i| txin.send(i).unwrap());
        (com, txin, rxout)
    }
}

impl From<&str> for Computer {
    fn from(input: &str) -> Self {
        let mem = input.lines()
            .take(1)
            .flat_map(|line| line.split(","))
            .map(|n| n.trim().parse().unwrap())
            .collect();
        Computer::new(mem)
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
                let input = com.inputs.as_ref().unwrap();

                let res = match input.try_recv() {
                    Ok(inp) => inp,
                    Err(_) => return Err(Input(Mode::dummy()))
                };
                if DBG { print!("in={} @{}", res, w.index()); }
                com.write(w, res);
                Ok(())
            }
            Output(a) => {
                let res = com.read(a);
                if DBG { print!("out={}", res); }
                let output = com.outputs.as_ref().unwrap();
                output.send(res).expect("output closed");
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

    fn from(computer: &Computer) -> Self {
        let Computer { mem, ptr, inputs: _, outputs: _, rel_base: _, is_done: _ } = computer;
        let ptr = *ptr;
        let instr = mem[ptr];
        let code = instr % 100;
        match code {
            1 => {
                if DBG { print!("[{}, {}, {}]", mem[ptr + 1], mem[ptr + 2], mem[ptr + 3], ); }
                Add(Mode::from(computer, 1), Mode::from(computer, 2), Mode::from(computer, 3))
            }
            2 => {
                if DBG { print!("[{}, {}, {}]", mem[ptr + 1], mem[ptr + 2], mem[ptr + 3], ); }
                Mult(Mode::from(computer, 1), Mode::from(computer, 2), Mode::from(computer, 3))
            }
            3 => {
                if DBG { print!("[{}]", mem[ptr + 1]); }
                Input(Mode::from(computer, 1))
            }
            4 => {
                if DBG { print!("[{}]", mem[ptr + 1]); }
                Output(Mode::from(computer, 1))
            }
            5 => {
                if DBG { print!("[{}, {}]", mem[ptr + 1], mem[ptr + 2]); }
                JumpNZero(Mode::from(computer, 1), Mode::from(computer, 2))
            }
            6 => {
                if DBG { print!("[{}, {}]", mem[ptr + 1], mem[ptr + 2]); }
                JumpZero(Mode::from(computer, 1), Mode::from(computer, 2))
            }
            7 => {
                if DBG { print!("[{}, {}, {}]", mem[ptr + 1], mem[ptr + 2], mem[ptr + 3], ); }
                Less(Mode::from(computer, 1), Mode::from(computer, 2), Mode::from(computer, 3))
            }
            8 => {
                if DBG { print!("[{}, {}, {}]", mem[ptr + 1], mem[ptr + 2], mem[ptr + 3], ); }
                Equal(Mode::from(computer, 1), Mode::from(computer, 2), Mode::from(computer, 3))
            }
            9 => {
                if DBG { print!("[{}]", mem[ptr + 1]); }
                SetRelBase(Mode::from(computer, 1))
            }
            99 => Halt,
            _ => unreachable!("Opcode::from instr={}", instr),
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
        let Computer { mem, ptr, inputs: _, outputs: _, rel_base, is_done: _ } = computer;
        let ptr = *ptr;
        let rel_base = *rel_base;
        let instr = mem[ptr];
        let key = (instr / 10_i64.pow((offset + 1) as u32)) % 10;
        match key {
            0 => Pos(mem[ptr + offset] as usize),
            1 => Imm(ptr + offset),
            2 => Rel((mem[ptr + offset] + rel_base) as usize),
            fail => unreachable!("Mode::from, key={}", fail),
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

#[macro_export]
macro_rules! send {
    ($sender:ident, [$($x:expr),*]) => {
        $(
            $sender.send($x).expect("unable to send");
        )*
    };
}