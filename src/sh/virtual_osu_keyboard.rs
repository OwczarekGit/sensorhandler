pub trait VirtualOsuKeyboard {
    fn set_k1_state(&mut self, state: i32);
    fn set_k2_state(&mut self, state: i32);
    fn set_back_state(&mut self, state: i32);
    fn set_skip_state(&mut self, state: i32);
    fn set_random_map_state(&mut self, state: i32);
    fn exec_event(&mut self);
}