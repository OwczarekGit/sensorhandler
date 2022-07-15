use evdev_rs::{DeviceWrapper, InputEvent, TimeVal, UInputDevice, UninitDevice};
use evdev_rs::enums::{BusType, EV_KEY, EV_SYN, EventCode, EventType};
use evdev_rs::enums::BusType::BUS_USB;
use crate::sh::virtual_osu_keyboard::VirtualOsuKeyboard;

pub struct UinputVirtualOsuKeyboard {
    device: Option<UInputDevice>,
    name: String,
    productID: i32,
    vendorID: i32,
}

impl UinputVirtualOsuKeyboard {
    pub fn new(name: &str, productID: i32, vendorID: i32) -> Self {
        let mut dev = UninitDevice::new().unwrap();

        dev.set_name(name);
        dev.set_bustype(BusType::BUS_USB as u16);
        dev.set_vendor_id(vendorID as u16);
        dev.set_product_id(productID as u16);
        dev.enable_event_type(&EventType::EV_KEY).unwrap();
        dev.enable_event_code(&EventCode::EV_KEY(EV_KEY::KEY_Z), None).unwrap();
        dev.enable_event_code(&EventCode::EV_KEY(EV_KEY::KEY_X), None).unwrap();
        dev.enable_event_code(&EventCode::EV_KEY(EV_KEY::KEY_SPACE), None).unwrap();
        dev.enable_event_code(&EventCode::EV_KEY(EV_KEY::KEY_ESC), None).unwrap();
        dev.enable_event_code(&EventCode::EV_KEY(EV_KEY::KEY_F2), None).unwrap();

        let device = match UInputDevice::create_from_device(&dev) {
            Ok(device) => { device }
            Err(_) => {
                panic!("Error creating virtual device.\nMake sure that you have permissions on '/dev/uinput'");
            }
        };

        UinputVirtualOsuKeyboard {
            device: Some(device),
            name: String::from(name),
            productID,
            vendorID,
        }
    }
}

impl VirtualOsuKeyboard for UinputVirtualOsuKeyboard {
    fn set_k1_state(&mut self, state: i32) {
        self.device.as_ref().unwrap().write_event(&InputEvent{
            time: TimeVal::new(0,0),
            event_code: EventCode::EV_KEY(EV_KEY::KEY_Z),
            value: state
        }).unwrap();
    }

    fn set_k2_state(&mut self, state: i32) {
        self.device.as_ref().unwrap().write_event(&InputEvent{
            time: TimeVal::new(0,0),
            event_code: EventCode::EV_KEY(EV_KEY::KEY_X),
            value: state
        }).unwrap();
    }

    fn set_back_state(&mut self, state: i32) {
        self.device.as_ref().unwrap().write_event(&InputEvent{
            time: TimeVal::new(0,0),
            event_code: EventCode::EV_KEY(EV_KEY::KEY_ESC),
            value: state
        }).unwrap();
    }

    fn set_skip_state(&mut self, state: i32) {
        self.device.as_ref().unwrap().write_event(&InputEvent{
            time: TimeVal::new(0,0),
            event_code: EventCode::EV_KEY(EV_KEY::KEY_SPACE),
            value: state
        }).unwrap();
    }

    fn set_random_map_state(&mut self, state: i32) {
        self.device.as_ref().unwrap().write_event(&InputEvent{
            time: TimeVal::new(0,0),
            event_code: EventCode::EV_KEY(EV_KEY::KEY_F2),
            value: state
        }).unwrap();
    }

    fn exec_event(&mut self) {
        self.device.as_ref().unwrap().write_event(&InputEvent{
            time: TimeVal::new(0, 0),
            event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
            value: 0
        }).unwrap();
    }
}