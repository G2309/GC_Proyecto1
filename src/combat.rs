use crate::Color;
use crate::render_text;
use crate::FrameBuffer;
use crate::Texture;
use crate::Party;
use crate::EnemiesData;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub struct CombatState {
    pub current_turn: usize,
    is_player_turn: bool,
    pub is_spell_active: bool,
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
    // Spell menu
    if combat_state.is_spell_active {
        if let Some(player) = party.players_data.get(combat_state.current_turn) {
            framebuffer.draw_rect(650, 125, 200, (player.spells.len() * 30) as usize, Color::new(0, 0, 0));
            framebuffer.draw_rect_outline(650, 125, 200, (player.spells.len() * 30) as usize, Color::new(255, 255, 255));
            let spell_keys = ['I', 'O', 'P'];
            for (index, spell) in player.spells.iter().enumerate() {
                if let Some(&key) = spell_keys.get(index) {
                let spell_text = format!("[{}] {}", key, spell);
                render_text(framebuffer, &spell_text, 660, 145 + (index * 30), Color::new(255, 255, 255), None);
            }
            }
        }
    }
    // draw enemy
    for (index, enemies) in enemiesdata.enemies.iter().enumerate() {
        framebuffer.draw2D_texture(&enemies.enemy_texture, 350, 100);

        framebuffer.draw_rect(285, 25, 450, 65, Color::new(0, 0, 0)); 
        framebuffer.draw_rect_outline(285, 25, 450, 65, Color::new(255, 255, 255));
        
        let enemy_hp_text = format!("Enemy HP: {} / {}", enemies.hp.max(0), enemies.max_hp);
        let enemy_name = format!("{}", enemies.name);
        
        // HP
        framebuffer.draw_rect(300, 60, enemies.hp as usize, 20, Color::new(255, 0, 0)); // Barra de HP
        framebuffer.draw_rect_outline(300, 60, enemies.max_hp as usize, 20, Color::new(255, 255, 255)); // Contorno de la barra de HP

        // Renderizar el texto de HP
        render_text(framebuffer, &enemy_hp_text, 550, 75, Color::new(255, 255, 255), None);
        render_text(framebuffer, &enemy_name, 485, 45, Color::new(255, 255, 255), None);

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
        let hp_text = format!("HP: {} / {}", player_data.hp.max(0), player_data.max_hp);
        let mp_text = format!("MP: {} / {}", player_data.mp.max(0), player_data.max_mp);
        let player_name = format!("{}", player_data.name);

        // Render HP y MP de cada jugador
        framebuffer.draw_rect(hp_bar_x + (325 * index), 575, 275, 200, Color::new(0, 0, 0));  // Fondo
        framebuffer.draw_rect_outline(hp_bar_x + (325 * index), 575, 275, 200, Color::new(255, 255, 255));  // Contorno
        
        // HP Bar
        framebuffer.draw_rect(hp_bar_x + (325 * index), 585, player_data.hp as usize, bar_height, Color::new(255, 0, 0));
        framebuffer.draw_rect_outline(hp_bar_x + (325 * index), 585, player_data.max_hp as usize, bar_height, Color::new(255, 255, 255));  // Contorno
        render_text(framebuffer, &hp_text, 150 + (325 * index), 600, Color::new(255, 255, 255), None);
        
        // MP Bar
        framebuffer.draw_rect(mp_bar_x + (325 * index), 625, player_data.mp as usize, bar_height, Color::new(0, 0, 255));
        framebuffer.draw_rect_outline(mp_bar_x + (325 * index), 625, player_data.max_mp as usize, bar_height, Color::new(255, 255, 255));  // Contorno
        render_text(framebuffer, &mp_text, 150 + (325 * index), 640, Color::new(255, 255, 255), None);

        // Textura del jugador
        framebuffer.draw2D_texture(&player_data.texture, 125 + (325 * index), 655);
        render_text(framebuffer, &player_name, 140 + (325 * index), 765, Color::new(255, 255, 255), None);

        let turn_color = if combat_state.is_player_turn && combat_state.current_turn == index {
            Color::new(0, 255, 0)
        } else {
            Color::new(255, 0, 0)
        };
        framebuffer.draw_rect(hp_bar_x + (325 * index), 764, 10, 10, turn_color);
        framebuffer.draw_rect_outline(hp_bar_x + (325 * index), 764, 10, 10, Color::new(255, 255, 255));
    }
    
    let actions = [" Attack - - - - - - - [a]", " Defend - - - - - - - [d]", " Spell  - - - - - - - [s]", " Pass   - - - - - - - [f]"];
    let mut y_pos = 170;

    framebuffer.draw_rect(26, 125, 275, 150, Color::new(0, 0, 0));
    framebuffer.draw_rect_outline(26, 125, 275, 150, Color::new(255, 255, 255));
    
    render_text(framebuffer," What would you do?", 30, 145, Color::new(255, 255, 255), None);

    for action in actions.iter() {
        render_text(framebuffer, action, 30, y_pos, Color::new(255,255,255), None);
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
}

pub fn player_defend(combat_state: &mut CombatState, party: &mut Party) {
    if let Some(player) = party.players_data.get_mut(combat_state.current_turn) {
        player.is_defending = true;  
    }
    sleep(Duration::from_millis(250));
}

pub fn enemy_action(combat_state: &mut CombatState, party: &mut Party) {
    let mut rng = rand::thread_rng();
    let action_is_spell = rng.gen_bool(0.5);

    let mut alive_players: Vec<_> = party.players_data
        .iter_mut()
        .filter(|player| player.hp > 0)
        .collect();

    if alive_players.is_empty() {
        return;
    }

    let target_index = rng.gen_range(0..alive_players.len());
    let target = &mut alive_players[target_index];

    let base_damage = if action_is_spell {
        if target.weakness.contains(&"magic".to_string()) {
            35
        } else {
            17
        }
    } else {
        14
    };

    let actual_damage = if target.is_defending {
        (base_damage as f32 * 0.5) as i32
    } else {
        base_damage
    };

    target.hp = target.hp.saturating_sub(actual_damage);

    for player in &mut party.players_data {
        player.is_defending = false;
    }

    combat_state.is_player_turn = true;
    sleep(Duration::from_millis(150));
}

pub fn player_spell(
    combat_state: &mut CombatState,
    enemiesdata: &mut EnemiesData,
    spell_name: &str,
    party: &mut Party,
) {
    let mut rng = rand::thread_rng();
    let is_critical = rng.gen_bool(0.15);

    if let Some(player) = party.players_data.get_mut(combat_state.current_turn) {
        // Si el hechizo es "dia"
        if spell_name == "dia" {
            if player.mp >= 10 {
                player.mp -= 10;

                // Buscar al miembro del grupo con menor HP
                if let Some(target) = party.players_data.iter_mut()
                    .filter(|p| p.hp > 0)  // Solo jugadores con HP > 0
                    .min_by_key(|p| p.hp)  // Encontrar al jugador con menor HP
                {
                    // Curar 20 HP (ajustar según lo que consideres adecuado)
                    target.hp = (target.hp + 20).min(target.max_hp);  // No superar el máximo HP
                    sleep(Duration::from_millis(250));  // Retraso de acción
                }
            }
        }
        // Si el hechizo no es "dia", atacar al enemigo como antes
        else if player.spells.contains(&spell_name.to_string()) && player.mp >= 10 {
            player.mp -= 10;
            if let Some(enemy) = enemiesdata.enemies.get_mut(0) {
                let base_damage = if enemy.weakness.contains(&spell_name.to_string()) {
                    30  
                } else {
                    15 
                };
                let damage = if is_critical { base_damage * 2 } else { base_damage };
                enemy.hp = enemy.hp.saturating_sub(damage);
                sleep(Duration::from_millis(250));
            }
        }
    }
}


impl CombatState {
    pub fn new() -> Self {
        Self {
            current_turn: 0,
            is_player_turn: true,
            is_spell_active: false,
        }
    }

    pub fn activate_spell(&mut self) {
        self.is_spell_active = true;
    }

    pub fn next_turn(&mut self, is_critical: bool, party: &Party, enemiesdata: &EnemiesData) {
        if is_critical {
            return; 
        }
        sleep(Duration::from_millis(150));

        loop {
            self.current_turn += 1;
            if self.is_player_turn {
                if self.current_turn >= party.players_data.len() {
                    self.is_player_turn = false;
                    self.current_turn = 0;
                }
                if let Some(player) = party.players_data.get(self.current_turn) {
                    if player.hp > 0 {
                        break;
                    }
                }
            } else {
                if let Some(enemy) = enemiesdata.enemies.get(0) {
                    if enemy.hp > 0 {
                        break;
                    } else {
                        self.is_player_turn = true;
                        self.current_turn = 0;
                    }
                }
            }
        }
        self.is_spell_active = false;
    }
}
