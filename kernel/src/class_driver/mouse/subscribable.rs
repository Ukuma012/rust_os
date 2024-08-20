use super::MouseButton;
use crate::library::math::vector::Vector2D;

pub trait MouseSubscribable {
    fn subscribe(
        &self,
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        prev_button: Option<MouseButton>,
        button: Option<MouseButton>,
    ) -> ();
}

impl<T> MouseSubscribable for T
where 
    T: Fn(
        Vector2D<usize>,
        Vector2D<usize>,
        Option<MouseButton>,
        Option<MouseButton>,
    ) -> ()
{
    fn subscribe(
            &self,
            prev_cursor: Vector2D<usize>,
            current_cursor: Vector2D<usize>,
            prev_button: Option<MouseButton>,
            button: Option<MouseButton>,
        ) -> () {
        self (
            prev_cursor,
            current_cursor,
            prev_button,
            button
        )
    }
}