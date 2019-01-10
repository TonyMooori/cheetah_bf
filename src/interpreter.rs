use command::Command;
use parser::Parser;
use libc::getchar;

pub struct Interpreter{
    memory : Vec<i32>,
    pointer : usize,
}

impl Interpreter{
    pub fn new()->Interpreter{
        Interpreter{
            memory : vec![0;16],
            pointer : 0,
        }
    }

    pub fn run(&mut self,code:String)->Result<(),String>{
        let mut parser = Parser::new(code);
        let code = parser.parse()?;
        //println!("{:?}",code);
        self.core_run(&code,true)
    }

    fn eval_command(&mut self,cmd : &Command)->Result<(),String>{
        match cmd{
            Command::AddMemory(mem) =>
                self.add_memory(*mem),

            Command::AddPointer(ptr) => 
                self.add_pointer(*ptr),

            Command::Output =>
                self.output(), 

            Command::Input =>
                self.input(), 

            Command::Loop(codes) =>
                self.run_loop(codes),

            Command::None =>
                Ok(()), 

            Command::Assign(v) => 
                self.assign(*v),

            Command::MultAdd(vs) =>
                self.mult_add(vs),

            Command::SkipWhile(v) => 
                self.skip_while(*v),
        }
    }

    fn add_memory(&mut self,mem:i32)->Result<(),String>{
        self.memory[self.pointer] += mem;
        self.memory[self.pointer] &= 255;
        Ok(())
    }

    fn add_pointer(&mut self,ptr:i32)->Result<(),String>{
        let temp = self.pointer as i32 + ptr;
        self.pointer = self.check_pointer(temp)?;

        Ok(())
    }

    fn assign(&mut self,v:i32)->Result<(),String>{
        self.memory[self.pointer] = v;
        Ok(())
    }

    fn mult_add(&mut self,vs: &Vec<(i32,i32)>)->Result<(),String>{
        let times = self.memory[self.pointer];

        if times == 0{
            return Ok(());
        }

        for (move_ptr,val) in vs{
            let temp = self.pointer as i32 + move_ptr;
            let ptr = self.check_pointer(temp)?;
            self.memory[ptr] += times * val;
            self.memory[ptr] &= 255;
        }
        self.memory[self.pointer] = 0;

        Ok(())
    }

    fn skip_while(&mut self,v:i32)->Result<(),String>{
        while self.memory[self.pointer] != 0{
            let temp = self.pointer as i32 + v;
            self.pointer = self.check_pointer(temp)?;
        }
        Ok(())
    }

    fn output(&self)->Result<(),String>{
        let val = self.memory[self.pointer]; 
        print!("{}",((val as u8) as char));
        Ok(())
    }

    fn input(&mut self)->Result<(),String>{
        unsafe{
            self.memory[self.pointer] = getchar() as i32;
        }
        Ok(())
    }

    fn run_loop(&mut self,codes:&Vec<Command>) -> Result<(),String>{
        if self.memory[self.pointer] == 0{
            Ok(())
        }else{
            self.core_run(codes,false) 
        }
    }

    fn core_run(&mut self,code:&Vec<Command>,is_flat:bool)->Result<(),String>{
        loop{
            for cmd in code.into_iter(){
                // println!("{:?}",self.memory);
                self.eval_command(cmd)?;
            }

            if is_flat || self.memory[self.pointer] == 0{
                break; 
            }
        }
        Ok(())
    }

    fn check_pointer(&mut self,new_pointer:i32)->Result<usize,String>{
        if new_pointer < 0{
            return Err(format!("pointer is negative"));
        }
        let new_pointer = new_pointer as usize;

        // realloc
        while new_pointer >= self.memory.len(){
            let new_size = self.memory.len() * 2;
            self.memory.resize(new_size,0);
        }

        Ok(new_pointer as usize)
    }
}

