use crate::class_driver::mouse::subscribable::MouseSubscribable;

#[derive(Clone, Debug)]
pub struct MouseSubscriber;

impl MouseSubscriber {
    #[inline(always)]
    pub const fn new() -> Self {
        Self
    }
}

impl MouseSubscribable for MouseSubscriber {
    fn subscribe(
            &self,
            prev_cursor: crate::library::math::vector::Vector2D<usize>,
            current_cursor: crate::library::math::vector::Vector2D<usize>,
            prev_button: Option<crate::class_driver::mouse::MouseButton>,
            button: Option<crate::class_driver::mouse::MouseButton>,
        ) -> () {
            ()
    }
}