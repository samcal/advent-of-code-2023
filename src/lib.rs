pub trait Semigroup {
    fn mappend(&self, b: &Self) -> Self;
}

pub trait Monoid: Semigroup {
    fn mempty() -> Self;
}

pub fn mconcat<M: Monoid>(ms: impl Iterator<Item = M>) -> M {
    ms.fold(M::mempty(), |acc, m| acc.mappend(&m))
}
