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
 
use std::fmt;
 
static DEBUG: bool = true;
 
#[derive(Debug,Clone)]
pub enum  AoType {
    Str(Box<String>),
    Tkn(Box<String>),
    Int(Box<i32>),
    Opr(Box<String>),
    //Ass(Box<Vec<str>>),
    Ass(Vec<Box<String>>),
    Var(Box<String>),
    Cmd(Box<String>),
    //Fct(Box<String>),
    Fct(Vec<AoType>),
    Spc,
}
 
impl fmt::Display for AoType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{:?}", self)
        match self {
            AoType::Ass(a) => {write!(f, "Ass {:?}", a)},
            AoType::Str(a) => {write!(f, "Str {:?}", a)},
            AoType::Tkn(a) => {write!(f, "Tkn {:?}", a)},
            AoType::Int(a) => {write!(f, "Int {:?}", a)},
            AoType::Opr(a) => {write!(f, "Opr {:?}", a)},
            AoType::Var(a) => {write!(f, "Var {:?}", a)},
            AoType::Cmd(a) => {write!(f, "Cmd {:?}", a)},
            AoType::Fct(a) => {write!(f, "Fct [{:?}]", a)},
            AoType::Spc => {write!(f, "")},
        }
    }
}
 
type Res<T, U> = IResult<T, U, VerboseError<T>>;
fn typ_int(input: &str) -> Res<&str, AoType> {
    if DEBUG {println!("integer input : {}",&input);}
    //let vec:Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9'];
    //context("intger",  take_till(|c: char| c == ' ' || !vec!['0','1','2','3','4','5','6','7','8','9'].contains(&c) ))(input)
    context("intger",  take_till(|c: char| c == ' ' || !c.is_digit(10) ))(input)
        .map(|(next_input, res)| {
            if DEBUG {println!("  typ_int next_input {:?}",next_input);}
            if DEBUG {println!("  typ_int res {:?}",&res);}
            (next_input, AoType::Int(Box::new(res.parse::<i32>().unwrap())))
        })
}
 
fn typ_token(input: &str) -> Res<&str, AoType> {
    if DEBUG {println!("integer input : {}",&input);}
    //let vec:Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9'];
    //context("intger",  take_till(|c: char| c == ' ' || !vec!['0','1','2','3','4','5','6','7','8','9'].contains(&c) ))(input)
    context("token",  preceded(tag("'"),take_till(|c: char| c == ' ' || !c.is_alphanumeric()) ))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Tkn(Box::new(String::from(res))))
        })
}
 
fn typ_string(input: &str) -> Res<&str, AoType> {
    delimited(tag("\""), take_while(|x| x != '\"'), tag("\""))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Str(Box::new(String::from(res))))
        })
}
 
/*
fn ao_function(input: &str) -> Res<&str, AoType> {
    delimited(tag("["), take_till(|x| x == ']'),tag("]"))(input)
        .map(|(next_input, res)| {
            if DEBUG {println!("ao_function next_input : {:?}",next_input);}
            if DEBUG {println!("ao_function res : {:?}",res);}
            (next_input, AoType::Fct(Box::new(String::from(res))))
        })
}
*/
 
fn ao_var(input: &str) -> Res<&str, AoType> {
    if DEBUG {println!("ao_var input : |{}|",&input);}
    delimited(
        tag("("),
        tuple((many0(terminated( alphanumeric1, tag(" "))),
               opt(alphanumeric1), )),
        tag(")")
    )
    (input).map(|(next_input, mut res)| {
        if DEBUG {println!("ao_var next_input {:?}",&next_input);}
        if DEBUG {println!("ao_var res {:?}",&res);}
        let mut ires:Vec<Box<String>> = Vec::new();
 
        for v in res.0 {
            ires.push(Box::new(String::from(v)))
        }
 
        match &res.1 {
            Some(p) => ires.push(Box::new(String::from(*p))),
            None => {}
        };
 
        let ires_f = AoType::Ass(ires);
        (next_input,ires_f)
    })
}
 
fn ao_val(input: &str) -> Res<&str, AoType> {
    pair(tag("$"),alphanumeric1)(input).map(|(next_input, mut res)| {
        if DEBUG {println!("ao_val : {:?}",res);}
        (next_input, AoType::Var(Box::new(String::from(res.1))))
    })
}
 
fn ao_space(input: &str) -> Res<&str, AoType> {
   alt((tag("\n"),tag("\t"),multispace1))(input)
    .map(|(next_input, mut res)| {
        if DEBUG {println!("ao_space : {:?}",res);}
        (next_input, AoType::Spc)
    })
}
 
