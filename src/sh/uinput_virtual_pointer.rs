use evdev_rs::{DeviceWrapper, InputEvent, TimeVal, UInputDevice, UninitDevice};
use evdev_rs::enums::{BusType, EV_KEY, EV_REL, EV_SYN, EventCode, EventType};
use crate::VirtualPointer;

pub struct UinputVirtualPointer{
    device: Option<UInputDevice>,
    name: String,
    productID: i32,
    vendorID: i32,
    speedMultiplier: f32
}


impl UinputVirtualPointer {
    pub fn new(name: &str, productID: i32, vendorID: i32) -> Self{
        let mut dev = UninitDevice::new().unwrap();

        dev.set_name(name);
        dev.set_bustype(BusType::BUS_USB as u16);
        dev.set_vendor_id(vendorID as u16);
        dev.set_product_id(productID as u16);
        dev.enable_event_type(&EventType::EV_KEY).unwrap();
        dev.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_LEFT), None).unwrap();
        dev.enable_event_code(&EventCode::EV_KEY(EV_KEY::BTN_RIGHT), None).unwrap();
        dev.enable_event_type(&EventType::EV_REL).unwrap();
        dev.enable_event_code(&EventCode::EV_REL(EV_REL::REL_X), None).unwrap();
        dev.enable_event_code(&EventCode::EV_REL(EV_REL::REL_Y), None).unwrap();
        dev.enable_event_code(&EventCode::EV_REL(EV_REL::REL_WHEEL), None).unwrap();

        let device = match UInputDevice::create_from_device(&dev){
            Ok(device) => { device }
            Err(_) => {
                panic!("Error creating virtual device.\nMake sure that you have permissions on '/dev/uinput'");
            }
        };

        UinputVirtualPointer{
            device: Some(device),
            name: String::from(name),
            productID,
            vendorID,
            speedMultiplier: 8.0,
        }
    }
}

impl VirtualPointer for UinputVirtualPointer{
    /// Move the cursor in X axis.
    fn moveX(&mut self, value: f32) {
        self.device.as_ref().unwrap().write_event(&InputEvent {
            time: TimeVal::new(0, 0),
            event_code: EventCode::EV_REL(EV_REL::REL_X),
            value: (value * self.speedMultiplier) as i32
        }).unwrap();
    }

    /// Move the cursor in Y axis.
    fn moveY(&mut self, value: f32) {
        self.device.as_ref().unwrap().write_event(&InputEvent {
            time: TimeVal::new(0, 0),
            event_code: EventCode::EV_REL(EV_REL::REL_Y),
            value: -(value * self.speedMultiplier) as i32
        }).unwrap();
    }

    /// Set the press state for left mouse button.
    fn pressLMB(&mut self, state: i32) {
        self.device.as_ref().unwrap().write_event(&InputEvent{
            time: TimeVal::new(0,0),
            event_code: EventCode::EV_KEY(EV_KEY::BTN_LEFT),
            value: state
        }).unwrap();
    }

    /// Set the press state for right mouse button.
    fn pressRMB(&mut self, state: i32) {
        self.device.as_ref().unwrap().write_event(&InputEvent{
            time: TimeVal::new(0,0),
            event_code: EventCode::EV_KEY(EV_KEY::BTN_RIGHT),
            value: state
        }).unwrap();
    }

    /// Scroll in X axis by.
    fn scrollX(&mut self, value: i32) {
        todo!()
    }

    /// Scroll in Y axis by.
    fn scrollY(&mut self, value: i32) {
        todo!()
    }

    /// Report the events to the virtual device.
    fn execEvent(&mut self) {
        self.device.as_ref().unwrap().write_event(&InputEvent {
            time: TimeVal::new(0, 0),
            event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
            value: 0
        }).unwrap();
    }
}
