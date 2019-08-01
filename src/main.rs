use std::fs::File;
use std::io::prelude::*;
use rusty_rholang::types::*;

fn eval(term: Term, tspace: &mut Tuplespace) {
    //println!("{:?}", &tspace);
    //println!("term: {:?}", &term);
    //use rusty_rholang::types::Term::*;

    match term {
        // Look for a receive on the specified channel, if one exists evaluate the continuation,
        // otherwise push the send onto the channel in the tuplespace
        Term::Send(p) => {
            match tspace.recvs.get_mut(&p.chan) {
                Some(recvs) => {
                    let cont = recvs.pop()
                        .expect(&format!("There should not be an empty vector for channel {}", &p.chan));

                    // TODO: There's gotta be a better way to deal with this garbage collection
                    if recvs.len() == 0 {
                        tspace.recvs.remove_entry(&p.chan);
                    }
                    // TODO: Substitition and evaluate p continutation
                    eval(cont, tspace)
                }
                None => tspace.insert( Term::Send(p) ),
            }
        },
        Term::Receive(p) => {
            match tspace.sends.get_mut(&p.chan) {
                Some(sends) => {
                    let cont = sends.pop()
                        .expect(&format!("There should not be an empty vector for channel {}", &p.chan));

                    if sends.len() == 0 {
                        tspace.sends.remove_entry(&p.chan);
                    }

                    eval(cont, tspace)
                }
                None => tspace.insert( Term::Receive(p) ),
            }
        },
        Term::Par(t1,t2) => {
            eval(*t1, tspace);
            eval(*t2, tspace);
        }
        Term::Nil => (),
    }
}

fn main() -> std::io::Result<()> {
    use rusty_rholang::parse;
    use rusty_rholang::types::Term::*;

    let chan_x = "x".to_string();
    let expr1 = Send( SendProc {
        chan: chan_x.clone(),
        cont: Box::new(Receive( ReceiveProc { chan: chan_x.clone(), cont: Box::new(Nil) } ))
    });

    let expr2 = Par(
        Box::new( Send( SendProc{ chan: "y".to_string(), cont: Box::new(Nil) } ) ),
        Box::new( Receive( ReceiveProc{ chan: chan_x.clone(), cont: Box::new(Nil) }) )
    );

    let mut tspace = Tuplespace::new();
    println!("{:?}", tspace);

    eval(expr1, &mut tspace);
    println!("{:?}", tspace);

    eval(expr2, &mut tspace);
    println!("{:?}", tspace);

    let mut file = File::open("test.rho")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let (_, term) = parse::term(&data)
        .expect("Failed to parse program");

    println!("{:?}", term);

    Ok(())
}