fn ao_function(input: &str) -> Res<&str, AoType> {
    println!("ao_function : |{:?}|",&input);
    delimited(tag("["), l_ao_all ,tag("]"))(input)
        .map(|(next_input, res)| {
            if DEBUG {println!("*********** AO_FUNCTION ***********");}
            if DEBUG {println!("ao_function next_input : {:?}",next_input);}
            if DEBUG {println!("ao_function res : {:?}",res.0);}
            let mut res_2: Vec<AoType> = Vec::new();
            let mut res_4: Vec<AoType> = Vec::new();
            if DEBUG {
                for x in res.0{
                    println!("aofunction res : {:?}",x);
                    res_2.push(x.clone());
                    res_4.push(x);
                }
            }
            if DEBUG {println!("ao_function res : {:?}",AoType::Fct(res_2));}
            if DEBUG {println!("*********** AO_FUNCTION ***********");}
//(next_input, AoType::Fct(Box::new(String::from(res))))
            //(next_input, AoType::Spc)
            (next_input, AoType::Fct(res_4))
           
        })
}
 
// 2ème étape
fn ao_all(input: &str) -> Res<&str, AoType> {
    alt((ao_operator,ao_command,ao_function,ao_var,ao_val,typ_string,typ_token,typ_int))(input)
    //                            |----^----|
    //alt((ao_operator,ao_command,ao_var,ao_val,typ_string,typ_token,typ_int))(input)
}
 
// 1er étape
pub fn l_ao_all(input: &str) -> Res<&str, (Vec<AoType>, Option<AoType>)> {
    context(
        "l_ao_all",
        tuple((many0(terminated( ao_all, ao_space)),
               opt(ao_all),
        )),
    )(&input).map(|(next_input, mut res)| {
        if DEBUG {println!("  l_ao_all next_input {:?}",next_input);}
        if DEBUG {println!("  l_ao_all res {:?}",&res);}
        match &res.1 {
            Some(p) => res.0.push(p.clone()),
            None => {}
        };
        (next_input,res) })
}
 
fn l_ao_all_box(input: &String) -> Res<&str, (Vec<AoType>, Option<AoType>)> {
    context(
        "l_ao_all",
        tuple((many0(terminated( ao_all, ao_space)),
               opt(ao_all),
        )),
    )(&input).map(|(next_input, mut res)| {
        if DEBUG {println!("  l_ao_all next_input {:?}",next_input);}
        if DEBUG {println!("  l_ao_all res {:?}",&res);}
        match &res.1 {
            Some(p) => res.0.push(p.clone()),
            None => {}
        };
        (next_input,res) })
}
 
 
fn ao_operator(input: &str) -> Res<&str, AoType> {
    alt((tag("+"),tag("-"),tag("*"),tag("/"),tag(">"),tag("<")))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Opr(Box::new(String::from(res))))
        })
}
 
fn ao_command(input: &str) -> Res<&str, AoType> {
    alt((tag("dup"),tag("eval"),tag("if"),tag("while")))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Cmd(Box::new(String::from(res))))
        })       
}
 
// ==================== Arithmetique function ====================
 
