use crate::player::Player;
use minifb::Key;

pub fn process_events(player: &mut Player, window: &minifb::Window) {
    let move_speed = 0.1;
    let rotate_speed = 0.1;
    
    if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
        player.move_forward(move_speed);
    }
    if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
        player.move_forward(-move_speed);
    }
    if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
        player.rotate(-rotate_speed);
    }
    if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
        player.rotate(rotate_speed);
    }
}
