use command::Command;
use optimizer::*;
use errors::ErrorKind;

pub struct Parser{
    code : Vec<char>,
    index : usize,
}

fn is_command(c:char)->bool{
    match c{
        '+' | '-' | '>' | '<' | '[' | ']' | '.' | ',' => true,
        _ => false,
    }
}

impl Parser{
    pub fn new(code:String) -> Parser{
        Parser{
            code : code.chars().collect(),
            index : 0,
        }
    }

    fn read_add_memory(&mut self)->Result<Command,ErrorKind>{
        let mut mem = 0 as i32;

        while let Some(c) = self.code.get(self.index){
            match *c {
                '+' => mem += 1,
                '-' => mem -= 1,
                v if is_command(v) => break,
                _ => {}
            }
            self.index += 1;
        }

        Ok(Command::AddMemory(mem))
    }

    fn read_add_pointer(&mut self)->Result<Command,ErrorKind>{
        let mut ptr = 0 as i32;

        while let Some(c) = self.code.get(self.index){
            match *c {
                '>' => ptr += 1,
                '<' => ptr -= 1,
                v if is_command(v) => break,
                _ => {}
            }
            self.index += 1;
        }

        Ok(Command::AddPointer(ptr))
    }

    fn get_next_char(&mut self)->Option<char>{
        while let Some(c) = self.code.get(self.index){
            match *c {
                v if is_command(v) => 
                    return Some(v),
                _ =>
                    self.index += 1,
            }
        }

        None
    }

    fn read_sequence(&mut self)->Result<Vec<Command>,ErrorKind>{
        let mut ret = Vec::new();

        loop{
            let c = match self.get_next_char(){
                Some(v) => v,
                None => break,
            };

            let cmd = match c{
                '+' => 
                    self.read_add_memory(),

                '-' => 
                    self.read_add_memory(),

                '>' =>
                    self.read_add_pointer(),

                '<' =>
                    self.read_add_pointer(),

                '[' =>{
                    self.index += 1;
                    let vs = self.read_sequence()?;
                    if Some(']') != self.get_next_char(){
                        Err(ErrorKind::ManyLeftBraces)
                    }else{
                        self.index += 1;        
                        Ok(Command::Loop(vs))
                    }
                },

                ']' => 
                    break,

                ',' =>{
                    self.index += 1;
                    Ok(Command::Input)
                },

                '.' => {
                    self.index += 1;
                    Ok(Command::Output)
                },

                _ => 
                    Err(ErrorKind::ProgramBug),
            }?;

            ret.push(cmd);
        }

        Ok(ret)
    }

    pub fn parse(&mut self)->Result<Vec<Command>,ErrorKind>{
        let ret = self.read_sequence()?;

        if self.get_next_char().is_some(){
            Err(ErrorKind::ManyRightBraces)
        }else{
            Ok(optimize(ret))
        }
    }
}


#[test]
fn test000(){
    let mut parser = Parser::new(format!("++++++++[->++++++++>]>+."));
    let xs = parser.parse();
    match xs{
        Ok(v) => eprintln!("{:?}",v),
        Err(v) => eprintln!("{:?}",v),
    }
}

