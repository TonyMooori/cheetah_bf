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

    fn core_parse(&mut self)->Result<Command,ErrorKind>{
        self.skip_meanless_char();

        let c : char= match self.code.get(self.index){
            Some(v) => *v,
            None => return Ok(Command::None),
        };

        match c {
            '+' | '-' =>
                self.read_add_memory(),

                '>' | '<' =>
                    self.read_add_pointer(),

                '[' =>
                    self.read_loop(),

                ']' =>
                    Err(ErrorKind::ManyRightBraces),

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

    fn read_loop(&mut self)->Result<Command,ErrorKind>{
        let mut ret : Vec<Command>= Vec::new();

        self.index += 1;

        loop{
            self.skip_meanless_char();

            let c : char = match self.code.get(self.index){
                Some(v) => *v,
                None => return Err(ErrorKind::ManyLeftBraces),
            };

            if c == ']'{
                self.index += 1;
                break;
            }else{
                ret.push(self.core_parse()?);
            }
        }

        Ok(Command::Loop(ret))
    }

    pub fn parse(&mut self)->Result<Vec<Command>,ErrorKind>{
        let mut ret : Vec<Command>= Vec::new();

        loop{
            self.skip_meanless_char();

            let c : char = match self.code.get(self.index){
                Some(v) => *v,
                None => break,
            };

            if c == ']'{
                return Err(ErrorKind::ManyRightBraces);
            }else{
                ret.push(self.core_parse()?);
            }
        }

        Ok(optimize(ret))
    }

    pub fn skip_meanless_char(&mut self){
        while let Some(c) = self.code.get(self.index){
            match *c {
                v if is_command(v) => break,
                _ =>
                    self.index += 1,
            }
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

