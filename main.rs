use std::collections::HashMap;

//type Tuplespace = HashMap<String, Term>;
struct Tuplespace (HashMap<String, Vec<Term>>);

impl Tuplespace {
    fn insert(&mut self, t: Term, chan_name: String) -> Option<Term> {
        if let Some(v) = self.0.get_mut( chan_name ) {
            v.push(t);
            None
        } else {
            self.0.insert(chan_name, vec![t])
        }
    }
}

enum Term {
    Par(Box<Term>, Box<Term>),
    Receive(Box<Term>),
    Send(Box<Term>),
    Nil,
}

/*
fn reduce (term: Term, tuplespace: Tuplespace) -> Term {
    match term {
        Par(t1, t2) => 
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
    assert!(true, tspace.insert(expr1, "x".to_string()).is_none());
    assert!(true, tspace.insert(expr2, "x".to_string()).is_none());
}
