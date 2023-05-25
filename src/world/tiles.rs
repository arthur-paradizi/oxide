use crate::entities::entity::Entity;

#[derive(Clone,Debug)]
pub struct Tile<'a>
{
	pub name: String,
	pub is_walkable: bool,
	pub entities: Vec<&'a Entity>

}

#[derive(Debug,Clone)]
pub struct Room
{
	pub tileset: String,
	pub walls: Vec<usize>,
	pub bounds: [[usize;2];2]
}

impl Room
{
	pub fn init() -> Room
	{
		return Room
		{
			tileset: "none".to_string(),
			walls: [].to_vec(),
			bounds: [[0,0],[0,0]]
		}
		
	}

	pub fn barren(&mut self, start_coord: [usize;2], end_coord: [usize;2])
	{
		self.tileset = "barren".to_string();
		self.walls = [].to_vec();
		self.bounds = [start_coord,end_coord];
	}

	pub fn empty(&mut self, start_coord: [usize;2], end_coord: [usize;2])
	{	
		let pos_walls = [0,0,1,0,0,1,14,0,15,0,15,1,0,14,0,15,1,15,14,15,15,15,15,14];
		let mut abs_walls = [0,0,1,0,0,1,14,0,15,0,15,1,0,14,0,15,1,15,14,15,15,15,15,14];
		let mut counter = 0;
		for coords in pos_walls.chunks_exact(2)
		{
			if coords.len() == 2
			{
				abs_walls[counter] = coords[0] + start_coord[0];
				abs_walls[counter+1] = coords[1] + start_coord[1];
				counter += 2;
	        }
		}
		self.tileset = "empty".to_string();
		self.walls = abs_walls.to_vec();
		self.bounds = [start_coord,end_coord];
	}
}