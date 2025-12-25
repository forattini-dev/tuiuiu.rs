//! Basic Counter Example
//!
//! Run with: cargo run --example counter

use tuiuiu::prelude::*;

fn main() -> std::io::Result<()> {
    // Create reactive state
    let (count, set_count) = create_signal(0);

    // Build the UI
    let ui = Box::new()
        .column()
        .padding(1)
        .border_round()
        .children([
            Text::new("🐦 Tuiuiu Counter").cyan().bold().build(),
            Text::new(format!("Count: {}", count.get())).build(),
            Text::new("↑/↓: change • Esc: exit").gray().dim().build(),
        ]);

    // Render and get the output
    let output = tuiuiu::core::renderer::render_to_string(&ui.build(), 40, 10);
    
    println!("{}", output);
    println!();
    println!("(Interactive mode not yet implemented)");
    println!("This example demonstrates the component structure.");

    Ok(())
}
