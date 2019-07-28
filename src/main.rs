use std::collections::HashMap;

//type Tuplespace = HashMap<String, Term>;
#[derive(Debug)]
//struct Tuplespace (HashMap<String, Vec<Term>>);
struct Tuplespace {
    sends: HashMap<String, Vec<Term>>,
    recvs: HashMap<String, Vec<Term>>,
}

impl Tuplespace {
    fn insert(&mut self, t: Term, chan_name: String) -> Option<Term> {
        use Term;

        match t {
            Send(cont) => self._add_to_space(t, chan_name),
            Receive(cont) => self._add_to_space(t, chan_name),
            // Add both terms in the par to the tuplespace independently. If the first term fails
            Par(t1,t2) => {
                match _add_to_space(t1, chan_name.clone()) {
                    Some(t) => Some( Par(t,t2) ),
                    None => match _add_to_space(t2, chan_name) {
                        Some(t) => Some( Par(t1,t) ),
                        None => None,
                    },
                }
            }
        }
    }

    fn _add_to_space(&mut self, t: Term, chan_name: String) -> Option<Term> {
        if let Some(v) = self.sends.get_mut( &chan_name ) {
            v.push(t);
            None
        } else {
            match self.sends.insert(chan_name, vec![t]) {
                Some(mut v) => v.pop(),
                None => None,
            }
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

    let mut tspace = Tuplespace( HashMap::new() );
    println!("{:?}", tspace);

    tspace.insert(expr1, "x".to_string());
    tspace.insert(expr2, "x".to_string());
    //assert!(true, tspace.insert(expr1, "x".to_string()).is_none());
    //assert!(true, tspace.insert(expr2, "x".to_string()).is_none());

    println!("{:?}", tspace);
}
