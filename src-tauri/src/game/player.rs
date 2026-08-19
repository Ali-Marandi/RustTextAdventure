use super::types::*;

// ============================================================
// Player State Management
// ============================================================

pub struct Player {
    pub inventory: Vec<Item>,
    pub health: i32,
    pub max_health: i32,
    pub moves: u32,
    pub rooms_explored: u32,
    pub puzzles_solved: u32,
    pub items_collected: u32,
    pub score: u32,
    pub play_time_seconds: u64,
    pub start_time: std::time::Instant,
}

impl Player {
    pub fn new() -> Self {
        Player {
            inventory: Vec::new(),
            health: 100,
            max_health: 100,
            moves: 0,
            rooms_explored: 0,
            puzzles_solved: 0,
            items_collected: 0,
            score: 0,
            play_time_seconds: 0,
            start_time: std::time::Instant::now(),
        }
    }

    pub fn has_item(&self, item_id: &str) -> bool {
        self.inventory.iter().any(|i| i.id == item_id)
    }

    pub fn add_item(&mut self, item: Item) {
        self.items_collected += 1;
        self.score += match item.category {
            ItemCategory::Treasure => 50,
            ItemCategory::Quest => 30,
            ItemCategory::Key => 25,
            ItemCategory::PuzzlePiece => 20,
            ItemCategory::Tool => 10,
            ItemCategory::Document => 15,
            ItemCategory::Consumable => 5,
            ItemCategory::Container => 15,
        };
        self.inventory.push(item);
    }

    pub fn remove_item(&mut self, item_id: &str) -> Option<Item> {
        if let Some(pos) = self.inventory.iter().position(|i| i.id == item_id) {
            Some(self.inventory.remove(pos))
        } else {
            None
        }
    }

    pub fn increment_moves(&mut self) {
        self.moves += 1;
    }

    pub fn update_play_time(&mut self) {
        self.play_time_seconds = self.start_time.elapsed().as_secs();
    }

    pub fn get_state(&self) -> PlayerState {
        PlayerState {
            health: self.health,
            max_health: self.max_health,
            moves: self.moves,
            rooms_explored: self.rooms_explored,
            puzzles_solved: self.puzzles_solved,
            items_collected: self.items_collected,
            play_time_seconds: self.play_time_seconds,
        }
    }

    pub fn get_inventory_items(&self) -> Vec<InventoryItem> {
        self.inventory.iter().map(|item| InventoryItem {
            id: item.id.clone(),
            name: item.name.clone(),
            description: item.description.clone(),
            icon: item.icon.clone(),
            category: format!("{:?}", item.category),
            usable: item.usable,
        }).collect()
    }
}
