use lib::entities::player::Player;
use lib::entities::entity::Entity;
use lib::ui::display_map::terminal_display;
use lib::world::world_gen;
use std::io;
//use rand::prelude::*;

fn main()
{
	
	let grid_size = get_grid_size();
	let mut world = world_gen::world_gen(grid_size,grid_size,Player::new([1,1], 0));
	for _n in 0..get_room_amount()
	{
		world.add_room("empty");
	}
	let mut player: &Player = &Player::new([1,1], 0);
	if let Entity::Player(player_ref) = &world.entities[0]
	{
        player = player_ref;
    }
	println!("{:?}", player);
	terminal_display(player, &world);
	
}

fn get_grid_size() -> usize
{
	println!("Please input world grid size (the bigger the number of rooms wanted the bigger the world needs to be):");
	let mut input_line = String::new();
	io::stdin()
    .read_line(&mut input_line)
    .expect("Failed to read line");
	let grid_size: usize = input_line.trim().parse().expect("Input not an integer");
	return grid_size;
}

fn get_room_amount() -> i32
{
	println!("Please input number of rooms desired:");
	let mut input_line = String::new();
	io::stdin()
    .read_line(&mut input_line)
    .expect("Failed to read line");
	let room_amount: i32 = input_line.trim().parse().expect("Input not an integer");
	return room_amount;
}