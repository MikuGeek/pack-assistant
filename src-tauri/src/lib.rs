// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// Constants for outer box parameters
const CARDBOARD_THICKNESS_CM: f64 = 0.6;
const CARDBOARD_WEIGHT_KG_PER_SQM: f64 = 0.54;

// Destination constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationConstraints {
    pub max_box_dimension: f64,     // Maximum dimension for any side of the box
    pub max_box_weight: f64,        // Maximum weight of a filled box
    pub alternative_dimensions: Option<(f64, f64, f64)>, // For special cases like Japan (length, width, height)
}

// Item dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub destination: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub weight: f64,
    // For tracking placement in solution
    pub position: Option<(f64, f64, f64)>,
    pub box_index: Option<usize>,
}

impl Item {
    pub fn volume(&self) -> f64 {
        self.length * self.width * self.height
    }
}

// Packed box with items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackedBox {
    pub items: Vec<Item>,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub weight: f64,
    pub destination: String,
}

impl PackedBox {
    pub fn new(destination: &str) -> Self {
        PackedBox {
            items: Vec::new(),
            length: 0.0,
            width: 0.0,
            height: 0.0,
            weight: 0.0,
            destination: destination.to_string(),
        }
    }

    pub fn volume(&self) -> f64 {
        self.length * self.width * self.height
    }

    pub fn add_item(&mut self, mut item: Item, position: (f64, f64, f64)) -> bool {
        // Calculate new dimensions after adding the item
        let new_length = (position.0 + item.length).max(self.length);
        let new_width = (position.1 + item.width).max(self.width);
        let new_height = (position.2 + item.height).max(self.height);
        // Store item weight before pushing to items vector
        let item_weight = item.weight;

        // Update item with position and box information
        item.position = Some(position);
        item.box_index = Some(self.items.len());

        self.items.push(item);
        self.length = new_length;
        self.width = new_width;
        self.height = new_height;
        self.weight += item_weight;

        // Calculate box weight including cardboard
        self.update_box_weight();

        true
    }

    fn update_box_weight(&mut self) {
        // Calculate box surface area in square meters
        let length_m = (self.length + 2.0 * CARDBOARD_THICKNESS_CM) / 100.0;
        let width_m = (self.width + 2.0 * CARDBOARD_THICKNESS_CM) / 100.0;
        let height_m = (self.height + 2.0 * CARDBOARD_THICKNESS_CM) / 100.0;

        // Calculate box surface area (2 * (lw + lh + wh))
        let surface_area = 2.0 * (length_m * width_m + length_m * height_m + width_m * height_m);

        // Calculate box weight
        let box_weight = surface_area * CARDBOARD_WEIGHT_KG_PER_SQM;

        // Total weight = items weight + box weight
        let items_weight: f64 = self.items.iter().map(|item| item.weight).sum();
        self.weight = items_weight + box_weight;
    }
}

// Packing solution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackingSolution {
    pub boxes: Vec<PackedBox>,
    pub total_volume: f64,
    pub unpacked_items: Vec<Item>,
}

// Get destination constraints
fn get_destination_constraints(destination: &str) -> DestinationConstraints {
    match destination {
        "Australia" => DestinationConstraints {
            max_box_dimension: 63.0,
            max_box_weight: 22.0,
            alternative_dimensions: None,
        },
        "USA" => DestinationConstraints {
            max_box_dimension: 63.0,
            max_box_weight: 22.0,
            alternative_dimensions: None,
        },
        "UK" => DestinationConstraints {
            max_box_dimension: 63.0,
            max_box_weight: 15.0,
            alternative_dimensions: None,
        },
        "Germany" => DestinationConstraints {
            max_box_dimension: 63.0,
            max_box_weight: 22.5,
            alternative_dimensions: None,
        },
        "Japan" => DestinationConstraints {
            max_box_dimension: 60.0,
            max_box_weight: 40.0,
            alternative_dimensions: Some((60.0, 50.0, 50.0)),
        },
        _ => DestinationConstraints {
            max_box_dimension: 63.0,
            max_box_weight: 22.0,
            alternative_dimensions: None,
        },
    }
}

// Check if an item fits within destination constraints
fn fits_constraints(item: &Item, constraints: &DestinationConstraints) -> bool {
    if let Some((max_length, max_width, max_height)) = constraints.alternative_dimensions {
        // Special case for destinations with specific dimension constraints (like Japan)
        return item.length <= max_length &&
               item.width <= max_width &&
               item.height <= max_height &&
               item.weight <= constraints.max_box_weight;
    } else {
        // Standard case
        return item.length <= constraints.max_box_dimension &&
               item.width <= constraints.max_box_dimension &&
               item.height <= constraints.max_box_dimension &&
               item.weight <= constraints.max_box_weight;
    }
}

