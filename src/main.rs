#[path="./lib.rs"]
mod wgpu_3d_engine;

use wgpu_3d_engine::run;

fn main() {
    run().unwrap();
}
