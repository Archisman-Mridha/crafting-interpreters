use ordered_float::OrderedFloat;

pub enum Value<'value> {
  Number(OrderedFloat<f64>),
  String(&'value str),
  Boolean(bool)
}
