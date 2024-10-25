use crate::Color;
use crate::render_text;
use crate::FrameBuffer;
use crate::Texture;
use crate::Party;
use crate::EnemiesData;

pub fn render_combat_ui(
    framebuffer: &mut FrameBuffer,
    party: &Party,
    enemiesdata: &EnemiesData,
    background_texture: &Texture,
    ) {
    framebuffer.draw_texture(&background_texture, 0, 0); 

    // draw enemy
    for (index, enemies) in enemiesdata.enemies.iter().enumerate() {
        framebuffer.draw2D_texture(&enemies.enemy_texture, 350, 100);

        framebuffer.draw_rect(285, 25, 450, 65, Color::new(0, 0, 0)); 
        
        let enemy_hp_text = format!("Enemy HP: {} / {}", enemies.hp, enemies.max_hp);
        let enemy_name = format!("{}", enemies.name);
        
        // HP
        framebuffer.draw_rect(300, 60, enemies.hp as usize, 20, Color::new(255, 0, 0)); // Barra de HP
        framebuffer.draw_rect_outline(300, 60, enemies.max_hp as usize, 20, Color::new(255, 255, 255)); // Contorno de la barra de HP

        // Renderizar el texto de HP
        render_text(framebuffer, &enemy_hp_text, 550, 75, Color::new(255, 255, 255));
        render_text(framebuffer, &enemy_name, 485, 45, Color::new(255, 255, 255));

    }

    let bar_height = 20;
    let hp_bar_x = 26; 
    let mp_bar_x = 26;

    for (index, player_data) in party.players_data.iter().enumerate() {
        let hp_text = format!("HP: {} / {}", player_data.hp, player_data.max_hp);
        let mp_text = format!("MP: {} / {}", player_data.mp, player_data.max_mp);
        let player_name = format!("{}", player_data.name);

        // Render HP y MP de cada jugador
        framebuffer.draw_rect(hp_bar_x + (325 * index), 575, 275, 200, Color::new(0, 0, 0));  // Fondo
        framebuffer.draw_rect_outline(hp_bar_x + (325 * index), 575, 275, 200, Color::new(255, 255, 255));  // Contorno
        
        // HP Bar
        framebuffer.draw_rect(hp_bar_x + (325 * index), 585, player_data.hp as usize, bar_height, Color::new(255, 0, 0));
        framebuffer.draw_rect_outline(hp_bar_x + (325 * index), 585, player_data.max_hp as usize, bar_height, Color::new(255, 255, 255));  // Contorno
        render_text(framebuffer, &hp_text, 150 + (325 * index), 600, Color::new(255, 255, 255));
        
        // MP Bar
        framebuffer.draw_rect(mp_bar_x + (325 * index), 625, player_data.mp as usize, bar_height, Color::new(0, 0, 255));
        framebuffer.draw_rect_outline(mp_bar_x + (325 * index), 625, player_data.max_mp as usize, bar_height, Color::new(255, 255, 255));  // Contorno
        render_text(framebuffer, &mp_text, 150 + (325 * index), 640, Color::new(255, 255, 255));

        // Textura del jugador
        framebuffer.draw2D_texture(&player_data.texture, 125 + (325 * index), 655);
        render_text(framebuffer, &player_name, 140 + (325 * index), 765, Color::new(255, 255, 255));
    }

}