fn add(op1:AoType,op2:AoType) -> AoType {
 
    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};
 
    AoType::Int(Box::new(*i_op1 + *i_op2))
}
fn sub(op1:AoType,op2:AoType) -> AoType {
 
    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};
 
    AoType::Int(Box::new(*i_op2 - *i_op1))
}
fn mul(op1:AoType,op2:AoType) -> AoType {
 
    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};
 
    AoType::Int(Box::new(*i_op1 * *i_op2))
}
fn div(op1:AoType,op2:AoType) -> AoType {
 
    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};
 
    AoType::Int(Box::new(*i_op2 / *i_op1))
}
fn sup(op1:AoType,op2:AoType) -> AoType {
 
    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};
    if i_op2 > i_op1
        {AoType::Int(Box::new(1))}
    else
        {AoType::Int(Box::new(0))}
}
fn inf(op1:AoType,op2:AoType) -> AoType {
 
    let AoType::Int(i_op1) = op1 else {panic!("add op1 wrong type")};
    let AoType::Int(i_op2) = op2 else {panic!("add op2 wrong type")};
    if i_op2 < i_op1
        {AoType::Int(Box::new(1))}
    else
        {AoType::Int(Box::new(0))}
}
//fn eval<'c>(lex: AoType<'c>,env:&mut Vec<AoType<'c>>, stack:&mut Vec<AoType<'c>>) -> AoType<'c> {
fn eval(lex: AoType,env:&mut HashMap<String,AoType>, st: Rc<RefCell<Vec<AoType>>>) -> AoType {
 
    //let mut vec_ref = Rc::clone(st).borrow_mut();
    //println!("start ==> {:?}",Rc::clone(&st).borrow_mut().pop());
    //println!("start ==> {:?}",Rc::clone(&st).borrow_mut().pop());
 
        match lex {
            AoType::Str(_)    => {
                if DEBUG {println!("Eval(Str_)");}
                st.borrow_mut().push(lex);AoType::Tkn(Box::new(String::from("void")))
            },
            AoType::Tkn(_)    => {
                if DEBUG {println!("Eval(Tkn_)");}
                st.borrow_mut().push(lex);AoType::Tkn(Box::new(String::from("void")))
            },
            AoType::Int(_)    => {
                if DEBUG {println!("Eval(Int_)");}
                st.borrow_mut().push(lex);AoType::Tkn(Box::new(String::from("void")))
            },
           
            AoType::Opr(val)  => {
                if DEBUG {println!("Eval(Opr_)");}
                //AoType::Tkn(Box::new("void"));
                if DEBUG {println!("Opr : {:?}",val);}
                let mut v = st.borrow_mut();
                let op1 = v.pop().unwrap();
                let op2 = v.pop().unwrap();
                match &*val.as_str() {
                    "+" => v.push(add(op1,op2)),
                    "-" => v.push(sub(op1,op2)),
                    "*" => v.push(mul(op1,op2)),
                    "/" => v.push(div(op1,op2)),
                    ">" => v.push(sup(op1,op2)),
                    "<" => v.push(inf(op1,op2)),
                    _ => {}
                }
               
                AoType::Tkn(Box::new(String::from("void")))
            },
            AoType::Opr(val)  => {
                if DEBUG {println!("Eval(Opr_)");}
                AoType::Tkn(Box::new(String::from("void")))
            }
            AoType::Ass(val)  => {
                for k in val.iter() {
                    if DEBUG {println!("Assignement : {:?}",&k);}
                    env.insert(k.to_string(), st.borrow_mut().pop().unwrap());  
                }
                AoType::Tkn(Box::new(String::from("void")))
            },
            AoType::Var(k) => {                
                if DEBUG {println!("Eval(var) : {:?}",&k);}
                match env.get(&k.to_string()) {
                    Some(v) => {
                        st.borrow_mut().push(v.clone());
                        if DEBUG {println!("var stack after : {:?}",&st);}
                    }
                    None => {}
                }
                AoType::Tkn(Box::new(String::from("void")))
            },
            AoType::Cmd(c) => {
                if DEBUG {println!("Eval CMD : {:?}",c);}
                if c.eq( &Box::new(String::from("eval")) ) {
                    let v = st.borrow_mut().pop().unwrap();
                    match v {
                        AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                        _ => {}
                    }
                }
                else if c.eq(&Box::new(String::from("if"))) {
                    // do the if
                    let then = st.borrow_mut().pop().unwrap();
                    let test = st.borrow_mut().pop().unwrap();
                    match test {
                        AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                        _ => {}
                    }
                    let res = st.borrow_mut().pop().unwrap();
                    match res {
                        AoType::Int(i) => {
                            if i == Box::new(1) {
                                match then {
                                    AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                else if c.eq(&Box::new(String::from("while"))) {
                    // do the if
                    let corps = st.borrow_mut().pop().unwrap();
                    let test = st.borrow_mut().pop().unwrap();
                    loop {
                        match &test {
                            AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                            _ => {}
                        }
                        let res = st.borrow_mut().pop().unwrap();
                        match res {
                            AoType::Int(i) => {
                                if i == Box::new(1) {
                                    match &corps {
                                        AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                                        _ => {}
                                    }
                                } else {break}
                            }
                            _ => {}
                        }
                    }
                }
                AoType::Tkn(Box::new(String::from("void")))
            }
            AoType::Fct(_) => {
                if DEBUG {println!("Eval Fct on stack");}
                st.borrow_mut().push(lex);
                AoType::Tkn(Box::new(String::from("void")))
            }
            AoType::Spc => {
                if DEBUG {println!("Eval Spc :");}
                AoType::Tkn(Box::new(String::from("void")))
            }
        }
}
 
fn eval_box(lex: Box<AoType>,env:&mut HashMap<String,AoType>, st: Rc<RefCell<Vec<AoType>>>) -> AoType {
 
    //let mut vec_ref = Rc::clone(st).borrow_mut();
    //println!("start ==> {:?}",Rc::clone(&st).borrow_mut().pop());
    //println!("start ==> {:?}",Rc::clone(&st).borrow_mut().pop());
 
    match *lex {
        AoType::Str(_)    => {st.borrow_mut().push(*lex);AoType::Tkn(Box::new(String::from("void")))},
        AoType::Tkn(_)    => {st.borrow_mut().push(*lex);AoType::Tkn(Box::new(String::from("void")))},
        AoType::Int(_)    => {st.borrow_mut().push(*lex);AoType::Tkn(Box::new(String::from("void")))},
       
        AoType::Opr(val)  => {
            //AoType::Tkn(Box::new("void"));
            if DEBUG {println!("Opr : {:?}",val);}
            let mut v = st.borrow_mut();
            let op1 = v.pop().unwrap();
            let op2 = v.pop().unwrap();
            match &*val.as_str() {
                "+" => v.push(add(op1,op2)),
                "-" => v.push(sub(op1,op2)),
                "*" => v.push(mul(op1,op2)),
                "/" => v.push(div(op1,op2)),
                ">" => v.push(sup(op1,op2)),
                "<" => v.push(inf(op1,op2)),
                _ => {}
            }
           
            AoType::Tkn(Box::new(String::from("void")))
        },
        AoType::Opr(val)  => {AoType::Tkn(Box::new(String::from("void")))}
        AoType::Ass(val)  => {
            for k in val.iter() {
                if DEBUG {println!("Assignement : {:?}",&k);}
                env.insert(k.to_string(), st.borrow_mut().pop().unwrap());  
            }
            AoType::Tkn(Box::new(String::from("void")))
        },
        AoType::Var(k) => {
            if DEBUG {println!("var : {:?}",&k);}
            match env.get(&k.to_string()) {
                Some(v) => {st.borrow_mut().push(v.clone())}
                None => {}
            }
            AoType::Tkn(Box::new(String::from("void")))
        },
        AoType::Cmd(c) => {
            if DEBUG {println!("CMD : {:?}",c);}
            if c.eq( &Box::new(String::from("eval")) ) {
                let v = st.borrow_mut().pop().unwrap();
                match v {
                    AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                    _ => {}
                }
            }
            else if c.eq(&Box::new(String::from("if"))) {
                // do the if
                let then = st.borrow_mut().pop().unwrap();
                let test = st.borrow_mut().pop().unwrap();
                match test {
                    AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                    _ => {}
                }
                let res = st.borrow_mut().pop().unwrap();
                match res {
                    AoType::Int(i) => {
                        if i == Box::new(1) {
                            match then {
                                AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            else if c.eq(&Box::new(String::from("while"))) {
                // do the if
                let corps = st.borrow_mut().pop().unwrap();
                let test = st.borrow_mut().pop().unwrap();
                loop {
                    match &test {
                        AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                        _ => {}
                    }
                    let res = st.borrow_mut().pop().unwrap();
                    match res {
                        AoType::Int(i) => {
                            if i == Box::new(1) {
                                match &corps {
                                    AoType::Fct(f) =>  interp_AoType(&f, env, Rc::clone(&st)),
                                    _ => {}
                                }
                            } else {break}
                        }
                        _ => {}
                    }
                }
            }
            AoType::Tkn(Box::new(String::from("void")))
        }
        AoType::Fct(_) => {
            st.borrow_mut().push(*lex);
            AoType::Tkn(Box::new(String::from("void")))
        }
        AoType::Spc => {
            AoType::Tkn(Box::new(String::from("void")))
        }
    }
}
 
 
pub fn interp(code:&str, env:&mut HashMap<String,AoType>, st: Rc<RefCell<Vec<AoType>>>) {
    let lex = l_ao_all(code);
    if let Err(ref lex2) = lex{
        if DEBUG {println!("Syntax error");}
    }
    else {
        if DEBUG {println!("analyse lexical :{:?}",&lex);}
        for v in lex.unwrap().1.0{
            eval(v,env,Rc::clone(&st));
        }
    }
}
 
pub fn interp_AoType(code:&Vec<AoType>, env:&mut HashMap<String,AoType>, st: Rc<RefCell<Vec<AoType>>>) {
 
        if DEBUG {println!("analyse lexical :{:?}",&code);}
        for v in code{
            eval(v.clone(),env,Rc::clone(&st));
        }
   
}
 
/*
pub fn interp_box(code:&String, env:&mut HashMap<String,AoType>, st: Rc<RefCell<Vec<AoType>>>) {
    let lex = l_ao_all_box(code);
    if let Err(ref lex2) = lex{
        if DEBUG {println!("Syntax error");}
    }
    else {
        if DEBUG {println!("analyse lexical :{:?}",&lex);}
        for v in lex.unwrap().1.0{
            eval_box(Box::new(v),env,Rc::clone(&st));
        }
    }
}
*/


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
