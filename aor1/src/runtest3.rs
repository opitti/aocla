use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod aoclalib;
use aoclalib::interp;
use aoclalib::AoType;

fn main() {
    runapp();
}

pub fn runapp() {
    println!("AO start");
    let mut env: HashMap<String, AoType> = HashMap::new();
    let mut stack: Rc<RefCell<Vec<AoType>>> = Rc::new(RefCell::new(Vec::new()));

    println!("*********************test 6****************************");
    interp(
        "10 (x5)    
                  [$x5 5 >] 
                  'tsup def 4 (x5) $x5 2 - tsup",
        &mut env,
        Rc::clone(&stack),
    );
    println!("test 6 env{:?}", &env);
    println!("\n>>>> STACK test 6 stack FINALE : {:?} <<<<<<\n", &stack);
    println!("\n>>>> ENV test 6 env \n{:?}\n", &env);
    println!("*******************************************************");
}
