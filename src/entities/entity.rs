use crate::entities::player::Player;
use crate::entities::npc::Monster;

#[derive(Clone,Debug)]
pub enum Entity
{
	Player(Player),
	Monster(Monster)
}

pub trait Move
{
	fn move_tile(&mut self, direction: &str);
}
pub trait SetPosition
{
	fn set_pos(&mut self, pos_set: [usize;2]);
}