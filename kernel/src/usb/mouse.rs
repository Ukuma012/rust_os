use crate::class_driver::mouse::subscribable::MouseSubscribable;
use crate::class_driver::mouse::MouseButton;
use crate::library::math::vector::Vector2D;

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
            update_cursor(current_cursor, button);
    }
}

fn update_cursor(current_cursor: Vector2D<usize>, button: Option<MouseButton>) {
    unimplemented!()
}