use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take},
    character::complete::{alpha1, alphanumeric1, one_of},
    combinator::opt,
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated, tuple},
    AsChar, Err as NomErr, IResult, InputTakeAtPosition,
};
//use nom::Err;

#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    scheme: Scheme,
    authority: Option<Authority<'a>>,
    host: Host,
    port: Option<u16>,
    path: Option<Vec<&'a str>>,
    query: Option<QueryParams<'a>>,
    fragment: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

pub type Authority<'a> = (&'a str, Option<&'a str>);

#[derive(Debug, PartialEq, Eq)]
pub enum Host {
    HOST(String),
    IP([u8; 4]),
}

pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

impl From<&str> for Scheme {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "http://" => Scheme::HTTP,
            "https://" => Scheme::HTTPS,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}

fn scheme(input: &str) -> Res<&str, Scheme> {
    context(
        "scheme",
        alt((tag_no_case("HTTP://"), tag_no_case("HTTPS://"))),
    )(input)
    .map(|(next_input, res)| {
        println!("  next_input : {}", next_input);
        println!("  res : {}", res);
        (next_input, res.into())
    })
}

fn authority(input: &str) -> Res<&str, (&str, Option<&str>)> {
    context(
        "authority",
        terminated(
            separated_pair(alphanumeric1, opt(tag(":")), opt(alphanumeric1)),
            tag("@"),
        ),
    )(input)
}
// ================= HOST ======================
fn host(input: &str) -> Res<&str, Host> {
    context(
        "host",
        alt((
            tuple((many1(terminated(alphanumerichyphen1, tag("."))), alpha1)),
            tuple((many_m_n(1, 1, alphanumerichyphen1), take(0 as usize))),
        )),
    )(input)
    .map(|(next_input, mut res)| {
        println!("  host next_input : {}", next_input);
        println!("  host res : {:?}", &res);
        if !res.1.is_empty() {
            res.0.push(res.1);
        }
        (next_input, Host::HOST(res.0.join(".")))
    })
}

fn alphanumerichyphen1<T>(i: T) -> Res<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    i.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum()
        },
        nom::error::ErrorKind::AlphaNumeric,
    )
}
// =========================================

// =================== IP ==================
fn ip_num(input: &str) -> Res<&str, u8> {
    context("ip number", n_to_m_digits(1, 3))(input).and_then(|(next_input, result)| {
        println!("      ip_num next_input  {}", next_input);
        println!("      ip_num result  {:?}", result);
        match result.parse::<u8>() {
            Ok(n) => Ok((next_input, n)),
            Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
        }
    })
}

fn n_to_m_digits<'a>(n: usize, m: usize) -> impl FnMut(&'a str) -> Res<&str, String> {
    move |input| {
        many_m_n(n, m, one_of("0123456789"))(input).map(|(next_input, result)| {
            println!("          n_to_m_digits next_input  {}", next_input);
            println!("          n_to_m_digits result  {:?}", result);
            (next_input, result.into_iter().collect())
        })
    }
}

fn ip(input: &str) -> Res<&str, Host> {
    context(
        "ip",
        tuple((count(terminated(ip_num, tag(".")), 3), ip_num)),
    )(input)
    .map(|(next_input, res)| {
        println!("  ip next_input  {}", next_input);
        println!("  ip result  {:?}", res);
        let mut result: [u8; 4] = [0, 0, 0, 0];
        res.0
            .into_iter()
            .enumerate()
            .for_each(|(i, v)| result[i] = v);
        result[3] = res.1;
        (next_input, Host::IP(result))
    })
}
// =======================================

// ================ PATH =================
fn url_code_points<T>(i: T) -> Res<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    i.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            println!("      url_code_points item: {}", &char_item);
            !(char_item == '-') && !char_item.is_alphanum() && !(char_item == '.')
            // ... actual ascii code points and url encoding...: https://infra.spec.whatwg.org/#ascii-code-point
        },
        nom::error::ErrorKind::AlphaNumeric,
    )
}

fn path(input: &str) -> Res<&str, Vec<&str>> {
    context(
        "path",
        tuple((
            tag("/"),
            many0(terminated(url_code_points, tag("/"))),
            opt(url_code_points),
        )),
    )(input)
    .map(|(next_input, res)| {
        println!("  path next_input  {}", next_input);
        println!("  path res  {:?}", res);
        let mut path: Vec<&str> = res.1.iter().map(|p| p.to_owned()).collect();
        if let Some(last) = res.2 {
            path.push(last);
        }
        (next_input, path)
    })
}
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum AoType<'a> {
    Str(Box<&'a str>),
    Tkn(Box<&'a str>),
    Int(Box<i32>),
    Opr(Box<&'a str>),
}
fn test<'a>(s: Rc<RefCell<Vec<AoType<'a>>>>) {
    s.borrow_mut().push(AoType::Str(Box::new("test")));
}
fn test_2<'a>(s: Rc<RefCell<Vec<AoType<'a>>>>) {
    s.borrow_mut().pop();
}
// ==============================================
fn main() {
    let r = scheme("https://yay");
    match r {
        Ok((res, typ)) => match typ {
            Scheme::HTTP => println!("Scheme http {:?}", res),
            Scheme::HTTPS => println!("Scheme https {:?}", res),
        },
        Err(e) => println!("Scheme {}", e),
    };

    println!("============ Scheme ============ ");
    println!("Scheme {:?}", scheme("bla://yay"));
    println!("============  auto ============ ");
    println!("auto {:?}", authority("username:password@zupzup.org"));
    println!("============  HOST ============ ");
    println!("host {:?}", host("localhost:8080"));
    println!("host {:?}", host("example.org:8080"));
    println!("host {:?}", host("some-subsite.example.org:8080"));
    println!("============  IP ============ ");
    println!("ip {:?}", ip("192.168.0.1:8080"));
    println!("============  path ============ ");
    println!("ip {:?}", path("/a/b/c?d"));

    let mut stack: Rc<RefCell<Vec<AoType>>> = Rc::new(RefCell::new(Vec::new()));
    test(Rc::clone(&stack));
    println!("stack : {:?}", Rc::clone(&stack));
    println!("stack : {:?}", test_2(Rc::clone(&stack)));
    println!("stack : {:?}", Rc::clone(&stack));
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
