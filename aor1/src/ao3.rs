//use std::borrow::BorrowMut;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take,take_till,take_while},
    character::complete::{alpha1, digit1, alphanumeric1, one_of},
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

// 2ème étape
fn ao_all(input: &str) -> Res<&str, AoType> {
    alt((ao_operator,ao_var,ao_val,typ_string,typ_token,typ_int,))(input)
}

// 1er étape
fn l_ao_all(input: &str) -> Res<&str, (Vec<AoType>, Option<AoType>)> {
    context(
        "l_ao_all",
        tuple((many0(terminated( ao_all, tag(" "))),
               opt(ao_all),
        )), 
    )(input).map(|(next_input, mut res)| {
        println!("  lInteger next_input {:?}",next_input);
        println!("  lInteger res {:?}",&res);
        match &res.1 {
            Some(p) => res.0.push(p.clone()),
            None => {}
        };
        (next_input,res) })
}

fn ao_operator(input: &str) -> Res<&str, AoType> {
    alt((tag("+"),tag("-"),tag("*"),tag("/")))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Opr(Box::new(res)))
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

    AoType::Int(Box::new(*i_op1 - *i_op2))
}
fn mul<'b>(op1:AoType<'b>,op2:AoType<'b>) -> AoType<'b> {

    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};

    AoType::Int(Box::new(*i_op1 * *i_op2))
}
fn div<'b>(op1:AoType<'b>,op2:AoType<'b>) -> AoType<'b> {

    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};

    AoType::Int(Box::new(*i_op1 / *i_op2))
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
                AoType::Tkn(Box::new("void"))},
            //AoType::Tkn(Box::new("void")) => {AoType::Tkn(Box::new("void"))}
            }
}


fn test<'a>(s: Rc<RefCell<Vec<AoType<'a>>>>) {
    s.borrow_mut().push(AoType::Str(Box::new("test")));
}


fn main() {
    println!("AO start");

    //let mut i_lex4 = ao_var("12 13 'tkn \"str 1\"");
    let mut i_lex4 = l_ao_all("11 22 33 (v1 w1 x1) 44 $v1 $w1 $x1 + + *");
    println!("*************************************************");
    if let Err(ref lex4) = i_lex4{
        println!("lex4 KO");
    } else {
        println!("lex4 OK : {:?}",&i_lex4.clone().unwrap().1.0);
    }
    
    let mut env: HashMap<String,AoType> = HashMap::new();
    let mut stack: Rc<RefCell<Vec<AoType>>> = Rc::new(RefCell::new(Vec::new()));

    for v in i_lex4.unwrap().1.0{
        println!("lex 4 eval : {:?}",eval(v,&mut env,Rc::clone(&stack)));
        println!("lex4 stack : {:?}",&stack);
        println!("lex4 env{:?}",&env);
    }
    println!("lex4 env{:?}",&env);
    println!(">>>> lex4 stack FINALE : {:?} <<<<<<",&stack);
    

    println!("*************************************************");
    
    let mut lex3 = ao_var("(112 113)");
    println!("*************************************************");
    println!("lex3 : {:?}",lex3);
    println!("*************************************************");

    let mut lex2 = l_ao_all("12 13 'tkn \"str 1\" 14 'var1 +");
    println!("*************************************************");
    println!("lex2 : {:?}",lex2);
    println!("*************************************************");
    
    let mut lex = l_ao_all("12 13 + 14 +");
    let mut env: HashMap<String,AoType> = HashMap::new();
    //let mut stack: Vec<AoType> = Vec::new();
    let mut stack: Rc<RefCell<Vec<AoType>>> = Rc::new(RefCell::new(Vec::new()));

    for v in lex.unwrap().1.0{
       // println!("{:?} {:?}",&stack.clone(),eval(v,&mut env,&mut stack));
        println!("{:?}",eval(v,&mut env,Rc::clone(&stack)));
        //test(Rc::clone(&stack));
    }
    println!("{:?}",&stack);

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
