use command::Command;
use optimizer::*;

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

    fn core_parse(&mut self)->Result<Command,String>{
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
                    Err(format!("too many ]")),

                ',' =>{
                    self.index += 1;
                    Ok(Command::Input)
                },

                '.' => {
                    self.index += 1;
                    Ok(Command::Output)
                },

                _ => 
                    Err(format!("something wrong")),
        }
    }

    fn read_add_memory(&mut self)->Result<Command,String>{
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

    fn read_add_pointer(&mut self)->Result<Command,String>{
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

    fn read_loop(&mut self)->Result<Command,String>{
        let mut ret : Vec<Command>= Vec::new();

        self.index += 1;

        loop{
            self.skip_meanless_char();

            let c : char = match self.code.get(self.index){
                Some(v) => *v,
                None => return Err(format!("too many [")),
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

    pub fn parse(&mut self)->Result<Vec<Command>,String>{
        let mut ret : Vec<Command>= Vec::new();

        loop{
            self.skip_meanless_char();

            let c : char = match self.code.get(self.index){
                Some(v) => *v,
                None => break,
            };

            if c == ']'{
                return Err(format!("too many ]"));
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
        Err(v) => eprintln!("{}",v),
    }
}

