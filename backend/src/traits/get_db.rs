pub trait GetFromDb: Sized { 
    fn get_by_id(id:i32) -> Result<Self, diesel::result::Error>;
}
