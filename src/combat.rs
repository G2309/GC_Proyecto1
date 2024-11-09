use crate::Color;
use crate::render_text;
use crate::FrameBuffer;
use crate::Texture;
use crate::Party;
use crate::EnemiesData;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

enum Action {
    Attack,
    Defend,
    Spell,
    Pass
}

pub struct CombatState {
    current_turn: usize,
    iteration: usize,
    is_player_turn: bool,
    action: Action,
}

pub fn render_combat_ui(
    framebuffer: &mut FrameBuffer,
    party: &mut Party,
    enemiesdata: &EnemiesData,
    background_texture: &Texture,
    combat_state: &mut CombatState,
    ) {
    framebuffer.draw_texture(&background_texture, 0, 0); 

    if !combat_state.is_player_turn {
        enemy_action(combat_state, party);
    }

    // draw enemy
    for (index, enemies) in enemiesdata.enemies.iter().enumerate() {
        framebuffer.draw2D_texture(&enemies.enemy_texture, 350, 100);

        framebuffer.draw_rect(285, 25, 450, 65, Color::new(0, 0, 0)); 
        framebuffer.draw_rect_outline(285, 25, 450, 65, Color::new(255, 255, 255));
        
        let enemy_hp_text = format!("Enemy HP: {} / {}", enemies.hp, enemies.max_hp);
        let enemy_name = format!("{}", enemies.name);
        
        // HP
        framebuffer.draw_rect(300, 60, enemies.hp as usize, 20, Color::new(255, 0, 0)); // Barra de HP
        framebuffer.draw_rect_outline(300, 60, enemies.max_hp as usize, 20, Color::new(255, 255, 255)); // Contorno de la barra de HP

        // Renderizar el texto de HP
        render_text(framebuffer, &enemy_hp_text, 550, 75, Color::new(255, 255, 255));
        render_text(framebuffer, &enemy_name, 485, 45, Color::new(255, 255, 255));

        let turn_color = if !combat_state.is_player_turn && combat_state.current_turn == index {
            Color::new(0, 255, 0)
        } else {
            Color::new(255, 0, 0)
        };

        // enemy turn
        framebuffer.draw_rect(285, 25, 10, 10, turn_color);
        framebuffer.draw_rect_outline(285, 25, 10, 10, Color::new(255, 255, 255));

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

        let turn_color = if combat_state.is_player_turn && combat_state.current_turn == index {
            Color::new(0, 255, 0)
        } else {
            Color::new(255, 0, 0)
        };
        framebuffer.draw_rect(hp_bar_x + (325 * index), 764, 10, 10, turn_color);
        framebuffer.draw_rect_outline(hp_bar_x + (325 * index), 764, 10, 10, Color::new(255, 255, 255));
    }
    
    let actions = ["Attack  [a]", "Defend   [d]", "Spell    [s]", "Pass    [f]"];
    let mut y_pos = 170;

    framebuffer.draw_rect(26, 125, 275, 150, Color::new(0, 0, 0));
    framebuffer.draw_rect_outline(26, 125, 275, 150, Color::new(255, 255, 255));
    
    render_text(framebuffer,"What would you do?", 30, 145, Color::new(255, 255, 255));

    for action in actions.iter() {
        render_text(framebuffer, action, 30, y_pos, Color::new(255,255,255));
        y_pos += 25;
    } 
}

pub fn player_attack(combat_state: &mut CombatState, enemiesdata: &mut EnemiesData) {
    let mut rng = rand::thread_rng();
    let is_critical = rng.gen_bool(0.2);
    let attack_percent = rng.gen_range(0..17);

    if let Some(enemy) = enemiesdata.enemies.get_mut(0) {
        let damage = if is_critical {
            26 + attack_percent 
        } else {
            10 + attack_percent 
        };

        enemy.hp = enemy.hp.saturating_sub(damage);
    }
    sleep(Duration::from_millis(250));
    combat_state.next_turn(is_critical, 3);
}

pub fn enemy_action(combat_state: &mut CombatState, party: &mut Party) {
    let mut rng = rand::thread_rng();
    let action_is_spell = rng.gen_bool(0.5);
    
    let target_index = rng.gen_range(0..party.players_data.len());
    let target = &mut party.players_data[target_index];
    let damage = if action_is_spell {
        if target.weakness.contains(&"magic".to_string()) {
            35 
        } else {
            17
        }
    } else {
        14
    };

    target.hp = target.hp.saturating_sub(damage);
    combat_state.next_turn(false, 3);
    sleep(Duration::from_millis(150));
}

impl CombatState {
    pub fn new() -> Self {
        Self {
            current_turn: 0,
            iteration: 0,
            is_player_turn: true,
            action: Action::Pass,
        }
    }

    pub fn next_turn(&mut self, is_critical: bool, party_size: usize) {
        if is_critical {
            return; 
        }

        let mut rng = rand::thread_rng();

        if self.is_player_turn {
            let mut next_turn;
            loop {
                next_turn = rng.gen_range(0..party_size); 
                if next_turn != self.current_turn {
                    break;
                }
            }
            self.current_turn = next_turn;
            self.iteration += 1;

            if self.iteration >= 3 {
                self.is_player_turn = false;
                self.current_turn = 0;
                self.iteration = 0; 
            }
        } else {
            self.is_player_turn = true; 
            self.current_turn = rng.gen_range(0..party_size); 
        }
    }

}
