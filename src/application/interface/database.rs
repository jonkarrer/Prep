#[derive(Clone)]
pub struct Database<T> {
    pub pool: T,
}
