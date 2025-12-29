#[derive(PartialEq)]
pub enum OptionErr<T> {
    Err(T),
    None,
}

impl<T> OptionErr<T> {
    fn Some(&mut self, err: T) {
        *self = OptionErr::Err(err);
    }
    fn None(&mut self) {
        *self = OptionErr::None;
    }
}
