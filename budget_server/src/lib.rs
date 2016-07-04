extern crate pencil;

use pencil::Pencil;

pub fn run_server() {
    let mut app = Pencil::new("/");

    app.run("0.0.0.0:5055");
}
