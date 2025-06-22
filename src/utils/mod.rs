pub mod symbolics;
pub mod vec;

pub trait ToS: ToString {
    fn s(&self) -> String {
        self.to_string()
    }
}

impl<S> ToS for S where S: ToString {}
