use std::collections::HashMap;

//type Tuplespace = HashMap<String, Term>;
//struct Tuplespace (HashMap<String, Vec<Term>>);
// TODO: Implement a reduction (or removal) as part of the tuplespace
// TODO: that should allow me to make sends/recvs private too
#[derive(Debug)]
pub struct Tuplespace {
    pub sends: HashMap<String, Vec<Term>>,
    pub recvs: HashMap<String, Vec<Term>>,
}

impl Tuplespace {
    pub fn new() -> Self {
        Tuplespace {
            sends: HashMap::new(),
            recvs: HashMap::new(),
        }
    }

    pub fn insert(&mut self, t: Term) {
        use Term::*;

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
            self.recvs.insert(chan, vec![*cont]);
        }
    }
}

// Make Send and Receive processes types rather than just variants so that we can utilize the type
// checker on functions dealing with just one of them, and not all terms (see _add_to_sends/recvs fns)
#[derive(Debug)]
pub struct ReceiveProc {
    pub chan: String,
    pub cont: Box<Term>,
}

#[derive(Debug)]
pub struct SendProc {
    pub chan: String,
    pub cont: Box<Term>,
}

#[derive(Debug)]
pub enum Term {
    Par(Box<Term>, Box<Term>),
    Receive(ReceiveProc),
    Send(SendProc),
    Nil,
}
