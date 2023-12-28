use crate::watcher::watch;

mod parsing;
mod cms_types;
mod gen;
mod watcher;


fn main() {
    watch();
}
