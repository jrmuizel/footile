// over.rs      Example stroking over
//
// Copyright (c) 2017  Douglas P Lau
//
extern crate footile;

use footile::PlotterBuilder;

fn main() {
    let mut p = PlotterBuilder::new()
                               .width(64)
                               .height(64)
                               .build();
    p.pen_width(8f32, false)
     .move_to(32f32, 16f32)
     .line_to(16f32, 16f32)
     .line_to(-16f32, 16f32)
     .line_to(-16f32, -16f32)
     .line_to(16f32, -16f32)
     .line_to(0f32, 32f32)
     .stroke();
    p.mask().write_png("./over.png").unwrap();
}