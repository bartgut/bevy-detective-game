use crate::clickable::items::components::Collectible;

pub trait Inventory {
    fn add_item(&mut self, collectible: &Collectible);
    fn get_all_items(&self) -> Vec<Collectible>;
}
