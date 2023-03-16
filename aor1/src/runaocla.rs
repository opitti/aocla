use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

mod aoclalib;
use aoclalib::AoType;
use aoclalib::interp;

fn main() {
   runapp();

}

pub fn runapp() {

    println!("AO start");
    let mut env: HashMap<String,AoType> = HashMap::new();
    let mut stack: Rc<RefCell<Vec<AoType>>> = Rc::new(RefCell::new(Vec::new()));

    //let mut i_lex4 = ao_var("12 13 'tkn \"str 1\"");
    //interp("11 22 33 (v1 w1 x1) 44 $v1 $w1 $x1 + + * dup",&mut env,Rc::clone(&stack));
    //println!("*********************test 1****************************");
    //println!("test 1 env{:?}",&env);
    //println!(">>>> test 1 stack FINALE : {:?} <<<<<<",&stack);
    //println!("*******************************************************");



    //let mut i_lex5 = l_ao_all("11 22 (x1 x2) $x1 $x2 [1 2 +] (add) ");
    //let mut i_lex5 = l_ao_all("[1 2 +] (add) 1 2 + $add");
    println!("*********************test 1****************************");
    interp("[(x) 10 $x *] (add) 1 2 + $add eval $add eval",&mut env,Rc::clone(&stack));
    println!("test 1 env{:?}",&env);
    println!(">>>> test 1 stack FINALE : {:?} <<<<<<",&stack);
    println!("*******************************************************");

    println!("*********************test 2****************************");
    interp("10 2 > 7 6 <",&mut env,Rc::clone(&stack));
    println!("test 2 env{:?}",&env);
    println!(">>>> test 2 stack FINALE : {:?} <<<<<<",&stack);
    println!("*******************************************************");

    println!("*********************test 2bis ****************************");
    println!("test 2bis env{:?}",&env);
    println!(">>>> test 2bis stack FINALE : {:?} <<<<<<",&stack);
    println!("*******************************************************");

    println!("*********************test 3****************************");
    interp("[2 0 >] ['Done] if [0 0 >] ['Done2] if",&mut env,Rc::clone(&stack));
    println!("test 3 env{:?}",&env);
    println!(">>>> test 3 stack FINALE : {:?} <<<<<<",&stack);
    println!("*******************************************************");

    
    println!("*********************test 4****************************");
    interp("10 (x5) $x5 1 - (x5) $x5 1 - (x5) $x5 1 - (x5) [(x6) $x6 1 -] (moinsun) $x5 $moinsun eval (x5) $x5 $moinsun eval (x5)",&mut env,Rc::clone(&stack));
    println!("test 4 env{:?}",&env);
    println!(">>>> test 4 stack FINALE : {:?} <<<<<<",&stack);
    println!("*******************************************************");


    // sans expace : analyse lexical :Ok(("", ([Int(10), Ass(["x5"]), Fct("$x5 0 >"), Fct("$x5 1 - (x5) 'wdone"), Cmd("while")], Some(Cmd("while")))))
    // avec espace : analyse lexical :Ok(("[$x5 0 >] [$x5 1 - (x5) 'wdone] while", ([Int(10), Ass(["x5"]), Spc], Some(Spc))))
    //interp("10 (x5) 1 $x5 -",&mut env,Rc::clone(&stack));
    println!("*********************test 5****************************");
    interp("10 (x5)    
                  [$x5 0 >] 
                  [$x5 1 - (x5) 
                  'wdone] 
                  while",&mut env,Rc::clone(&stack));
    println!("test 5 env{:?}",&env);
    println!(">>>> test 5 stack FINALE : {:?} <<<<<<",&stack);

    println!("*******************************************************");
}