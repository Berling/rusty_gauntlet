//pub mod level {
    pub enum Direction {
        Up, Down, Left, Right
    }
    fn move_in_dir(x: i32, y: i32, dir: Direction) -> (i32,i32) {
        return match dir {
            Direction::Up    => (x,y-1),
            Direction::Down  => (x,y+1),
            Direction::Left  => (x-1,y),
            Direction::Right => (x+1,y),
        }
    }


    #[derive(Copy, Clone)]
    pub enum TileType {
        Floor,
        Wall
    }

    #[derive(Copy, Clone)]
    pub enum Entity {
        Player{hp: i32, dmg: i32, score: i32},
        Dragon{hp: i32, dmg: i32},
        Treasure
    }

    #[derive(Copy, Clone)]
    pub struct Tile {
        pub tile_type: TileType,
        pub entity: Option<Entity>
    }

    #[derive(Default)]
    pub struct Level {
        pub width: i32,
        pub height: i32,
        pub tiles: Vec<Vec<Tile>>,
        pub on_player_damaged:   Option<fn(player: &Entity, attacker: &Entity)>,
        pub on_player_killed:    Option<fn(player: &Entity)>,
        pub on_player_attacked:  Option<fn(player: &Entity, target: &Entity)>,
        pub on_player_collected: Option<fn(player: &Entity)>
    }

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    impl Level {
        pub fn new(path: &Path) -> Level {
            let f = File::open(&path).unwrap();
            let file = BufReader::new(&f);
            let mut lines = file.lines();
            let first_line = lines.next().unwrap().unwrap();
            let mut size = first_line
                                .split_whitespace()
                                .map(|s| s.parse::<i32>());

            let w = size.next().unwrap().unwrap();
            let h = size.next().unwrap().unwrap();
            let mut data = Vec::with_capacity(h as usize);

            for line in lines {
                let l = line.unwrap();
                let mut row = Vec::with_capacity(w as usize);

                for c in l.chars() {
                    let tt = match c {
                        '#' => TileType::Wall,
                        '.' => TileType::Floor,
                        _ => TileType::Floor
                    };
                    let entity = match c {
                        '@' => Some(Entity::Player{hp:5, dmg:2,score:0}),
                        'D' => Some(Entity::Dragon{hp:4, dmg:1}),
                        '$' => Some(Entity::Treasure),
                        _ => None
                    };
                    row.push(Tile{tile_type: tt, entity: entity});
                }
                if row.len()!=w as usize {
                    panic!("Error in map file. Too few columns ({}) in row {}.", row.len(), data.len()+1);
                }
                data.push(row);
            }
            if data.len()!=h as usize {
                panic!("Error in map file. Too few rows ({}).", data.len());
            }

            Level {width: w, height: h, tiles: data, ..Default::default()}
        }

        pub fn debug_print(&self, ) {
            println!("Level data ({} x {}):", self.width, self.height);
            for line in &self.tiles {
                for tile in line {
                    match tile.entity {
                        Some(Entity::Player{..}) => print!("@"),
                        Some(Entity::Dragon{..}) => print!("D"),
                        Some(Entity::Treasure) => print!("$"),
                        None => {
                            match tile.tile_type {
                                TileType::Floor => print!("."),
                                TileType::Wall => print!("#")
                            }
                        }
                    }
                }
                println!("");
            }

            println!("----------------");
        }

        pub fn get_entity(&self, pos: (i32,i32)) -> Option<Entity> {
            let (x,y) = pos;
            return self.tiles[y as usize][x as usize].entity;
        }

        pub fn get_player_pos(&self) -> Option<(i32,i32)> {
            let mut y = 0;
            for line in &self.tiles {
                let mut x = 0;
                for tile in line {
                    match tile.entity {
                        Some(Entity::Player{..}) => return Some((x,y)),
                        _ => {}
                    }
                    x+=1;
                }
                y+=1;
            }

            return None;
        }

        pub fn interact(&mut self, pos: (i32,i32), dir: Direction) -> (i32,i32) {
            let mut new_pos = pos;
            let (x,y) = pos;
            let mut curr_tile = self.tiles[y as usize][x as usize];
            {
                let (nx,ny) = move_in_dir(x,y,dir);
                let mut new_tile = &mut self.tiles[ny as usize][nx as usize];

                match new_tile.tile_type {
                    TileType::Wall => return new_pos,
                    _ => {}
                }

                match curr_tile.entity {
                    Some(Entity::Player{hp,score,dmg:pdmg}) => {
                        match new_tile.entity {
                            Some(Entity::Treasure) => {
                                new_tile.entity = Some(Entity::Player{hp:hp,score:score+1,dmg:pdmg});
                                curr_tile.entity = None;
                                new_pos = (nx,ny);

                                match self.on_player_collected {
                                    Some(f) => f(&new_tile.entity.unwrap()),
                                    _ => {}
                                }
                            },
                            Some(Entity::Dragon{hp,dmg}) => {
                                match self.on_player_attacked {
                                    Some(f) => f(&new_tile.entity.unwrap(), &curr_tile.entity.unwrap()),
                                    _ => {}
                                }

                                if hp<=pdmg {
                                    new_tile.entity = Some(Entity::Treasure);
                                } else {
                                    new_tile.entity = Some(Entity::Dragon{hp:hp-pdmg,dmg:dmg});
                                }
                            },
                            _ => {
                                new_tile.entity = curr_tile.entity;
                                curr_tile.entity = None;
                                 new_pos = (nx,ny);
                            }
                        }
                    },
                    Some(Entity::Dragon{dmg,..}) => {
                        match new_tile.entity {
                            Some(Entity::Player{hp,dmg:pdmg,score}) => {
                                if hp<=dmg {
                                    match self.on_player_killed {
                                        Some(f) => f(&new_tile.entity.unwrap()),
                                        _ => {}
                                    }
                                    new_tile.entity = None;

                                } else {
                                    match self.on_player_damaged {
                                        Some(f) => f(&new_tile.entity.unwrap(), &curr_tile.entity.unwrap()),
                                        _ => {}
                                    }
                                    new_tile.entity = Some(Entity::Player{hp:hp-dmg,score:score,dmg:pdmg});
                                }
                            },
                            Some(Entity::Treasure) => {},
                            _ => {
                                new_tile.entity = curr_tile.entity;
                                curr_tile.entity = None;
                                 new_pos = (nx,ny);
                            }
                        }
                    }
                    _ => {}
                };
            }

            self.tiles[y as usize][x as usize] = curr_tile;
            return new_pos;
        }
    }

//}
