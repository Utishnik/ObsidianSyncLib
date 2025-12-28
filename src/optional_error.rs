pub enum OptionErr {
    Err(Box<dyn std::error::Error>),
    None,
}

impl OptionErr {
    fn Some(&mut self, err: Box<dyn std::error::Error>) {
        *self = OptionErr::Err(err);
    }
    fn None(&mut self) {
        *self = OptionErr::None;
    }
}
