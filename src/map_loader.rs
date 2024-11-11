use std::fs::File;
use std::io::{BufRead, BufReader};
use nalgebra_glm::Vec2;

pub struct MapData {
    pub map: Vec<Vec<char>>,
    pub player_pos: Vec2,
    pub enemies_pos: Vec<Vec2>
}

pub fn load_map(filename: &str) -> MapData {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut player_pos = Vec2::new(0.0,0.0);
    let mut enemies_pos = Vec::new();

    for (row_idx, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");
        let mut row: Vec<char> = Vec::new();

        for (col_idx, ch) in line.chars().enumerate() {
            match ch {
                'p' => {
                    player_pos = Vec2::new(col_idx as f32, row_idx as f32);
                    row.push(' ');
                }
                'e' => {
                    enemies_pos.push(Vec2::new(col_idx as f32, row_idx as f32));
                    row.push(' ');
                }
                _ => row.push(ch),
            }
        }
        map.push(row);
    }
    MapData {
        map,
        player_pos,
        enemies_pos,
    }
}

pub fn change_map(map_files: &Vec<&str>, current_map_index: &mut usize) -> MapData {
    *current_map_index = (*current_map_index + 1) % map_files.len();
    load_map(map_files[*current_map_index])
}

pub fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}
