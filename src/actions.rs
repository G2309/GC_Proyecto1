use nalgebra_glm::Vec2;
use crate::player::Player;
use crate::map_loader::load_map;

pub struct Actions;

impl Actions {
    pub fn check_doors(player: &Player, map: &mut Vec<Vec<char>>) {
        let player_pos = Vec2::new(player.pos.x / 50.0, player.pos.y / 50.0);
        let directions = vec![
            Vec2::new(1.0,0.0),
            Vec2::new(-1.0,0.0),
            Vec2::new(0.0,1.0),
            Vec2::new(0.0,-1.0),
            Vec2::new(2.0,0.0),
            Vec2::new(-2.0,0.0),
            Vec2::new(0.0,2.0),
            Vec2::new(0.0,-2.0),
        ];

        for dir in directions {
            let check_pos = player_pos + dir;
            let check_x = check_pos.x as usize;
            let check_y = check_pos.y as usize;
            
            if map[check_y][check_x] == 'd' {
                Actions::open_door(map, check_x, check_y);
            }
        }

    }
    
    fn open_door(map: &mut Vec<Vec<char>>, x:usize, y:usize) {
        map[y][x] = ' ';
        if x > 0 && x < map[0].len() - 1 && map[y][x-1] == '-' && map[y][x+1] == '-' {
            map[y][x-1] = ' ';
            map[y][x+1] = ' ';
        } else if y > 0 && y < map.len() - 1 && map[y-1][x] == '|' && map[y+1][x] == '|' {
            map[y-1][x] = ' ';
            map[y+1][x] = ' ';
        }
    }

}
