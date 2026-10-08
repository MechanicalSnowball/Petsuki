use core::f32;

use libraries::functions::*;
use libraries::types::{Action::*, Effect::*, PetState};

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

    let mut mado_pet = PetState {
        direction: (0.0, 1.0),
        action: Still,
        current_effect: NoEffect,
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

    let mut animation_frames_counter = 0;

    //PROBABLEMENTE ESTO SEA CAMBIADO EN UN FUTURO
    rl.set_window_size(
        (mado_pet.width * mado_pet.scale) as i32,
        (mado_pet.height * mado_pet.scale) as i32,
    );

    rl.set_window_position(
        (scr_wd / 2) - ((mado_pet.width * mado_pet.scale) as i32) / 2,
        scr_hg / 2 - ((mado_pet.height * mado_pet.scale) as i32) / 2,
    );

    let mut rand_screen_pos: Vector2 = rl.get_window_position();

    //println!("wd: {}, hg: {}", scr_wd, scr_hg);
    let mut frame_counter: i32 = 0; //contaremos fps con esta cosa, y así medir tiempo, porque no se me ocurre una mejor manera, ok?
    let mut rand_waiting: i32 = rl.get_random_value(180..=300);
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let current_mouse_pos = rl.get_mouse_position();
        let delta_mouse_pos = current_mouse_pos - last_mouse_pos;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let random: i32 = rl.get_random_value(0..=1);

            if random == 1 {
                dame.play();
            } else {
                muri.play();
            }

            (mado_pet.action, mado_pet.direction) = (Grabbed, (0.0, 1.0));
        }

        //+--------------------------------------------+
        //|          LÓGICA DE MOVIMIENTO AUTÓNOMO     |
        //+--------------------------------------------+

        match (&mado_pet.action, &mado_pet.direction) {
            (Still, (x, y)) => {
                //Last_mouse_pos debe de ser actualizada siempre y cuando no estemos en el caso de grabbing.
                //porque es lo que nos permite agarrar a Mado.
                last_mouse_pos = current_mouse_pos;

                if frame_counter > rand_waiting {
                    rand_waiting = rl.get_random_value(180..=300);
                    frame_counter = 0;

                    rand_screen_pos = {
                        let rand_x: i32 = rl.get_random_value(
                            0..=(scr_wd - (mado_pet.width * mado_pet.scale) as i32),
                        );
                        let rand_y: i32 = rl.get_random_value(
                            0..=scr_hg - ((mado_pet.height * mado_pet.scale) as i32),
                        );

                        Vector2::new(rand_x as f32, rand_y as f32)
                    };

                    (mado_pet.action, mado_pet.direction) = (Moving, (*x, *y));
                } else {
                    frame_counter += 1;
                };
            }

            (Moving, (_, _)) => {
                //Last_mouse_pos debe de ser actualizada siempre y cuando no estemos en el caso de grabbing.
                //porque es lo que nos permite agarrar a Mado.
                last_mouse_pos = current_mouse_pos;

                let delta_window_pos = rl.get_window_position().move_towards_orthogonal(
                    rand_screen_pos,
                    4.0,
                    &mut mado_pet,
                );
                rl.set_window_position(delta_window_pos.x as i32, delta_window_pos.y as i32);
            }

            (Grabbed, (_, _)) => {
                if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    let window_pos = rl.get_window_position();
                    rl.set_window_position(
                        (window_pos.x + delta_mouse_pos.x) as i32,
                        (window_pos.y + delta_mouse_pos.y) as i32,
                    );
                } else {
                    (mado_pet.action, mado_pet.direction) = (Still, (0.0, 1.0));
                };
            }
        }

        //println!("window pos: {:?}, rand_window_pos: {:?}, mouse_click{}", rl.get_window_position(), rand_screen_pos, rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT));

        //+-----------------------------------------+
        //|          LÓGICA DE DIBUJADO             |
        //+-----------------------------------------+

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(color::Color::BLANK);

        let y_cut: f32 = match mado_pet.direction {
            (1.0, 0.0) => 31.0,
            (-1.0, 0.0) => 93.0,
            (0.0, 1.0) => 62.0,
            _ => 0.0,
        };

        let x_cut: f32 = match mado_pet.action {
            Moving => {
                animation_frames_counter += 1;
                let sprite_index: usize = (animation_frames_counter / 7) % 4;

                [0.0, 21.0, 42.0, 21.0][sprite_index]
            }

            _ => {
                animation_frames_counter = 0;
                21.0
            }
        };

        //println!("{:?}, frame: {:?}", current_state, animation_frames_counter);

        let sprite_mado_slice = Rectangle::new(x_cut, y_cut, mado_pet.width, mado_pet.height);

        let scaled_mado = Rectangle::new(
            0.0,
            0.0,
            mado_pet.width * mado_pet.scale,
            mado_pet.height * mado_pet.scale,
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
