use hecs::World;

pub struct Document(pub World);

impl Document {
    pub fn new() -> Document {
        Document(World::new())
    }
}