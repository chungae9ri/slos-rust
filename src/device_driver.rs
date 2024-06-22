pub mod interface {
    pub trait DeviceDriver {
        unsafe fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }
}

#[derive(Copy, Clone)]
pub struct DeviceDriverDesc {
    pub device_driver: &'static (dyn interface::DeviceDriver + Sync),
}

impl DeviceDriverDesc {
    pub fn new(
        device_driver: &'static (dyn interface::DeviceDriver + Sync),
    ) -> Self {
        DeviceDriverDesc {
            device_driver,
        }
    }
}

const NUM_DRIVERS: usize = 1;

pub struct DriverManager {
    driver_desc: [Option<DeviceDriverDesc>;NUM_DRIVERS],
    driver_desc_cnt: usize 
}

impl DriverManager{
    pub const fn new() -> Self {
        Self {
            driver_desc_cnt: 0,
            driver_desc: [None; NUM_DRIVERS],
        }
    }

    pub fn register_driver_desc(&mut self, descriptor: DeviceDriverDesc) {

        self.driver_desc[self.driver_desc_cnt] = Some(descriptor);
        self.driver_desc_cnt += 1;
    }
}