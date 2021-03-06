mod cli;
mod list;
mod memory;
mod person;
mod tree;
mod vector;

fn main() {
    crate::cli::run();
    crate::list::run();
    crate::person::run();
    crate::tree::run();
    crate::vector::run();
    crate::memory::run();
}
