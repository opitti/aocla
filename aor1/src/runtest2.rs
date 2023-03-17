use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod aoclalib;
use aoclalib::interp;
use aoclalib::interp_ao_type;
use aoclalib::l_ao_all;
use aoclalib::AoType;

fn main() {
    runapp();
}

pub fn runapp() {
    println!("AO start");
    let mut env: HashMap<String, AoType> = HashMap::new();
    let mut stack: Rc<RefCell<Vec<AoType>>> = Rc::new(RefCell::new(Vec::new()));

    //let res = l_ao_all("[1 2 + [10 11] -]");
    let res = l_ao_all("1 2 + [3 3 +] (x) $x 4 +");
    println!("{:?}", &res);
    let c = &res.unwrap().1 .0.pop().unwrap();

    match c {
        AoType::Lst(f) => {
            println!("run : {:?}", &f);
            interp_AoType(&f, &mut env, Rc::clone(&stack));
            println!("Stack : {:?}", &stack);
        }
        _ => {
            interp("1 2 + [3 3 +] (x) $x eval 4 +", &mut env, Rc::clone(&stack));
            println!("Stack : {:?}", &stack);

            /*
            interp("10 (x5)
                     [$x5 0 >]
                     [$x5 1 - (x5)
                     'wdone]
                     while",&mut env,Rc::clone(&stack));
            println!("Stack : {:?}",&stack);
            */
        }
    };

    //println!("Stack : {:?}",&stack);

    //interp_AoType(&c,&mut env,Rc::clone(&stack));
    //let mut i_lex4 = ao_var("12 13 'tkn \"str 1\"");
    //interp("11 22 33 (v1 w1 x1) 44 $v1 $w1 $x1 + + * dup",&mut env,Rc::clone(&stack));
}
