pub trait VirtualPointer{
    fn moveX(&mut self, value: f32);
    fn moveY(&mut self, value: f32);
    fn pressLMB(&mut self, state: i32);
    fn pressRMB(&mut self, state: i32);
    fn scrollX(&mut self, value: i32);
    fn scrollY(&mut self, value: i32);
    fn execEvent(&mut self);
}