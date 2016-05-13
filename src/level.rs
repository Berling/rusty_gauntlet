pub mod level {
    pub enum TileType {
        Floor,
        Wall
    }

    pub struct Tile {
        pub tile_type: TileType
    }

    pub struct Level {
        pub width: i32,
        pub height: i32,
        pub tiles: Vec<Vec<Tile>>
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
                        _ => TileType::Wall
                    };
                    row.push(Tile{tile_type: tt});
                }
                if row.len()!=w as usize {
                    panic!("Error in map file. Too few columns ({}) in row {}.", row.len(), data.len()+1);
                }
                data.push(row);
            }
            if data.len()!=h as usize {
                panic!("Error in map file. Too few rows ({}).", data.len());
            }

            Level {width: w, height: h, tiles: data}
        }

        pub fn debug_print(&self) {
            println!("Level data ({} x {}):", self.width, self.height);
            for line in &self.tiles {
                for tile in line {
                    match tile.tile_type {
                        TileType::Floor => print!("."),
                        TileType::Wall => print!("#")
                    }
                }
                println!("");
            }

            println!("----------------");
        }
    }

}
