pub use a::TypeA;
// pub type TypeA = a::TypeA;

#[no_mangle]
pub unsafe extern "C" fn free_standing_function(grand_parent: *const TypeA) -> u32 {
    dbg!(grand_parent);

    1234
}
