use core::f32;

use raylib::ffi::Vector2;

use crate::PetState::{Grabbed, Moving, Still};

#[derive(Debug)]
enum PetState {
    Still(f32, f32),
    Moving(f32, f32),
    Grabbed(f32, f32),
}

struct IndividualSpriteSize {
    width: f32,
    height: f32,
    scale: f32,
}

trait OrthogonalMovement {
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
            *current_state = PetState::Moving(dx.signum(), 0.0);

            moved_a_bit
        } else if dy.abs() >= close_enough {
            moved_a_bit.x = self.x;
            moved_a_bit.y = self.y + (dy.signum()) * max_distance;
            *current_state = PetState::Moving(0.0, dy.signum());

            moved_a_bit
        } else {
            let direction = match current_state {
                PetState::Moving(a, b) => (a, b),
                PetState::Still(a, b) => (a, b),
                PetState::Grabbed(a, b) => (a, b),
            };
            *current_state = PetState::Still(direction.0.clone(), direction.1.clone());

            target
        }
    }
}

fn main() {
    use raylib::prelude::*;

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("The game: you lost!")
        .topmost()
        .undecorated()
        .transparent()
        .build();

    let (scr_wd, scr_hg) = {
        let monitor = get_current_monitor();
        let width = get_monitor_width(monitor);
        let height = get_monitor_height(monitor);

        (width, height)
    };

    let mado_sprite_size = IndividualSpriteSize {
        width: 21.0,
        height: 31.0,
        scale: 4.0,
    };

    let sprite_sheet = rl
        .load_texture(&thread, "./assets_image/mado_spritesheet.png")
        .unwrap();

    let mut last_mouse_pos = Vector2::new(0.0, 0.0);

    let audio = raylib::core::audio::RaylibAudio::init_audio_device()
        .expect("No hay audio en esta cuestión");
    let dame = audio
        .new_sound("./assets_audio/yume_nikki_dame.mp3")
        .expect("No se pudo cargar efecto 'dame'");
    let muri = audio
        .new_sound("./assets_audio/yume_nikki_muri.mp3")
        .expect("No se pudo cargar efecto 'muri'");

    let mut current_state = PetState::Still(0.0, 1.0);

    let mut animation_frames_counter = 0;

    //PROBABLEMENTE ESTO SEA CAMBIADO EN UN FUTURO
    rl.set_window_size(
        (mado_sprite_size.width * mado_sprite_size.scale) as i32,
        (mado_sprite_size.height * mado_sprite_size.scale) as i32,
    );

    rl.set_window_position(
        (scr_wd / 2) - ((mado_sprite_size.width * mado_sprite_size.scale) as i32) / 2,
        scr_hg / 2 - ((mado_sprite_size.height * mado_sprite_size.scale) as i32) / 2,
    );

    let mut rand_screen_pos: Vector2 = rl.get_window_position();

    //println!("wd: {}, hg: {}", scr_wd, scr_hg);

    let si_esta_variable_no_existe_el_programa_explota: bool = false;

    let mut frame_counter: i32 = 0; //contaremos fps con esta cosa, y así medir tiempo, porque no se me ocurre una mejor manera, ok?
    rl.set_target_fps(60);
    while !rl.window_should_close()
        && (si_esta_variable_no_existe_el_programa_explota
            == si_esta_variable_no_existe_el_programa_explota)
    {
        let current_mouse_pos = rl.get_mouse_position();
        let delta_mouse_pos = current_mouse_pos - last_mouse_pos;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let random: i32 = rl.get_random_value(0..=1);

            if random == 1 {
                dame.play();
            } else {
                muri.play();
            }

            current_state = PetState::Grabbed(0.0, 1.0);
        }

        //+--------------------------------------------+
        //|          LÓGICA DE MOVIMIENTO AUTÓNOMO     |
        //+--------------------------------------------+

        match current_state {
            Still(x, y) => {
                //Last_mouse_pos debe de ser actualizada siempre y cuando no estemos en el caso de grabbing.
                //porque es lo que nos permite agarrar a Mado.
                last_mouse_pos = current_mouse_pos;

                let rand_waiting: i32 = rl.get_random_value(180..=300);

                if frame_counter > rand_waiting {
                    frame_counter = 0;

                    rand_screen_pos = {
                        let rand_x: i32 = rl.get_random_value(
                            0..=(scr_wd - (mado_sprite_size.width * mado_sprite_size.scale) as i32),
                        );
                        let rand_y: i32 = rl.get_random_value(
                            0..=scr_hg
                                - ((mado_sprite_size.height * mado_sprite_size.scale) as i32),
                        );
                        let vector = Vector2::new(rand_x as f32, rand_y as f32);

                        vector
                    };

                    current_state = PetState::Moving(x, y);
                } else {
                    frame_counter += 1;
                };
            }

            Moving(_, _) => {
                //Last_mouse_pos debe de ser actualizada siempre y cuando no estemos en el caso de grabbing.
                //porque es lo que nos permite agarrar a Mado.
                last_mouse_pos = current_mouse_pos;

                let delta_window_pos = rl.get_window_position().move_towards_orthogonal(
                    rand_screen_pos,
                    4.0,
                    &mut current_state,
                );
                rl.set_window_position(delta_window_pos.x as i32, delta_window_pos.y as i32);
            }

            Grabbed(_, _) => {
                if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    let window_pos = rl.get_window_position();
                    rl.set_window_position(
                        (window_pos.x + delta_mouse_pos.x) as i32,
                        (window_pos.y + delta_mouse_pos.y) as i32,
                    );
                } else {
                    current_state = PetState::Still(0.0, 1.0);
                };
            }
        }

        //println!("window pos: {:?}, rand_window_pos: {:?}, mouse_click{}", rl.get_window_position(), rand_screen_pos, rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT));

        //+-----------------------------------------+
        //|          LÓGICA DE DIBUJADO             |
        //+-----------------------------------------+

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(color::Color::BLANK);

        let slice_sprite_coords: (f32, f32) = match current_state {
            Moving(1.0, 0.0) => {
                let mut a = 0.0;

                if animation_frames_counter < 7 {
                    a = 0.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 14 {
                    a = 21.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 21 {
                    a = 42.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 28 {
                    a = 21.0;
                    animation_frames_counter += 1;
                } else {
                    animation_frames_counter = 0;
                }
                (a, 31.0)
            }

            Moving(-1.0, 0.0) => {
                let mut a = 0.0;

                if animation_frames_counter < 7 {
                    a = 0.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 14 {
                    a = 21.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 21 {
                    a = 42.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 28 {
                    a = 21.0;
                    animation_frames_counter += 1;
                } else {
                    animation_frames_counter = 0;
                }
                (a, 93.0)
            }

            Moving(0.0, 1.0) => {
                let mut a = 0.0;

                if animation_frames_counter < 7 {
                    a = 0.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 14 {
                    a = 21.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 21 {
                    a = 42.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 28 {
                    a = 21.0;
                    animation_frames_counter += 1;
                } else {
                    animation_frames_counter = 0;
                }

                (a, 61.0)
            }

            Moving(0.0, -1.0) => {
                let mut a = 0.0;

                if animation_frames_counter < 7 {
                    a = 0.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 14 {
                    a = 21.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 21 {
                    a = 42.0;
                    animation_frames_counter += 1;
                } else if animation_frames_counter < 28 {
                    a = 21.0;
                    animation_frames_counter += 1;
                } else {
                    animation_frames_counter = 0;
                }

                (a, 0.0)
            }

            Still(1.0, 0.0) => (21.0, 31.0),
            Still(-1.0, 0.0) => (21.0, 93.0),
            Still(0.0, 1.0) => (21.0, 62.0),
            Still(0.0, -1.0) => (21.0, 0.0),
            _ => (21.0, 62.0),
        };

        //NECESITO PENSAR EN ALGO MEJOR.

        //println!("{:?}, frame: {:?}", current_state, animation_frames_counter);

        let sprite_mado_slice = Rectangle::new(
            slice_sprite_coords.0,
            slice_sprite_coords.1,
            mado_sprite_size.width,
            mado_sprite_size.height,
        );

        let scaled_mado = Rectangle::new(
            0.0,
            0.0,
            mado_sprite_size.width * mado_sprite_size.scale,
            mado_sprite_size.height * mado_sprite_size.scale,
        );

        d.draw_texture_pro(
            &sprite_sheet,
            sprite_mado_slice,
            scaled_mado,
            (0.0, 0.0),
            0.0,
            Color::WHITE,
        );
    }
}
