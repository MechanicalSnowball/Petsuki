use crate::types::*;
use raylib::ffi::Vector2;

pub trait OrthogonalMovement {
    fn move_towards_orthogonal(
        self,
        target: Vector2,
        max_distance: f32,
        current_state: &mut PetState,
    ) -> Vector2;
}

impl OrthogonalMovement for Vector2 {
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
