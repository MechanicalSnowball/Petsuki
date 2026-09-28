#[derive(Debug)]
pub enum PetState {
    Still(f32, f32),
    Moving(f32, f32),
    Grabbed(f32, f32),
}

pub struct IndividualSpriteSize {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}
