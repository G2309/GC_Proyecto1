use crate::Color;
use crate::render_text;
use crate::FrameBuffer;
use crate::Texture;

pub fn render_combat_ui(
    framebuffer: &mut FrameBuffer,
    player_maxhp: i32, 
    player_hp: i32, 
    player_maxmp: i32, 
    player_mp: i32, 
    background_texture: &Texture,
    player_1_texture: &Texture,
    ) {
    framebuffer.draw_texture(&background_texture, 0, 0); 

    let bar_height = 20;
    let hp_bar_x = 25; 
    let mp_bar_x = 25;
    let hp_text = format!("HP: {} / {}", player_hp, player_maxhp);
    let mp_text = format!("MP: {} / {}", player_mp, player_maxmp);

    // player 0

    framebuffer.draw_rect(hp_bar_x, 575, 275, 200, Color::new(0, 0, 0));
    framebuffer.draw_rect_outline(hp_bar_x, 575, 275, 200, Color::new(255, 255, 255));
    
    framebuffer.draw_rect(hp_bar_x, 585, player_maxhp as usize, bar_height, Color::new(50, 50, 50)); // HP Bar fondo
    render_text(framebuffer, &hp_text, 150, 600, Color::new(255,255,255));
    framebuffer.draw_rect(mp_bar_x, 625, player_maxmp as usize, bar_height, Color::new(50, 50, 50)); // MP Bar fondo
    render_text(framebuffer, &mp_text, 150, 640, Color::new(255,255,255));

    framebuffer.draw_rect(hp_bar_x, 585, player_hp as usize, bar_height, Color::new(255, 0, 0)); // HP llena
    framebuffer.draw_rect(mp_bar_x,  625, player_mp as usize, bar_height, Color::new(0, 0, 255)); // MP llena

    framebuffer.draw_rect_outline(hp_bar_x, 585, player_maxhp as usize, bar_height, Color::new(255, 255, 255)); // HP Contorno
    framebuffer.draw_rect_outline(mp_bar_x, 625, player_maxmp as usize, bar_height, Color::new(255, 255, 255)); // MP Contorno
    
    // player 1

    // Textura de los personajes
    framebuffer.draw_texture(&player_1_texture, 125, 655);
    render_text(framebuffer, "Walter", 140, 765, Color::new(255,255,255));

}

