//@ run-rustfix
//@ compile-flags: --cfg=whatever -Aunused

#[cfg(whatever)]
use y::Whatever;

mod y {
    pub(crate) fn z() {}
    pub(crate) struct Whatever;
}

fn main() {
    z();
    //~^ ERROR cannot find function `z` in this scope
}
