//use std::borrow::BorrowMut;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take,take_till,take_while},
    character::complete::{alpha1, digit1, alphanumeric1, one_of,multispace1},
    character::is_digit,
    combinator::{opt,map_parser},
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated, tuple,delimited,pair},
    AsChar, Err as NomErr, IResult, InputTakeAtPosition,
};

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum  AoType<'a> {
    Str(Box<&'a str>),
    Tkn(Box<&'a str>),
    Int(Box<i32>),
    Opr(Box<&'a str>),
    Ass(Box<Vec<&'a str>>),
    Var(Box<&'a str>),
    Cmd(Box<&'a str>),
    Fct(Box<&'a str>),
    Spc,
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;
fn typ_int(input: &str) -> Res<&str, AoType> {
    println!("integer input : {}",&input);
    //let vec:Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9'];
    //context("intger",  take_till(|c: char| c == ' ' || !vec!['0','1','2','3','4','5','6','7','8','9'].contains(&c) ))(input)
    context("intger",  take_till(|c: char| c == ' ' || !c.is_digit(10) ))(input)
        .map(|(next_input, res)| {
            println!("  typ_int next_input {:?}",next_input);
            println!("  typ_int res {:?}",&res);
            (next_input, AoType::Int(Box::new(res.parse::<i32>().unwrap())))
        })
}

fn typ_token(input: &str) -> Res<&str, AoType> {
    println!("integer input : {}",&input);
    //let vec:Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9'];
    //context("intger",  take_till(|c: char| c == ' ' || !vec!['0','1','2','3','4','5','6','7','8','9'].contains(&c) ))(input)
    context("token",  preceded(tag("'"),take_till(|c: char| c == ' ' || !c.is_alphanumeric()) ))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Tkn(Box::new(res)))
        })
}

fn typ_string(input: &str) -> Res<&str, AoType> {
    delimited(tag("\""), take_while(|x| x != '\"'), tag("\""))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Str(Box::new(res)))
        })
}


fn ao_function(input: &str) -> Res<&str, AoType> {
    delimited(tag("["), take_till(|x| x == ']'),tag("]"))(input)
        .map(|(next_input, res)| {
            println!("ao_function next_input : {:?}",next_input);
            println!("ao_function res : {:?}",res);
            (next_input, AoType::Fct(Box::new(res)))
        })
}

fn ao_var(input: &str) -> Res<&str, AoType> {
    println!("ao_var input : {}",&input);
    delimited(
        tag("("),
        tuple((many0(terminated( alphanumeric1, tag(" "))),
               opt(alphanumeric1), )),
        tag(")")
    )
    (input).map(|(next_input, mut res)| {
        println!("ao_var next_input {:?}",&next_input);
        println!("ao_var res {:?}",&res);
        let mut ires = Box::new(Vec::new());

        for v in res.0 {
            ires.push(v.clone())
        }

        match &res.1 {
            Some(p) => ires.push(p.clone()),
            None => {}
        };

        let ires_f = AoType::Ass(ires);
        (next_input,ires_f)
    })
}

fn ao_val(input: &str) -> Res<&str, AoType> {
    pair(tag("$"),alphanumeric1)(input).map(|(next_input, mut res)| {
        println!("ao_val : {:?}",res);
        (next_input, AoType::Var(Box::new(res.1)))
    })
}

fn ao_space(input: &str) -> Res<&str, AoType> {
    multispace1(input)
    .map(|(next_input, mut res)| {
        println!("ao_space : {:?}",res);
        (next_input, AoType::Spc)
    })
}

// 2ème étape
fn ao_all(input: &str) -> Res<&str, AoType> {
    alt((ao_operator,ao_command,ao_function,ao_var,ao_val,typ_string,typ_token,typ_int))(input)
}

// 1er étape
fn l_ao_all(input: &str) -> Res<&str, (Vec<AoType>, Option<AoType>)> {
    context(
        "l_ao_all",
        tuple((many0(terminated( ao_all, tag(" "))),
               opt(ao_all),
        )), 
    )(input).map(|(next_input, mut res)| {
        println!("  l_ao_all next_input {:?}",next_input);
        println!("  l_ao_all res {:?}",&res);
        match &res.1 {
            Some(p) => res.0.push(p.clone()),
            None => {}
        };
        (next_input,res) })
}

fn ao_operator(input: &str) -> Res<&str, AoType> {
    alt((tag("+"),tag("-"),tag("*"),tag("/"),tag(">"),tag("<")))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Opr(Box::new(res)))
        })
}

fn ao_command(input: &str) -> Res<&str, AoType> {
    alt((tag("dup"),tag("eval"),tag("if"),tag("while")))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Cmd(Box::new(res)))
        })        
}

// ==================== Arithmetique function ====================

