pub trait Visitor<O> {
    type Result;

    fn visit(&mut self, object: &O) -> Self::Result;
}

pub trait Acceptor<O> {
    fn accept<V: Visitor<O>>(&self, visitor: V) -> V::Result;
}
