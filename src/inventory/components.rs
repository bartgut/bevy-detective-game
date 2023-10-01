use crate::clickable::items::components::Collectible;

pub trait Inventory {
    fn add_item(&mut self, collectible: &Collectible);
    fn get_all_items(&self) -> Vec<Collectible>;
    fn has_item(&self, item_name: &str) -> bool {
        self.get_all_items().iter().any(|i| i.name == item_name)
    }
}
