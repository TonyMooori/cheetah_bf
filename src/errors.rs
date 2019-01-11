#[derive(PartialEq,Debug,Clone)]
pub enum ErrorKind{
    NegativePointer,
    ManyLeftBraces,
    ManyRightBraces,
    ProgramBug,
}

