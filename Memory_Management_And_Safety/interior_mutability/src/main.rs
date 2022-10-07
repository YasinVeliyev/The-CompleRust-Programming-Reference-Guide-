mod cell;
use cell::*;
mod refcell;
use refcell::*;
mod uses_of_interior_mutability;
use uses_of_interior_mutability::*;
fn main() {
    cell();
    uses_of_interior_mutability();
    refcell();
}
