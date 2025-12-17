use crate::tokinezed;

/*
todo
например надо спарсить время d: '',h: '' ...
и в d:'' например могут быть не конретно дни а милисекунды и это удобная
обертка для работы с этим
*/
trait AbstractValue {
    type Item;
    fn set(&mut self, item: Self::Item);
    fn get_ref(&self) -> Option<&Self::Item>;
    fn get_owned(&self) -> Option<Self::Item>;
    fn size(&self) -> usize;
}

pub fn smart_abstract_multiconstruction_parse() {}

pub fn skip_value<T>() {}
