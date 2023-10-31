use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub type Keys = [bool; 16];
type Pause = bool;

pub fn get_processed_input(input: &WinitInputHelper) -> (Keys, Pause) {
    let key_1 = input.key_held(VirtualKeyCode::Key1);
    let key_2 = input.key_held(VirtualKeyCode::Key2);
    let key_3 = input.key_held(VirtualKeyCode::Key3);
    let key_c = input.key_held(VirtualKeyCode::Key4);

    let key_4 = input.key_held(VirtualKeyCode::Q);
    let key_5 = input.key_held(VirtualKeyCode::W);
    let key_6 = input.key_held(VirtualKeyCode::E);
    let key_d = input.key_held(VirtualKeyCode::R);

    let key_7 = input.key_held(VirtualKeyCode::A);
    let key_8 = input.key_held(VirtualKeyCode::S);
    let key_9 = input.key_held(VirtualKeyCode::D);
    let key_e = input.key_held(VirtualKeyCode::F);

    let key_a = input.key_held(VirtualKeyCode::Z);
    let key_0 = input.key_held(VirtualKeyCode::X);
    let key_b = input.key_held(VirtualKeyCode::C);
    let key_f = input.key_held(VirtualKeyCode::V);

    let toggle_pause = input.key_pressed(VirtualKeyCode::P);

    let keys = [
        key_1, key_2, key_3, key_c, key_4, key_5, key_6, key_d, key_7, key_8, key_9, key_e, key_a,
        key_0, key_b, key_f,
    ];

    (keys, toggle_pause)
}
