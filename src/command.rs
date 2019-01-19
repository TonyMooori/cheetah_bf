#[derive(PartialEq,Debug,Clone)]
pub enum Command{
    AddMemory(i32),
    AddPointer(i32),
    Output,
    Input,
    Loop(Vec<Command>),
    Assign(i32),
    MultAdd(Vec<(i32,i32)>),
    SkipWhile(i32),
}

