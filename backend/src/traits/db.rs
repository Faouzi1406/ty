pub trait Create<T> {
    fn create(&self) -> Result<T, diesel::result::Error>;
}

pub trait ReadWrite {
    fn read(&self) -> Self;
    fn update(&self) -> Result<(), diesel::result::Error>;
    fn delete(&self) -> Result<(), diesel::result::Error>;
    fn all() -> Result<Vec<Self>, diesel::result::Error> where Self: Sized;
}
