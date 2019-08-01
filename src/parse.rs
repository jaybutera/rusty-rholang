#[macro_use]
use nom::{named, tag};
use nom::{
    IResult,
    sequence::delimited,
    character::complete::char,
    bytes::complete::{is_not, tag},
    branch::alt,
};
use crate::types::*;

fn term(input: &str) -> IResult<&str, Term> {
    alt((nil, send, receive, par))(input)
}

fn send(input: &str) -> IResult<&str, Term> {
    let (input, _)    = tag("!")(input)?;
    let (input, chan) = is_not("(")(input)?;
    let (input, cont_str) = delimited(
        char('('), is_not(")"), char(')'))(input)?;

    let (_, cont) = term(cont_str)?;

    Ok( (input,
         Term::Send(
             SendProc {
                 chan: chan.to_string(),
                 cont: Box::new(cont) })
         ) )
}

fn receive(input: &str) -> IResult<&str, Term> {
    let (input, _)    = tag("?")(input)?;
    let (input, chan) = is_not("(")(input)?;
    let (input, cont_str) = delimited(
        char('('), is_not(")"), char(')'))(input)?;

    let (_, cont) = term(cont_str)?;

    Ok( (input,
         Term::Receive(
             ReceiveProc {
                 chan: chan.to_string(),
                 cont: Box::new(cont) })
         ) )
}

fn par(input: &str) -> IResult<&str, Term> {
    let (input, proc1) = term(input)?;
    let (input, _)     = tag("|")(input)?;
    let (input, proc2) = term(input)?;

    Ok( (input,
         Term::Par(
            Box::new(proc1),
            Box::new(proc2))
        ))
}

fn nil(input: &str) -> IResult<&str, Term> {
    let (input, _) = tag("Nil")(input)?;
    Ok( (input, Term::Nil) )
}

#[test]
fn test_send() {
    let program = "!x(Nil)";

    assert!(term(program).is_ok());
}

/*
fn main () -> std::io::Result<()> {
    let mut file = File::open("test.rho")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    println!("{:?}", send(&data));

    //println!("{:?}", test("heyo world!") );
    Ok(())
}
*/
