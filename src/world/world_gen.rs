use crate::world::tiles::Tile;
use crate::world::tiles::Room;
use rand::prelude::*;

pub struct FloorMap
{
	pub tile_vector: Vec<Vec<Tile>>,
	pub _floor_number: i32,
	pub _entity_count: i32,
	pub rooms: Vec<Room>,

}

impl FloorMap
{
	fn init_floor(height: usize, lenght: usize, floor_number: i32) -> FloorMap
	{
		return FloorMap
		{	
			tile_vector: vec![vec![Tile {name: "Floor".to_string(), _is_walkable: true}; height]; lenght],
			_floor_number: floor_number,
			_entity_count: 0,
			rooms: Vec::new()
		}
		
	}

	pub fn add_room(&mut self, room_type: &str)
	{
		let mut room: Room = Room::init();

		if self.rooms.capacity() == 0
		{
			let middle_coord = [self.tile_vector[0].len()/2,self.tile_vector.len()/2];
			room = Self::match_room_type(room_type, middle_coord[0],middle_coord[1],room);

			self.add_tiles_to_world(&room);
			let mut room_list: Vec<Room> = [room].to_vec();
			self.rooms.append(&mut room_list);

		}
		else {
			let mut available_room_slots = "".to_string();
			let mut cord_x = 0;
			let mut cord_y = 0;
			// This loop might eventually get stuck in the future in the situation where we can't find any available rooms
			// but i can't think of a good fix rn
			while available_room_slots.is_empty()
			{
				let room_number = rand::thread_rng().gen_range(0..self.rooms.len());
				cord_x = self.rooms[room_number].bounds[0][0];
				cord_y = self.rooms[room_number].bounds[0][1];
				available_room_slots = self.room_collision_checker(room_number); 
			}

			let direction_index = rand::thread_rng().gen_range(0..available_room_slots.len());
			let direction = available_room_slots.chars().nth(direction_index);
			println!("{:?}",direction);

			match direction
			{
				Some('N') => room = Self::match_room_type(room_type,cord_x,cord_y+16,room),
				Some('E') => room = Self::match_room_type(room_type,cord_x+16,cord_y,room),
				Some('S') => room = Self::match_room_type(room_type,cord_x,cord_y-16,room),
				Some('W') => room = Self::match_room_type(room_type,cord_x-16,cord_y,room),
				_ => panic!("There should never be a situation where we have a room that doesn't match at this depth!")
			}

			self.add_tiles_to_world(&room);
			let mut room_list: Vec<Room> = [room].to_vec();
			self.rooms.append(&mut room_list);
			self.rooms.iter().for_each(|x|
			{
				println!("{:?}",x.bounds);
			})
			
		}
	}

	//Function currently only supports adding walls but it will be adding every single other tile type as time goes on
	fn add_tiles_to_world(&mut self, room: &Room)
	{
		println!("{:?}", room.walls);
		for coords in room.walls.chunks_exact(2)
		{
			if coords.len() == 2
			{
				self.tile_vector[coords[0]][coords[1]] = Tile {name: "Wall".to_string(), _is_walkable: false};
	        }
		}
	}

	fn room_collision_checker(&self, room_number: usize) -> String
	{
		let room_x = self.rooms[room_number].bounds[0][0];
		let room_y = self.rooms[room_number].bounds[0][1];
		let mut available_room_slots ="NESW".to_string();
		self.rooms.iter().for_each(|room|
		{
			if room.bounds[0][0] == room_x && room.bounds[0][1] == room_y+16
			{
				available_room_slots = available_room_slots.replace("N","");
			}
			if room.bounds[0][0] == room_x+16 && room.bounds[0][1] == room_y
			{
				available_room_slots = available_room_slots.replace("E","");
			}
			if room.bounds[0][0] == room_x && room.bounds[0][1] == room_y-16
			{
				available_room_slots = available_room_slots.replace("S","");
			}
			if room.bounds[0][0] == room_x-16 && room.bounds[0][1] == room_y
			{
				available_room_slots = available_room_slots.replace("W","");
			}
		});
		return available_room_slots;

	}

	fn match_room_type(room_type: &str, x: usize, y: usize, mut room: Room) -> Room
	{
		match room_type
		{
			"barren" => room.barren([x,y],[x+15,y+15]),
			"empty" => room.empty([x,y],[x+15,y+15]),
			_ => panic!("No room type provided!")
		};
		return room;
	}
}

pub fn world_gen(height: usize, lenght: usize) -> FloorMap
{	
	let world = FloorMap::init_floor(height, lenght, 0);
	return world;
}