use image::{ImageBuffer, Rgba};

const GRID_LINE_COUNT: u32 = 10;
const GRID_WIDTH: u32 = 500;
const GRID_HEIGHT: u32 = 500;
const CELL_WIDTH: u32 = GRID_WIDTH / GRID_LINE_COUNT;
const CELL_HEIGHT: u32 = GRID_HEIGHT / GRID_LINE_COUNT;

fn main() {
    ImageBuffer::from_par_fn(GRID_WIDTH, GRID_HEIGHT, |i, j| {
        if i % CELL_HEIGHT == 0
            || j % CELL_WIDTH == 0
            || i == GRID_HEIGHT - 1
            || j == GRID_WIDTH - 1
        {
            Rgba([255u8, 0, 0, 255])
        } else {
            Rgba([0, 0, 0, 0])
        }
    })
    .save("./assets/grid.png")
    .unwrap();
}
