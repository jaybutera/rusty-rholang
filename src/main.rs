use std::collections::HashMap;

//type Tuplespace = HashMap<String, Term>;
//struct Tuplespace (HashMap<String, Vec<Term>>);
#[derive(Debug)]
struct Tuplespace {
    sends: HashMap<String, Vec<Term>>,
    recvs: HashMap<String, Vec<Term>>,
}

impl Tuplespace {
    fn new() -> Self {
        Tuplespace {
            sends: HashMap::new(),
            recvs: HashMap::new(),
        }
    }

    pub fn insert(&mut self, t: Term) {
        use crate::Term::*;

        match t {
            Send(p)    => self._add_to_sends(p),
            Receive(p) => self._add_to_recvs(p),
            Par(t1,t2) => {
                self.insert(*t1);
                self.insert(*t2);
            },
            Nil => (),
        };
    }

    fn _add_to_sends(&mut self, p: SendProc) {
        let SendProc {chan, cont} = p;

        if let Some(v) = self.sends.get_mut( &chan ) {
            v.push(*cont);
        } else {
            // TODO: This should never return Some(_), but that should be explicit
            self.sends.insert(chan, vec![*cont]);
        }
    }
    fn _add_to_recvs(&mut self, p: ReceiveProc) {
        let ReceiveProc {chan, cont} = p;

        if let Some(v) = self.recvs.get_mut( &chan ) {
            v.push(*cont);
        } else {
            // TODO: This should never return Some(_), but that should be explicit
            self.sends.insert(chan, vec![*cont]);
        }
    }
}

// Make Send and Receive processes types rather than just variants so that we can utilize the type
// checker on functions dealing with just one of them, and not all terms (see _add_to_sends/recvs fns)
#[derive(Debug)]
struct ReceiveProc {
    chan: String,
    cont: Box<Term>,
}

#[derive(Debug)]
struct SendProc {
    chan: String,
    cont: Box<Term>,
}

#[derive(Debug)]
enum Term {
    Par(Box<Term>, Box<Term>),
    Receive(ReceiveProc),
    Send(SendProc),
    Nil,
}

fn eval(term: Term, tspace: &mut Tuplespace) {
    //println!("{:?}", &tspace);
    //println!("term: {:?}", &term);

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

fn main() {
    use Term::*;

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
}
