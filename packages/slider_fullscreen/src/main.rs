use device_query::{DeviceQuery, DeviceState};
use libraries::{
    functions::is_mouse_on_any_pet,
    types::{Action::*, Effect::*, PetStateEx},
};
use raylib::ffi::KeyboardKey::KEY_N;

fn main() {
    use raylib::prelude::*;

    let (mut rl, thread) = raylib::init()
        .title("The game: you lost!")
        .topmost()
        .undecorated()
        .transparent()
        .mouse_passthrough()
        .build();

    let (scr_wd, scr_hg) = {
        let monitor = get_current_monitor();
        let width = get_monitor_width(monitor);
        let height = get_monitor_height(monitor);

        (width, height)
    };

    rl.set_window_size(scr_wd + 1, scr_hg);
    rl.set_window_position(0, 0);

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

    let mado_instance = PetStateEx {
        direction: (0.0, 1.0),
        action: Still,
        current_effect: ChairSpin,
        width: 21.0,
        height: 31.0,
        x_position: (scr_wd as f32 / 2.0 - (21.0 * 4.0)),
        y_position: (scr_hg as f32 / 2.0 - (31.0 * 4.0)),
        scale: 4.0,
        speed: (4.0, 4.0),
    };

    let mut instances_vec: Vec<PetStateEx> = Vec::new();
    instances_vec.push(mado_instance);

    let mut animation_frames_counter = 0;
    let raw_passthrough_flag = ConfigFlags::FLAG_WINDOW_MOUSE_PASSTHROUGH as u32;

    rl.set_target_fps(60);
    let device_state = DeviceState::new();

    while !rl.window_should_close() {
        if rl.is_key_pressed(KEY_N) {
            instances_vec.push(PetStateEx {
                direction: (0.0, 1.0),
                action: Still,
                current_effect: ChairSpin,
                width: 21.0,
                height: 31.0,
                x_position: (scr_wd as f32 / 2.0 - (21.0 * 4.0)),
                y_position: (scr_hg as f32 / 2.0 - (31.0 * 4.0)),
                scale: 4.0,
                speed: (4.0, 4.0),
            });
        }

        let mouse_pos = device_state.get_mouse().coords;

        if is_mouse_on_any_pet(mouse_pos, &instances_vec) {
            unsafe {
                let state_flag: WindowState = std::mem::transmute(raw_passthrough_flag);
                rl.clear_window_state(state_flag);
            }

            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                let random: i32 = rl.get_random_value(0..=1);
                if random == 1 {
                    dame.play();
                } else {
                    muri.play();
                }
            };
        } else {
            unsafe {
                let state_flag: WindowState = std::mem::transmute(raw_passthrough_flag);
                rl.set_window_state(state_flag);
            }
        }

        for mado_pet in instances_vec.iter_mut() {
            if (mado_pet.x_position >= scr_wd as f32 - 64.0) || mado_pet.x_position <= -12.0 {
                mado_pet.speed.0 *= -1.0;
            }

            if (mado_pet.y_position >= scr_hg as f32 - 108.0) || (mado_pet.y_position <= -12.0) {
                mado_pet.speed.1 *= -1.0;
            };

            (mado_pet.x_position, mado_pet.y_position) = (
                (mado_pet.x_position + mado_pet.speed.0),
                (mado_pet.y_position + mado_pet.speed.1),
            );
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(color::Color::BLANK);

        animation_frames_counter += 1;

        for mado_pet in instances_vec.iter_mut() {
            let sprite_index: usize = (animation_frames_counter / 7) % 4;
            let x_cut = 63.0;
            let y_cut = [0.0, 31.0, 62.0, 93.0][sprite_index];

            let sprite_mado_slice = Rectangle::new(x_cut, y_cut, mado_pet.width, mado_pet.height);

            d.draw_texture_pro(
                &sprite_sheet,
                sprite_mado_slice,
                Rectangle::new(
                    mado_pet.x_position,
                    mado_pet.y_position,
                    mado_pet.width * mado_pet.scale,
                    mado_pet.height * mado_pet.scale,
                ),
                (0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
    }
}
