use crate::world::world_gen::FloorMap;

#[derive(Debug,Clone)]
pub struct Player
{
    pub pos: [usize;2],
    pub hp: i32,
    pub speed: i32,
    pub current_floor: FloorMap
}
impl Player 
{
    fn move_tile(&mut self, direction: &str)
    {
        match direction  
        {
            "N" => self.pos[1] = self.pos[1] + 1,
            "E" => self.pos[0] = self.pos[0] + 1,
            "S" => self.pos[1] = self.pos[1] - 1,
            "W" => self.pos[0] = self.pos[0] - 1,
            _ => panic!("no direction provided!")
        }
    }

    fn set_pos(&mut self, pos_set: [usize;2])
    {
        self.pos = pos_set;
    }
}
