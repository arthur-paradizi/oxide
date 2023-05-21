mod tiles;
use tiles::Tile;
use tiles::Room;

struct FloorMap
{
	tile_vector: Vec<Vec<Tile>>,
	_floor_number: i32,
	_entity_count: i32,
	rooms: Vec<(Room, [bool;4])>,

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

	fn add_room(&mut self, room_type: &str)
	{
		let mut room: Room = Room::init();

		if self.rooms.capacity() == 0
		{
			let middle_coord = [self.tile_vector.capacity()/2,self.tile_vector[0].capacity()/2];
			println!("{:?}", middle_coord);
			match room_type
			{
				"barren" => room.barren([middle_coord[0],middle_coord[1]],[middle_coord[0]+15,middle_coord[1]+15]),
				"empty" => room.empty([middle_coord[0],middle_coord[1]],[middle_coord[0]+15,middle_coord[1]+15]),
				_ => panic!("No room type provided!")
			}
			self.add_tiles_to_world(&room);
			let mut room_list: Vec<(Room, [bool;4])> = [(room,[false,false,false,true])].to_vec();
			self.rooms.append(&mut room_list);

		}
		else {
			unimplemented!();
		}
	}

	//Function currently only supports adding walls but it will be adding every single other tile type as time goes on
	fn add_tiles_to_world(&mut self, room: &Room)
	{
		for coords in room.walls.chunks_exact(2)
		{
			if coords.len() == 2
			{
				self.tile_vector[coords[0]][coords[1]] = Tile {name: "Wall".to_string(), _is_walkable: false};
	        }
		}
	}
}

fn world_gen(height: usize, lenght: usize) -> FloorMap
{	
	let world = FloorMap::init_floor(height, lenght, 0);
	return world;
}

fn main()
{
	let mut world = world_gen(64,64);
	world.add_room("empty");
	//println!("Here's the world representation: \n{:?}", world);
	world.tile_vector.iter().for_each(|tile_vec|
	{
			tile_vec.iter().for_each(|tile|
			{
				match tile.name.as_str() {
					"Wall" => print!("|"),
					"Floor" => print!("{}", ' '),
					_ => print!("{}", ' '),
				};
			});
            print!("{}", '\n');
   })
}