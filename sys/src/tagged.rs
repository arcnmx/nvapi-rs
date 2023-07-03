pub trait TaggedData {
    type Repr: Copy + Ord;
    type Id: Copy + Ord + TryFrom<Self::Repr>;

    fn tag(&self) -> Self::Repr;
}

impl<I: Copy + Ord, T> TaggedData for (I, T) where
{
    type Repr = I;
    type Id = I;

    fn tag(&self) -> Self::Repr {
        self.0
    }
}
