use device_query::{DeviceQuery, DeviceState};
use libraries::types::{Action::*, Effect::*, PetState};
use raylib::ffi::Rectangle;

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

    let mado_pet = PetState {
        direction: (0.0, 1.0),
        action: Still,
        current_effect: ChairSpin,
        width: 21.0,
        height: 31.0,
        scale: 4.0,
    };

    let mut position_on_screen = Vector2::from((
        ((scr_wd as f32 / 2.0 - (mado_pet.width * mado_pet.scale)) / 2.0),
        ((scr_hg as f32 / 2.0 - (mado_pet.height * mado_pet.scale)) / 2.0),
    ));
    let mut animation_frames_counter = 0;
    let mut speed: (f32, f32) = (4.0, 4.0);
    let raw_passthrough_flag = ConfigFlags::FLAG_WINDOW_MOUSE_PASSTHROUGH as u32;

    rl.set_target_fps(60);
    let device_state = DeviceState::new();

    while !rl.window_should_close() {
        let pet_area = Rectangle::new(
            position_on_screen.x,
            position_on_screen.y,
            mado_pet.width * mado_pet.scale,
            mado_pet.height * mado_pet.height,
        );

        let mouse_pos = device_state.get_mouse().coords;

        if is_mouse_on_bounded_area(mouse_pos, pet_area) {
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

        if (position_on_screen.x >= scr_wd as f32 - 64.0) || position_on_screen.x <= -12.0 {
            speed.0 *= -1.0;
        }

        if (position_on_screen.y >= scr_hg as f32 - 108.0) || (position_on_screen.y <= -12.0) {
            speed.1 *= -1.0;
        };

        position_on_screen = (
            (position_on_screen.x + speed.0),
            (position_on_screen.y + speed.1),
        )
            .into();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(color::Color::BLANK);

        animation_frames_counter += 1;
        let sprite_index: usize = (animation_frames_counter / 7) % 4;
        let x_cut = 63.0;
        let y_cut = [0.0, 31.0, 62.0, 93.0][sprite_index];

        let sprite_mado_slice = Rectangle::new(x_cut, y_cut, mado_pet.width, mado_pet.height);

        let scaled_mado = Rectangle::new(
            position_on_screen.x,
            position_on_screen.y,
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

fn is_mouse_on_bounded_area(mouse_pos: (i32, i32), area: Rectangle) -> bool {
    area.x <= mouse_pos.0 as f32
        && mouse_pos.0 as f32 <= area.x + area.width
        && area.y <= mouse_pos.1 as f32
        && mouse_pos.1 as f32 <= area.y + area.height
}
