#[derive(Debug)]
pub enum Action {
    Still,
    Moving,
    Grabbed,
    //Actioned
}

//pub enum Effect {por hacer}

pub struct PetState {
    pub direction: (f32, f32),
    pub action: Action,
    //pub CurrentEffect: Effect,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}
