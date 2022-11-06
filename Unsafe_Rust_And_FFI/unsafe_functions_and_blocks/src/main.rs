unsafe fn get_value(i: *const i32) ->i32{
    *i
}

fn main() {
   let foo = &1024 as *const i32;
    let _bar= unsafe { println!("{:?}",get_value(foo)) };
    let my_num: Box<i32> = Box::new(10);
    let my_num_ptr: *const i32 = &*my_num;
    let mut my_speed: Box<i32> = Box::new(88);
    let mut  y = *my_speed;
    let my_speed_ptr:*const i32 = &mut y;
    let my_speed_ptr2=&mut y;
    println!("{:?} {:?}",my_speed_ptr,my_speed_ptr2);
}
