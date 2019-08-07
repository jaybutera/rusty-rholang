#[macro_use]
use nom::{
    IResult,
    sequence::{delimited, pair},
    character::complete::char,
    bytes::complete::{is_not, tag},
    branch::alt,
};
use crate::types::*;

pub fn term(input: &str) -> IResult<&str, Term> {
    alt((nil, send, receive, par))(input)
}

pub fn send(input: &str) -> IResult<&str, Term> {
    let (input, _)    = tag("!")(input)?;
    let (input, chan) = is_not("(")(input)?;
    let (input, _)    = char('(')(input)?;
    let (input, cont) = term(input)?;
    let (input, _)    = char(')')(input)?;

    Ok( (input,
         Term::Send(
             SendProc {
                 chan: chan.to_string(),
                 cont: Box::new(cont) })
         ) )
}

pub fn receive(input: &str) -> IResult<&str, Term> {
    let (input, _)    = tag("?")(input)?;
    let (input, chan) = is_not("(")(input)?;
    let (input, _)    = char('(')(input)?;
    let (input, cont) = term(input)?;
    let (input, _)    = char(')')(input)?;

    Ok( (input,
         Term::Receive(
             ReceiveProc {
                 chan: chan.to_string(),
                 cont: Box::new(cont) })
         ) )
}

pub fn par(input: &str) -> IResult<&str, Term> {
    let (input, _)     = pair(tag("par"), char('('))(input)?;
    let (input, proc1) = term(input)?;
    //let (input, _)     = ws!(tag("|"))(input)?;
    let (input, _)     = tag(" | ")(input)?;
    let (input, proc2) = term(input)?;
    let (input, _)     = char(')')(input)?;

    Ok( (input,
         Term::Par(
            Box::new(proc1),
            Box::new(proc2))
        ))
}

pub fn nil(input: &str) -> IResult<&str, Term> {
    let (input, _) = tag("Nil")(input)?;
    Ok( (input, Term::Nil) )
}

#[test]
fn test_send() {
    let program = "!x(Nil)";

    assert!(term(program).is_ok());
}
