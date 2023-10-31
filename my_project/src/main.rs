// main.rs
mod database;

fn main() {
    // Create the table
    match database::create_table() {
        Ok(()) => println!("Table created successfully."),
        Err(e) => println!("Error creating table: {}", e),
    }

    // Insert items
    match database::insert_item("Apple") {
        Ok(()) => println!("Item inserted successfully."),
        Err(e) => println!("Error inserting item: {}", e),
    }

    match database::insert_item("Banana") {
        Ok(()) => println!("Item inserted successfully."),
        Err(e) => println!("Error inserting item: {}", e),
    }

    // Read items
    match database::read_items() {
        Ok(()) => println!("Items read successfully."),
        Err(e) => println!("Error reading items: {}", e),
    }

    // Update an item
    match database::update_item(1, "Orange") {
        Ok(()) => println!("Item updated successfully."),
        Err(e) => println!("Error updating item: {}", e),
    }

    // Read items again to see the updated item
    match database::read_items() {
        Ok(()) => println!("Items read successfully."),
        Err(e) => println!("Error reading items: {}", e),
    }

    // Delete an item
    match database::delete_item(2) {
        Ok(()) => println!("Item deleted successfully."),
        Err(e) => println!("Error deleting item: {}", e),
    }

    // Read items again to see the remaining items
    match database::read_items() {
        Ok(()) => println!("Items read successfully."),
        Err(e) => println!("Error reading items: {}", e),
    }
}
