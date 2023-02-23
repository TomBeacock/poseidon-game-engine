use sdl2::event::Event;

pub trait Layer {
    /// Opportunity to handle an event.
    /// 
    /// Return true to end event propagation.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The event to handle
    fn on_event(&mut self, event: &Event) -> bool;

    /// Called once each frame
    fn on_update(&mut self);
}