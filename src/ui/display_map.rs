use crate::entities::player::Player;

fn terminal_display(player: Player)
{
    let mut viewport_top = [0,0];

    player.pos.iter().enumerate().for_each(|(pos, coord)|
    {
        if coord < &7
        {
            viewport_top[pos] = 0;
            return;
        }
        if coord + 7 > player.current_floor.tile_vector[0].len() -1
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
            match player.current_floor.tile_vector[x][y].name.as_str()
            {
                "Floor" => print!("{}", ' '),
                "Wall" => print!("{}", ' '),
                _ => panic!("Tile has no name!")
            }
            print!("");
        }
    }


}