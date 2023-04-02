#[derive(Debug)]
pub struct Carrot {}

mod paprika {
    pub mod seeds {
        pub fn grow() {}
    }
}

pub fn regenerate_food() {
    /* Absolute path */
    crate::garden::vegetables::paprika::seeds::grow();
    /* The absolute might say that the first part is unnecessary,
       it is way cleaner, you will know exactly where it is. */

    /* Relative path */
    paprika::seeds::grow();
}