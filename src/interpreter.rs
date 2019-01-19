use command::Command;
use parser::Parser;
use libc::getchar;
use errors::ErrorKind;

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

    pub fn run(&mut self,code:String)->Result<(),ErrorKind>{
        let mut parser = Parser::new(code);
        let code = parser.parse()?;
        //println!("{:?}",code);
        self.core_run(&code,true)
    }

    fn eval_command(&mut self,cmd : &Command)->Result<(),ErrorKind>{
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

            Command::Assign(v) => 
                self.assign(*v),

            Command::MultAdd(vs) =>
                self.mult_add(vs),

            Command::SkipWhile(v) => 
                self.skip_while(*v),
        }
    }

    fn add_memory(&mut self,mem:i32)->Result<(),ErrorKind>{
        self.memory[self.pointer] += mem;
        self.memory[self.pointer] &= 255;
        Ok(())
    }

    fn add_pointer(&mut self,ptr:i32)->Result<(),ErrorKind>{
        self.pointer = self.calc_next_pointer(ptr)?;

        Ok(())
    }

    fn assign(&mut self,v:i32)->Result<(),ErrorKind>{
        self.memory[self.pointer] = v;

        Ok(())
    }

    fn mult_add(&mut self,vs: &Vec<(i32,i32)>)->Result<(),ErrorKind>{
        let times = self.memory[self.pointer];

        if times == 0{
            return Ok(());
        }

        for (move_ptr,val) in vs{
            let ptr = self.calc_next_pointer(*move_ptr)?;
            self.memory[ptr] += times * val;
            self.memory[ptr] &= 255;
        }
        self.memory[self.pointer] = 0;

        Ok(())
    }

    fn skip_while(&mut self,v:i32)->Result<(),ErrorKind>{
        while self.memory[self.pointer] != 0{
            self.pointer = self.calc_next_pointer(v)?;
        }
        Ok(())
    }

    fn output(&self)->Result<(),ErrorKind>{
        let val = self.memory[self.pointer]; 
        print!("{}",((val as u8) as char));
        Ok(())
    }

    fn input(&mut self)->Result<(),ErrorKind>{
        unsafe{
            self.memory[self.pointer] = getchar() as i32;
        }
        Ok(())
    }

    fn run_loop(&mut self,codes:&Vec<Command>) -> Result<(),ErrorKind>{
        if self.memory[self.pointer] == 0{
            Ok(())
        }else{
            self.core_run(codes,false) 
        }
    }

    fn core_run(&mut self,code:&Vec<Command>,is_flat:bool)->Result<(),ErrorKind>{
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

    fn calc_next_pointer(&mut self,ptr_change:i32)->Result<usize,ErrorKind>{
        let new_pointer = self.pointer as i32 + ptr_change;

        if new_pointer < 0 {
            return Err(ErrorKind::NegativePointer);
        }

        let new_pointer = new_pointer as usize;

        // realloc
        while new_pointer >= self.memory.len(){
            let new_size = self.memory.len() * 2;
            self.memory.resize(new_size,0);
        }

        Ok(new_pointer)
    }
}

