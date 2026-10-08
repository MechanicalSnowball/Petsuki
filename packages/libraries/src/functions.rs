use crate::types::*;
use raylib::ffi::Vector2;

pub trait MoveCloser {
    fn move_towards_orthogonal(
        self,
        target: Vector2,
        max_distance: f32,
        current_state: &mut PetState,
    ) -> Vector2;
}

impl MoveCloser for Vector2 {
    fn move_towards_orthogonal(
        self,
        target: Vector2,
        max_distance: f32,
        current_state: &mut PetState,
    ) -> Vector2 {
        let mut moved_a_bit = Vector2::new(0.0, 0.0);
        let dx = target.x - self.x;
        let dy = target.y - self.y;
        let close_enough = 5.0;

        if dx.abs() >= close_enough {
            moved_a_bit.x = self.x + (dx.signum()) * max_distance;
            moved_a_bit.y = self.y;
            current_state.direction = (dx.signum(), 0.0);
            current_state.action = Action::Moving;

            moved_a_bit
        } else if dy.abs() >= close_enough {
            moved_a_bit.x = self.x;
            moved_a_bit.y = self.y + (dy.signum()) * max_distance;
            current_state.direction = (0.0, dy.signum());
            current_state.action = Action::Moving;

            moved_a_bit
        } else {
            current_state.action = Action::Still;

            target
        }
    }
}

//puede ser reescrito usando .iter().any()
pub fn is_mouse_on_any_pet(mouse_pos: (i32, i32), areas: &[PetStateEx]) -> bool {
    for mado_pet in areas.iter() {
        if mado_pet.x_position <= mouse_pos.0 as f32
            && mouse_pos.0 as f32 <= mado_pet.x_position + (mado_pet.width * mado_pet.scale)
            && mado_pet.y_position <= mouse_pos.1 as f32
            && mouse_pos.1 as f32 <= mado_pet.y_position + (mado_pet.height * mado_pet.scale)
        {
            return true;
        }
    }
    false
}
