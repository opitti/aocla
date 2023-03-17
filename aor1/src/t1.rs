use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take, take_till, take_while, take_while1},
    character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1, multispace1, one_of},
    character::is_digit,
    combinator::{map_parser, opt},
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    AsChar, Err as NomErr, IResult, InputTakeAtPosition,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, Clone)]
pub enum AoType {
    Str(Box<String>),
    Tkn(Box<String>),
    Int(Box<i32>),
    Opr(Box<String>),
    //Ass(Box<Vec<str>>),
    Ass(Vec<Box<String>>),
    Var(Box<String>),
    //Cmd(Box<String>),
    Cmd(AoKeyword),
    //Fct(Box<String>),
    Fct(Vec<AoType>),
    Lst(Vec<AoType>),
    Spc,
}

impl AoType {
    fn to_fct(&self) -> Option<AoType> {
        match self {
            AoType::Lst(lst) => Some(AoType::Fct(lst.to_vec())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AoKeyword {
    Eval,
    If,
    While,
    Def,
}

static DEBUG: bool = true;

fn ao_val(input: &str) -> Res<&str, AoType> {
    println!("2. ao_val start : {}", input);
    for (i, c) in input.chars().enumerate() {
        println!("ao_val: {}, {}", i, c);
    }
    alt((
        tuple((alpha1, alphanumeric0)),
        pair(tag("$"), alphanumeric1),
    ))(input)
    .map(|(next_input, mut res)| {
        if DEBUG {
            println!("ao_val next_input : {:?}", next_input);
        }
        if DEBUG {
            println!(
                "ao_val res : {:?} {} {}",
                res,
                res.0.is_empty(),
                res.1.is_empty()
            );
        }
        if (res.0.is_empty() == false) && (res.1.is_empty() == true) {
            (next_input, AoType::Var(Box::new(String::from(res.0))))
        } else {
            (next_input, AoType::Var(Box::new(String::from(res.1))))
        }
    })
}
fn main() {
    println!("{:?}", ao_val("$tsup"));
}
