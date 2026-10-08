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

    let sprite_sheet = rl
        .load_texture(&thread, "./assets_image/mado_spritesheet.png")
        .unwrap();
    let audio = raylib::core::audio::RaylibAudio::init_audio_device()
        .expect("No hay audio en esta cuestión");
    let dame = audio
        .new_sound("./assets_audio/yume_nikki_dame.mp3")
        .expect("No se pudo cargar efecto 'dame'");
    let muri = audio
        .new_sound("./assets_audio/yume_nikki_muri.mp3")
        .expect("No se pudo cargar efecto 'muri'");

    let mado_pet = PetState {
        direction: (0.0, 1.0),
        action: Still,
        current_effect: ChairSpin,
        width: 21.0,
        height: 31.0,
        scale: 4.0,
    };

    rl.set_window_size(
        (mado_pet.width * mado_pet.scale) as i32,
        (mado_pet.height * mado_pet.scale) as i32,
    );

    rl.set_window_position(
        (scr_wd / 2) - ((mado_pet.width * mado_pet.scale) as i32) / 2,
        scr_hg / 2 - ((mado_pet.height * mado_pet.scale) as i32) / 2,
    );

    let mut animation_frames_counter = 0;
    let mut speed: (f32, f32) = (4.0, 4.0);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let random: i32 = rl.get_random_value(0..=1);
            if random == 1 {
                dame.play();
            } else {
                muri.play();
            }
        }

        let window_pos = rl.get_window_position();

        if (window_pos.x >= scr_wd as f32 - 64.0) || window_pos.x <= -12.0 {
            speed.0 *= -1.0;
        } else if (window_pos.y >= scr_hg as f32 - 108.0) || (window_pos.y <= -12.0) {
            //ese numero mágico es la altura real
            //del sprite de mado
            speed.1 *= -1.0;
        };

        //println!("{}", window_pos.y);
        rl.set_window_position(
            (window_pos.x + speed.0) as i32,
            (window_pos.y + speed.1) as i32,
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(color::Color::BLANK);

        animation_frames_counter += 1;
        let sprite_index: usize = (animation_frames_counter / 7) % 4;
        let x_cut = 63.0;
        let y_cut = [0.0, 31.0, 62.0, 93.0][sprite_index];

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
