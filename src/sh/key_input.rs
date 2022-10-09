#[test]
fn should_fire_only_second_time(){
    let data1 = 0b0001;
    let data2 = 0b0011;
    let data3 = 0b0000;
    let data4 = 0b1111;
    let mut key = KeyInputU128::new(0, 4);

    assert!(key.get_event(data1).is_some());
    assert!(key.get_event(data2).is_none());
    assert!(key.get_event(data3).is_some());
    assert!(key.get_event(data4).is_some());
}

#[derive(Debug)]
pub struct KeyInputU128<T> {
    mask: u128,
    key: T,
    current_state: u8,
}

impl<T: Clone + Copy> KeyInputU128<T> {
    pub fn new(bit: u8, key: T) -> Self{
        Self{ mask: 0u128 | (1 << bit), key, current_state: 0 }
    }

    pub fn get_event(&mut self, mask: u128) -> Option<(T, i32)>{
        let state = if self.mask & mask > 0 {1} else {0};

        if self.current_state == state {
            return None;
        }

        self.current_state = state;
        return Some((self.key, self.current_state as i32));
    }
}

#[derive(Debug)]
pub struct KeyInputU8<T> {
    mask: u8,
    key: T,
    current_state: u8,
}

impl<T: Clone + Copy> KeyInputU8<T> {
    pub fn new(bit: u8, key: T) -> Self{
        Self{ mask: 0u8 | (1 << bit), key, current_state: 0 }
    }

    pub fn get_event(&mut self, mask: u8) -> Option<(T, i32)>{
        let state = if self.mask & mask > 0 {1} else {0};

        if self.current_state == state {
            return None;
        }

        self.current_state = state;
        return Some((self.key, self.current_state as i32));
    }
}





