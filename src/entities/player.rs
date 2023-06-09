use crate::entities::entity::Move;
use crate::entities::entity::SetPosition;

#[derive(Debug,Clone)]
pub struct Player
{
    pub position: [usize;2],
    pub hitponts: i32,
    pub speed: i32,
    pub current_floor: i32
}

impl Player
{
    pub fn new(position: [usize;2], current_floor: i32) -> Player
    {
        return Player
        {
            position: position,
            hitponts: 100,
            speed: 1,
            current_floor: current_floor
        }
    }    
}

impl Move for Player 
{
    fn move_tile(&mut self, direction: &str)
    {
        match direction  
        {
            "N" => self.position[1] = self.position[1] + 1,
            "E" => self.position[0] = self.position[0] + 1,
            "S" => self.position[1] = self.position[1] - 1,
            "W" => self.position[0] = self.position[0] - 1,
            _ => panic!("no direction provided!")
        }
    }
}

impl SetPosition for Player
{
    fn set_pos(&mut self, pos_set: [usize;2])
    {
        self.position = pos_set;
    }
}
