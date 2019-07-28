use std::collections::HashMap;

//type Tuplespace = HashMap<String, Term>;
#[derive(Debug)]
//struct Tuplespace (HashMap<String, Vec<Term>>);
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

    fn insert(&mut self, t: Term, chan_name: String) {
        use crate::Term::*;

        match t {
            Send(cont) => self._add_to_space(*cont, chan_name),
            Receive(cont) => self._add_to_space(*cont, chan_name),
            Par(t1,t2) => {
                self._add_to_space(*t1, chan_name.clone());
                self._add_to_space(*t2, chan_name.clone());
            },
            Nil => (),
        };
    }

    fn _add_to_space(&mut self, t: Term, chan_name: String) {
        if let Some(v) = self.sends.get_mut( &chan_name ) {
            v.push(t);
        } else {
            // TODO: This should never return Some(_), but that should be explicit
            self.sends.insert(chan_name, vec![t]);
        }
    }
}

#[derive(Debug)]
enum Term {
    Par(Box<Term>, Box<Term>),
    Receive(Box<Term>),
    Send(Box<Term>),
    Nil,
}

/*
fn eval(term: Term, tspace: Tuplespace) -> Term {
    match term {
        Send(t) => {
            match tspace.get(chan) {
            }
        },
        //Par(t1, t2) => eval(t1, tuplespace)
    }
}
*/

fn main() {
    use Term::*;

    let expr1 = Send( Box::new(Receive( Box::new(Nil) )) );

    let expr2 = Par(
        Box::new( Send( Box::new(Nil) ) ),
        Box::new( Receive( Box::new(Nil)) )
    );

    let mut tspace = Tuplespace::new();
    println!("{:?}", tspace);

    tspace.insert(expr1, "x".to_string());
    tspace.insert(expr2, "x".to_string());
    //assert!(true, tspace.insert(expr1, "x".to_string()).is_none());
    //assert!(true, tspace.insert(expr2, "x".to_string()).is_none());

    println!("{:?}", tspace);
}
