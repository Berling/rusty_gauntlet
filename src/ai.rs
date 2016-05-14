use level::*;

pub fn ai_step(level : &mut Level, player_pos: (i32,i32)) {
    let mut x = 0;
    let mut y = 0;
    let mut enemies = Vec::with_capacity(16);

    for line in &level.tiles {
        for tile in line {
            match tile.entity {
                Some(Entity::Dragon{..}) => {
                    enemies.push((x,y));
                },
                _ => {}
            }
            x+=1;
        }
        y+=1;
        x=0;
    }

    let (px, py) = player_pos;
    for e in enemies {
        let (x,y) = e;
        let dx = (px-x) as f32;
        let dy = (py-y) as f32;
        let distance = (dx*dx + dy*dy).sqrt();

        if distance<6f32 {
            let dir = if dx<0f32 {
                Direction::Left
            } else if dx>0f32 {
                Direction::Right
            } else if dy<0f32 {
                Direction::Up
            } else {
                Direction::Down
            };
            level.interact((x,y), dir);
        }
    }
}
