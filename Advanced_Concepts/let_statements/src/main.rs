mod let_ref_mut;
use let_ref_mut::*;
mod destructure_struct;
use destructure_struct::*;
mod match_ref;
use match_ref::*;
mod destructure_enum;
use destructure_enum::*;
mod destructure_func_param;
use destructure_func_param::*;
fn main() {
    let_red_mut();
    destructure_struct();
    match_ref();
    destructure_enum();
    destructure_func_param();
}
