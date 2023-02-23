pub trait Create<T> {
    /// Create a single record in the database
    fn create(&self) -> Result<T, diesel::result::Error>;
}

pub trait ReadWrite {
    /// Read a single record from the database
    fn read(&self) -> Self;

    /// Update a single record in the database
    fn update(&self) -> Result<(), diesel::result::Error>;

    /// Delete a single record from the database
    fn delete(&self) -> Result<(), diesel::result::Error>;

    /// Read all records from the database
    fn all() -> Result<Vec<Self>, diesel::result::Error> where Self: Sized;
}

pub trait Read<T> {
    /// Read a single record from the database
    fn read(&self) -> Result<T, diesel::result::Error>;
}

pub trait Relational<T> {
    fn get_with_relation() -> Result<Vec<T>, diesel::result::Error>;
}
