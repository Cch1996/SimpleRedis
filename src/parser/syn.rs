#[derive(Debug)]
pub enum NumVal<'input> {
    Int(i32),
    // There is no +/- in v
    Float { neg: bool, v: &'input str },
}

impl NumVal<'_> {
    pub fn neg(self) -> Self {
        match self {
            NumVal::Int(v) => NumVal::Int(-v),
            NumVal::Float { neg, v } => NumVal::Float { neg: !neg, v },
        }
    }
}

#[derive(Debug)]
pub enum StrVal<'input> {
    InPlace(&'input str),
    Dyn(String),
}

#[derive(Debug)]
pub enum Value<'input> {
    Num(NumVal<'input>),
    Str(StrVal<'input>),
}

#[derive(Debug)]
pub struct SetCommand<'input> {
    pub key: StrVal<'input>,
    pub value: Value<'input>
}

#[derive(Debug)]
pub struct GetCommand<'input> {
    pub key: StrVal<'input>
}

#[derive(Debug)]
pub enum Command<'input> {
    SetCommand(SetCommand<'input>),
    GetCommand(GetCommand<'input>),
    Empty,
}

