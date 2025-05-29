pub trait IterExt: Iterator + Sized {
    fn collvect(self) -> Vec<Self::Item> {
        self.collect::<Vec<Self::Item>>()
    }
}

impl<IT: Iterator> IterExt for IT {}
