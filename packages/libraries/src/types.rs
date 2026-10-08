#[derive(Debug)]
pub enum Action {
    Still,
    Moving,
    Grabbed,
    //Actioned
}
#[derive(PartialEq)]
pub enum Effect {
    NoEffect,
    ChairSpin,
}

pub struct PetState {
    pub direction: (f32, f32),
    pub action: Action,
    pub current_effect: Effect,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}

pub struct PetStateEx {
    pub direction: (f32, f32),
    pub action: Action,
    pub current_effect: Effect,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub x_position: f32,
    pub y_position: f32,
    pub speed: (f32, f32),
}