// Check if item can be placed at the position without collision
fn can_place_item(box_data: &PackedBox, item: &Item, position: (f64, f64, f64)) -> bool {
    let (x, y, z) = position;

    // Check if the item fits within box constraints
    let constraints = get_destination_constraints(&box_data.destination);

    // Check if item extends beyond the maximum allowed dimensions
    if let Some((max_length, max_width, max_height)) = constraints.alternative_dimensions {
        if x + item.length > max_length ||
           y + item.width > max_width ||
           z + item.height > max_height {
            return false;
        }
    } else {
        if x + item.length > constraints.max_box_dimension ||
           y + item.width > constraints.max_box_dimension ||
           z + item.height > constraints.max_box_dimension {
            return false;
        }
    }

    // Check for collisions with existing items
    for existing_item in &box_data.items {
        if let Some((ex, ey, ez)) = existing_item.position {
            // Check if the new item overlaps with any existing item
            if !(x + item.length <= ex ||
                 ex + existing_item.length <= x ||
                 y + item.width <= ey ||
                 ey + existing_item.width <= y ||
                 z + item.height <= ez ||
                 ez + existing_item.height <= z) {
                return false;
            }
        }
    }

    // Check if total weight would exceed maximum
    let new_weight = box_data.weight + item.weight;
    if new_weight > constraints.max_box_weight {
        return false;
    }

    true
}

// Find the best position to place an item in a box
fn find_best_position(box_data: &PackedBox, item: &Item) -> Option<(f64, f64, f64)> {
    // For an empty box, place at origin
    if box_data.items.is_empty() {
        return Some((0.0, 0.0, 0.0));
    }

    // Get all extreme points (candidates for placement)
    let mut candidates = Vec::new();

    // Add (0,0,0) as a candidate
    candidates.push((0.0, 0.0, 0.0));

    // Add extreme points based on existing items
    for existing_item in &box_data.items {
        if let Some((ex, ey, ez)) = existing_item.position {
            // Point to the right of the item
            candidates.push((ex + existing_item.length, ey, ez));

            // Point to the front of the item
            candidates.push((ex, ey + existing_item.width, ez));

            // Point on top of the item
            candidates.push((ex, ey, ez + existing_item.height));
        }
    }

    // Sort candidates by the sum of coordinates (prefer closer to origin)
    candidates.sort_by(|a, b| {
        let sum_a = a.0 + a.1 + a.2;
        let sum_b = b.0 + b.1 + b.2;
        sum_a.partial_cmp(&sum_b).unwrap_or(Ordering::Equal)
    });

    // Try each candidate position
    for pos in candidates {
        if can_place_item(box_data, item, pos) {
            return Some(pos);
        }
    }

    None
}

// Main packing algorithm implementation
fn pack_items_impl(items: Vec<Item>) -> PackingSolution {
    // Group items by destination
    let mut items_by_destination: std::collections::HashMap<String, Vec<Item>> = std::collections::HashMap::new();

    for item in items {
        items_by_destination
            .entry(item.destination.clone())
            .or_insert_with(Vec::new)
            .push(item);
    }

    let mut solution = PackingSolution {
        boxes: Vec::new(),
        total_volume: 0.0,
        unpacked_items: Vec::new(),
    };

    // Process each destination separately
    for (destination, mut destination_items) in items_by_destination {
        // Sort items by volume (decreasing)
        destination_items.sort_by(|a, b| {
            b.volume().partial_cmp(&a.volume()).unwrap_or(Ordering::Equal)
        });

        let mut boxes_for_destination: Vec<PackedBox> = Vec::new();
        let mut unpacked = Vec::new();

        // Process each item
        for item in destination_items {
            let constraints = get_destination_constraints(&destination);

            // Check if the item itself is too large for constraints
            if !fits_constraints(&item, &constraints) {
                unpacked.push(item);
                continue;
            }

            let mut placed = false;

            // Try to place in existing boxes
            for box_data in &mut boxes_for_destination {
                if let Some(position) = find_best_position(box_data, &item) {
                    box_data.add_item(item.clone(), position);
                    placed = true;
                    break;
                }
            }

            // If not placed, create a new box
            if !placed {
                let mut new_box = PackedBox::new(&destination);
                new_box.add_item(item.clone(), (0.0, 0.0, 0.0));
                boxes_for_destination.push(new_box);
            }
        }

        // Add to solution
        solution.boxes.extend(boxes_for_destination);
        solution.unpacked_items.extend(unpacked);
    }

    // Calculate total volume
    solution.total_volume = solution.boxes.iter().map(|b| b.volume()).sum();

    solution
}

// Calculate the cost of a packing solution (assuming cost is proportional to volume)
fn calculate_cost_impl(solution: &PackingSolution) -> f64 {
    solution.total_volume
}

// Define commands in a separate module to avoid name conflicts
pub mod commands {
    use super::*;

    #[tauri::command]
    pub fn pack_items(items: Vec<Item>) -> PackingSolution {
        pack_items_impl(items)
    }

    #[tauri::command]
    pub fn calculate_cost(solution: PackingSolution) -> f64 {
        calculate_cost_impl(&solution)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
        commands::pack_items,
        commands::calculate_cost
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
