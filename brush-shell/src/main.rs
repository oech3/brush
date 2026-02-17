//! Main entry for the `brush` shell.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
fn main() {
    brush_shell::entry::run();
}
