mod result_commont_pattern;
mod using_map;
use result_commont_pattern::*;
use using_map::*;
fn main() {
    using_map();
    let valid_str = str_upper_match(vec![121, 97, 89]);
    println!("{:?}", valid_str);
    let valid_str = str_upper_match_with_question_operator(vec![121, 97, 89]);
    println!("{:?}", valid_str);
}
