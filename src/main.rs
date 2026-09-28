use core::f32;

mod functions;
mod types;
use functions::*;
use types::*;

use crate::types::PetState::{Grabbed, Moving, Still};

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

        let y_cut: f32 = match current_state {
            Moving(1.0, 0.0) | Still(1.0, 0.0) => 31.0,
            Moving(-1.0, 0.0) | Still(-1.0, 0.0) => 93.0,
            Moving(0.0, 1.0) | Still(0.0, 1.0) => 62.0,
            _ => 0.0,
        };

        let slice_sprite_coords: (f32, f32) = match current_state {
            Moving(_, _) => {
                animation_frames_counter += 1;
                let x_cut = match animation_frames_counter {
                    1..=7 => 0.0,
                    8..=14 => 21.0,
                    15..=21 => 42.0,
                    22..=27 => 21.0,
                    _ => {
                        animation_frames_counter = 0;
                        21.0
                    }
                };

                (x_cut, y_cut)
            }

            Still(_, _) => (21.0, y_cut),

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
