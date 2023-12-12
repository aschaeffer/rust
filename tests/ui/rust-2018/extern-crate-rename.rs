// aux-build:edition-lint-paths.rs
// run-rustfix

// Oddball: crate is renamed, making it harder for us to rewrite
// paths. We don't (and we leave the `extern crate` in place).

#![deny(absolute_paths_not_starting_with_crate)]

extern crate edition_lint_paths as my_crate;

use my_crate::foo;
//~^ ERROR absolute paths must start
//~| WARNING this is accepted in the current edition

fn main() {
    foo();
}
