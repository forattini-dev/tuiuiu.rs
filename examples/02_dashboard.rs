//! Dashboard Example
//!
//! Run with: cargo run --example dashboard

use tuiuiu::prelude::*;

fn main() -> std::io::Result<()> {
    let ui = Box::new()
        .column()
        .padding(1)
        .gap(1)
        .border_round()
        .children([
            Text::new("📊 Dashboard").cyan().bold().build(),
            
            Box::new()
                .row()
                .gap(2)
                .children([
                    Box::new()
                        .column()
                        .padding(1)
                        .border(tuiuiu::utils::border::BorderStyle::Single)
                        .children([
                            Text::new("CPU").yellow().build(),
                            Text::new("45%").green().build(),
                        ])
                        .build(),
                    
                    Box::new()
                        .column()
                        .padding(1)
                        .border(tuiuiu::utils::border::BorderStyle::Single)
                        .children([
                            Text::new("Memory").yellow().build(),
                            Text::new("2.1 GB").cyan().build(),
                        ])
                        .build(),
                ])
                .build(),
            
            Text::new("Press q to quit").gray().dim().build(),
        ]);

    let output = tuiuiu::core::renderer::render_to_string(&ui.build(), 50, 15);
    println!("{}", output);

    Ok(())
}
