use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take,take_till,take_while},
    character::complete::{alpha1, digit1, alphanumeric1, one_of},
    character::is_digit,
    combinator::{opt,map_parser},
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated, tuple,delimited},
    AsChar, Err as NomErr, IResult, InputTakeAtPosition,
};
#[derive(Debug)]
pub enum  AoType<'a> {
    Str(Box<&'a str>),
    Tkn(Box<&'a str>)
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn test_parser_token(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == ' ' )(input)
        .map(|(next_input, res)| {
            (next_input, res)
        })
}

fn token(input: &str) -> Res<&str, &str> {
    println!("integer input : {}",&input);
    context("intger",  take_till(|c| c == ' '))(input)

}

fn typ_token(input: &str) -> Res<&str, AoType> {
    println!("integer input : {}",&input);
    context("intger",  take_till(|c| c == ' '))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Tkn(Box::new(res)))
        })

}

fn ao_string(input: &str) -> Res<&str, &str> {
    delimited(tag("\""), take_while(|x| x != '\"'), tag("\""))(input)
}

fn typ_string(input: &str) -> Res<&str, AoType> {
    delimited(tag("\""), take_while(|x| x != '\"'), tag("\""))(input)
        .map(|(next_input, res)| {
            (next_input, AoType::Str(Box::new(res)))
        })
}

//fn lToken(input: &str) -> Res<&str, (Vec<&str>, Option<&str>)> {
fn lToken(input: &str) -> Res<&str, Vec<AoType>> {
    context(
        "lInteger",
        tuple((many0(terminated( token, tag(" "))),
            opt(token),
        )),
    )(input).map(|(next_input, mut res)| {
        println!("  lInteger next_input {:?}",next_input);
        println!("  lInteger res {:?}",&res);
        (next_input,res) }).map(|(next_input, mut res)| {
        println!("  lInteger next_input {:?}",next_input);
        println!("  lInteger res {:?}",&res);
        match  res.1 {
            Some(n) => res.0.push(n),
            None => {}
        };
        let mut vres: Vec<AoType> = Vec::new();
        res.0
            .into_iter()
            .enumerate()
            .for_each(|(i, v)| vres.push(AoType::Tkn(Box::new(v))));
        (next_input,vres) })
}

//fn lAoString(input: &str) -> Res<&str, (Vec<&str>, Option<&str>)> {
fn lAoString(input: &str) -> Res<&str, Vec<AoType>> {
    context(
        "lInteger",
        tuple((many0(terminated( ao_string, tag(" "))),
               opt(ao_string),
        )),
    )(input).map(|(next_input, mut res)| {
        println!("  lInteger next_input {:?}",next_input);
        println!("  lInteger res {:?}",&res);
        match  res.1 {
            Some(n) => res.0.push(n),
            None => {}
        };
        let mut vres: Vec<AoType> = Vec::new();
        res.0
            .into_iter()
            .enumerate()
            .for_each(|(i, v)| vres.push(AoType::Str(Box::new(v))));
        (next_input,vres) })
}

fn ao_all(input: &str) -> Res<&str, AoType> {
    alt((typ_string,typ_token))(input)
}

fn l_ao_all(input: &str) -> Res<&str, (Vec<AoType>, Option<AoType>)> {
    context(
        "lInteger",
        tuple((many0(terminated( ao_all, tag(" "))),
               opt(ao_all),
        )),
    )(input).map(|(next_input, mut res)| {
        println!("  lInteger next_input {:?}",next_input);
        println!("  lInteger res {:?}",&res);
        (next_input,res) })
}

/*
fn token(input: &str) -> Res<&str, &str> {

        alt((alpha1, is_digit))(input)

}
*/
fn main() {
    println!("AO start");
    eprintln!("test parser {:?}",test_parser_token("test 12"));
    println!("integer {:?}",token("1a 13 13 15"));
    println!("lInteger {:?}",lToken("test \"13\" 13 15"));

    println!("ao string {:?}",ao_string("\"test de ao string\""));
    println!("lAoString {:?}",lAoString("\"test de ao string\" \"test de ao string 2\" \"test de ao string 3\""));

    println!("all {:?}",ao_all("12"));
    println!("all {:?}",ao_all("\"test de ao string\""));

    println!("lall {:?}",l_ao_all("12 \"test de ao string\""));
    println!("lall {:?}",l_ao_all("12 13 \"test de ao string\" \"test de ao string\" 14 'olivier"));
}