fn add<'b>(op1:AoType<'b>,op2:AoType<'b>) -> AoType<'b> {

    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};

    AoType::Int(Box::new(*i_op1 + *i_op2))
}
fn sub<'b>(op1:AoType<'b>,op2:AoType<'b>) -> AoType<'b> {

    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};

    AoType::Int(Box::new(*i_op2 - *i_op1))
}
fn mul<'b>(op1:AoType<'b>,op2:AoType<'b>) -> AoType<'b> {

    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};

    AoType::Int(Box::new(*i_op1 * *i_op2))
}
fn div<'b>(op1:AoType<'b>,op2:AoType<'b>) -> AoType<'b> {

    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};

    AoType::Int(Box::new(*i_op2 / *i_op1))
}
fn sup<'b>(op1:AoType<'b>,op2:AoType<'b>) -> AoType<'b> {

    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};
    if i_op2 > i_op1 
        {AoType::Int(Box::new(1))}
    else 
        {AoType::Int(Box::new(0))}
}
fn inf<'b>(op1:AoType<'b>,op2:AoType<'b>) -> AoType<'b> {

    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};
    if i_op2 < i_op1 
        {AoType::Int(Box::new(1))}
    else 
        {AoType::Int(Box::new(0))}
}
//fn eval<'c>(lex: AoType<'c>,env:&mut Vec<AoType<'c>>, stack:&mut Vec<AoType<'c>>) -> AoType<'c> {
fn eval<'a>(lex: AoType<'a>,env:&mut HashMap<String,AoType<'a>>, st: Rc<RefCell<Vec<AoType<'a>>>>) -> AoType<'a> {

    //let mut vec_ref = Rc::clone(st).borrow_mut();
    //println!("start ==> {:?}",Rc::clone(&st).borrow_mut().pop());
    //println!("start ==> {:?}",Rc::clone(&st).borrow_mut().pop());

        match lex {
            AoType::Str(_)    => {st.borrow_mut().push(lex);AoType::Tkn(Box::new("void"))},
            AoType::Tkn(_)    => {st.borrow_mut().push(lex);AoType::Tkn(Box::new("void"))},
            AoType::Int(_)    => {st.borrow_mut().push(lex);AoType::Tkn(Box::new("void"))},
            
            AoType::Opr(val)  => {
                //AoType::Tkn(Box::new("void"));
                println!("Opr : {:?}",val);
                let mut v = st.borrow_mut();
                let op1 = v.pop().unwrap();
                let op2 = v.pop().unwrap();
                match *val {
                    "+" => v.push(add(op1,op2)),
                    "-" => v.push(sub(op1,op2)),
                    "*" => v.push(mul(op1,op2)),
                    "/" => v.push(div(op1,op2)),
                    ">" => v.push(sup(op1,op2)),
                    "<" => v.push(inf(op1,op2)),
                    _ => {}
                }
                
                AoType::Tkn(Box::new("void"))
            },
            AoType::Opr(val)  => {AoType::Tkn(Box::new("void"))}
            AoType::Ass(val)  => {
                for k in val.iter() {
                    println!("Assignement : {:?}",&k);
                    env.insert(k.to_string(), st.borrow_mut().pop().unwrap());   
                }
                AoType::Tkn(Box::new("void"))
            },
            AoType::Var(k) => {
                println!("var : {:?}",&k);
                match env.get(&k.to_string()) {
                    Some(v) => {st.borrow_mut().push(v.clone())}
                    None => {}
                }
                AoType::Tkn(Box::new("void"))
            },
            AoType::Cmd(c) => {
                println!("CMD : {:?}",c);
                if c.eq( &Box::new("eval") ) {
                    let v = st.borrow_mut().pop().unwrap();
                    match v {
                        AoType::Fct(f) =>  interp(&f, env, Rc::clone(&st)),
                        _ => {}
                    }
                }
                else if c.eq(&Box::new("if")) {
                    // do the if 
                    let then = st.borrow_mut().pop().unwrap();
                    let test = st.borrow_mut().pop().unwrap();
                    match test {
                        AoType::Fct(f) =>  interp(&f, env, Rc::clone(&st)),
                        _ => {}
                    }
                    let res = st.borrow_mut().pop().unwrap();
                    match res {
                        AoType::Int(i) => {
                            if i == Box::new(1) {
                                match then {
                                    AoType::Fct(f) =>  interp(&f, env, Rc::clone(&st)),
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                else if c.eq(&Box::new("while")) {
                    // do the if 
                    let corps = st.borrow_mut().pop().unwrap();
                    let test = st.borrow_mut().pop().unwrap();
                    loop {
                        match &test {
                            AoType::Fct(f) =>  interp(&f, env, Rc::clone(&st)),
                            _ => {}
                        }
                        let res = st.borrow_mut().pop().unwrap();
                        match res {
                            AoType::Int(i) => {
                                if i == Box::new(1) {
                                    match &corps {
                                        AoType::Fct(f) =>  interp(&f, env, Rc::clone(&st)),
                                        _ => {}
                                    }
                                } else {break}
                            }
                            _ => {}
                        }
                    }
                }
                AoType::Tkn(Box::new("void"))
            }
            AoType::Fct(_) => {
                st.borrow_mut().push(lex);
                AoType::Tkn(Box::new("void"))
            }
            AoType::Spc => {
                AoType::Tkn(Box::new("void"))
            }
        }
}

fn interp<'a>(code:&'a str, env:&mut HashMap<String,AoType<'a>>, st: Rc<RefCell<Vec<AoType<'a>>>>) {
    let lex = l_ao_all(code);
    if let Err(ref lex2) = lex{
        println!("Syntax error");
    }
    else {
        println!("analyse lexical :{:?}",&lex);
        for v in lex.unwrap().1.0{
            eval(v,env,Rc::clone(&st));
        }
    }
}


fn main() {
    println!("AO start");
    let mut env: HashMap<String,AoType> = HashMap::new();
    let mut stack: Rc<RefCell<Vec<AoType>>> = Rc::new(RefCell::new(Vec::new()));

    //let mut i_lex4 = ao_var("12 13 'tkn \"str 1\"");
    interp("11 22 33 (v1 w1 x1) 44 $v1 $w1 $x1 + + * dup",&mut env,Rc::clone(&stack));
    println!("*********************test 1****************************");

    println!("test 1 env{:?}",&env);
    println!(">>>> test 1 stack FINALE : {:?} <<<<<<",&stack);

    println!("*******************************************************");

    //let mut i_lex5 = l_ao_all("11 22 (x1 x2) $x1 $x2 [1 2 +] (add) ");
    //let mut i_lex5 = l_ao_all("[1 2 +] (add) 1 2 + $add");
    interp("[(x) 10 $x *] (add) 1 2 + $add eval $add eval",&mut env,Rc::clone(&stack));
    println!("*********************test 2****************************");

    println!("test 2 env{:?}",&env);
    println!(">>>> test 2 stack FINALE : {:?} <<<<<<",&stack);

    println!("*******************************************************");

    interp("10 2 > 7 6 <",&mut env,Rc::clone(&stack));
    println!("*********************test 2****************************");

    println!("test 2 env{:?}",&env);
    println!(">>>> test 2 stack FINALE : {:?} <<<<<<",&stack);

    println!("*******************************************************");

    interp("[2 0 >] ['Done] if [0 0 >] ['Done2] if",&mut env,Rc::clone(&stack));
    println!("*********************test 3****************************");

    println!("test 3 env{:?}",&env);
    println!(">>>> test 3 stack FINALE : {:?} <<<<<<",&stack);

    println!("*******************************************************");

    interp("10 (x5) $x5 1 - (x5) $x5 1 - (x5) $x5 1 - (x5) [(x6) $x6 1 -] (moinsun) $x5 $moinsun eval (x5) $x5 $moinsun eval (x5)",&mut env,Rc::clone(&stack));
    //interp("10 (x5) 1 $x5 -",&mut env,Rc::clone(&stack));
    println!("*********************test 4****************************");

    println!("test 4 env{:?}",&env);
    println!(">>>> test 4 stack FINALE : {:?} <<<<<<",&stack);

    println!("*******************************************************");

    interp("10 (x5) [$x5 0 >] [$x5 1 - (x5) 'wdone] while",&mut env,Rc::clone(&stack));
    // sans expace : analyse lexical :Ok(("", ([Int(10), Ass(["x5"]), Fct("$x5 0 >"), Fct("$x5 1 - (x5) 'wdone"), Cmd("while")], Some(Cmd("while")))))
    // avec espace : analyse lexical :Ok(("[$x5 0 >] [$x5 1 - (x5) 'wdone] while", ([Int(10), Ass(["x5"]), Spc], Some(Spc))))
    //interp("10 (x5) 1 $x5 -",&mut env,Rc::clone(&stack));
    println!("*********************test 5****************************");

    println!("test 5 env{:?}",&env);
    println!(">>>> test 5 stack FINALE : {:?} <<<<<<",&stack);

    println!("*******************************************************");

}

/*
https://blog.logrocket.com/parsing-in-rust-with-nom/

https://docs.rs/nom/latest/nom/sequence/fn.tuple.html
https://docs.rs/nom/latest/nom/multi/fn.count.html
https://docs.rs/nom/6.2.1/nom/macro.alt.html
https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

To see for error handling :
https://iximiuz.com/en/posts/rust-writing-parsers-with-nom/



Other :
https://github.com/zupzup/rust-nom-parsing/blob/main/src/lib.rs


objective:
https://github.com/antirez/aocla

[(l f) // list and function to call with each element.
    $l len (e)  // Get list len in "e"
    0 (j)       // j is our current index
    [$j $e <] [
        $l $j get@  // Get list[j]
        $f upeval   // We want to evaluate in the context of the caller
        $j 1 + (j)  // Go to the next index
    ] while
] 'foreach def

||

[1 2 3] [printnl] foreach

||

[$a 2 >] ["a is > 2" printnl] if

||

9 (a)

||

[$a 11 ==] ["11 reached" printnl] [$a 1 + (a)] ifelse

||

10 [dup 0 >] [dup printnl 1 -] while

||

10 (x) [$x 0 >] [$x printnl $x 1 - (x)] while

||

1 2 3

||

(a _ b) $_ $a $b +

||

[(n l)
    [$n 0 >]
    [$l eval $n 1 - (n)]
    while
] 'repeat def

||

3 ["Hello!" printnl] repeat

*/
