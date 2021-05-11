#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use init::InitVulkan;

mod init;

fn main() {
    let mut app = InitVulkan::initialize();
    app.main_loop();
}
