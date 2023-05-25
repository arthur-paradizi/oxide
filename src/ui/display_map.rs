use crate::world::world_gen::FloorMap;
use crate::entities::player::Player;
use crate::entities::entity::Entity;

pub fn terminal_display(player: &Player, current_floor: &FloorMap)
{
    let mut viewport_top = [0,0];

    player.position.iter().enumerate().for_each(|(pos, coord)|
    {
        if coord < &7
        {
            viewport_top[pos] = 0;
            return;
        }
        if coord + 7 > current_floor.tile_vector[0].len() -1
        {
            viewport_top[pos] = 0;
            return;
        }
        viewport_top[pos] = *coord;
    });
    println!("coords are {:?}", viewport_top);
    for x in 0..15
    {
        for y in 0..15
        {
            floor_rendering(&current_floor, x, y);
        }
        println!("");
    }
}

fn floor_rendering(current_floor: &FloorMap, x: usize, y: usize)
{
    if current_floor.tile_vector[x][y].entities.len() > 0
    {
        match &current_floor.tile_vector[x][y].entities[0] {
            Entity::Player(_) => print!("@"),
            Entity::Monster(_) => print!("M")
        }
    }
    else
    {
        match current_floor.tile_vector[x][y].name.as_str()
        {
            "Floor" => print!("{}", ' '),
            "Wall" => print!("{}", "|"),
            _ => panic!("Tile has no name!")
        }
    }
}
