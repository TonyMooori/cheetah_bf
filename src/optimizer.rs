use command::Command;
use std::collections::HashMap;

pub fn optimize(ast:Vec<Command>)->Vec<Command>{
    ast
        .into_iter()
        .map(|x| optimize_command(x))
        .collect()
}

fn optimize_command(cmd: Command)->Command{
    match cmd{
        Command::Loop(v) =>{
            let temp = optimize_loop(v);

            temp
        },
        val => val,
    }
}

/*
   fn optimize_sequence(ast:Vec<Command>)->Vec<Command>{
   let mut ret = Vec::new();

   for cmd in ast{
   if ret.len() == 0{
   ret.push(cmd);
   continue;
   }else if let Command::AddMemory(v) = cmd{
   let last_cmd = ret.pop().unwrap();

   if let Command::Assign(w) = last_cmd{
   ret.push(Command::Assign(w+v));
   }else{
   ret.push(last_cmd);
   ret.push(Command::AddMemory(v));
   }
   }else{
   ret.push(cmd);
   }
   }

   ret
   }
   */

fn optimize_loop(ast:Vec<Command>)->Command{
    let ast : Vec<Command> = optimize(ast);

    if let Some(ret) = optimize_assign_zero(&ast){
        ret
    }else if let Some(ret) = optimize_skip_while(&ast){
        ret
    }else if let Some(ret) = optimize_mult_add(&ast){
        ret
    }else{
        Command::Loop(ast)
    }
}

fn optimize_skip_while(ast:&Vec<Command>)->Option<Command>{
    // [<<<<<<<<] を関数呼び出ししなくて良いようにする

    if ast.len() != 1{
        return None
    }

    match ast[0]{
        Command::AddPointer(v) =>
            Some(Command::SkipWhile(v)),

        _ => 
            None,
    }
}


fn optimize_assign_zero(ast:&Vec<Command>)->Option<Command>{
    if ast.len() != 1{
        return None;
    }

    match ast[0]{
        Command::AddMemory(-1) => 
            Some(Command::Assign(0)),

        Command::AddMemory(1) => 
            Some(Command::Assign(0)),

        _ => 
            None,
    }
}

fn optimize_mult_add(ast:&Vec<Command>)->Option<Command>{
    let (last_pointer,mut hm) = check_pointer_movement(ast)?;

    if last_pointer != 0{
        None
    }else if *hm.get(&0).unwrap_or(&0) == -1{
        hm.remove(&0);

        let mut vs = Vec::new();
        for (ptr,val) in &hm{
            vs.push((*ptr,*val));
        }

        Some(Command::MultAdd(vs))
    }else{
        None
    }
}

fn check_pointer_movement(ast:&Vec<Command>) -> Option<(i32,HashMap<i32,i32>)>{
    let mut pointer = 0;
    let mut hm = HashMap::new();

    for cmd in ast{
        match cmd{
            Command::AddMemory(v) =>{
                let v0 = *hm.get(&pointer).unwrap_or(&0);
                hm.insert(pointer,v0+v);
            },

            Command::AddPointer(v) => 
                pointer += v,

            _ =>
                return None,
        }
    }

    Some((pointer,hm))
}

