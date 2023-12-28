use crate::watcher::watch;

mod parsing;
mod cms_types;
mod gen;
mod watcher;
mod assets;
mod img_handling;

fn main() {
    watch();
}
