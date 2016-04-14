pub enum OptErr<T, E> {
    Full(T),
    Empty,
    Error(E),
}
