enum Term {
    Send(Box<Term>),
    Receive(Box<Term>),
}
