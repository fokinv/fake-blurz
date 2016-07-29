use fake_device::FakeBluetoothDevice;
use std::cell::{Cell, RefCell};

#[derive(Clone, Debug)]
pub struct FakeBluetoothGATTService {
    object_path: RefCell<String>,
    device: RefCell<FakeBluetoothDevice>,
    gattCharacteristics: RefCell<Vec<String>>,
    isPrimary: Cell<bool>,
    uuid: RefCell<String>,
}

impl FakeBluetoothGATTService {
    pub fn new(object_path: String,
               device: FakeBluetoothDevice,
               gattCharacteristics: Vec<String>,
               isPrimary: bool,
               uuid: String)
               -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            object_path: RefCell::new(object_path),
            device: RefCell::new(device),
            gattCharacteristics: RefCell::new(gattCharacteristics),
            isPrimary: Cell::new(isPrimary),
            uuid: RefCell::new(uuid),
        }
    }

    pub fn new_empty() -> FakeBluetoothGATTService {
        FakeBluetoothGATTService {
            object_path: RefCell::new(String::new()),
            device: RefCell::new(FakeBluetoothDevice::new_empty()),
            gattCharacteristics: RefCell::new(vec![]),
            isPrimary: Cell::new(false),
            uuid: RefCell::new(String::new()),
        }
    }

    pub fn get_object_path(&self) -> String {
        self.object_path.borrow().clone()
    }

    pub fn set_object_path(&mut self, path: String) {
        *self.object_path.borrow_mut() = path;
    }

    pub fn get_device(&self) -> FakeBluetoothDevice {
        self.device.borrow().clone()
    }

    pub fn set_device(&mut self, device: FakeBluetoothDevice) {
        *self.device.borrow_mut() = device;
    }

    pub fn get_gatt_characteristics(&self) -> Vec<String> {
        self.gattCharacteristics.borrow().clone()
    }

    pub fn set_gatt_characteristics(&mut self, characteristics: Vec<String>) {
        *self.gattCharacteristics.borrow_mut() = characteristics;
    }

    pub fn is_primary(&self) -> bool {
        self.isPrimary.get()
    }

    pub fn set_is_primary(&mut self, value: bool) {
        self.isPrimary.set(value);
    }

    pub fn get_uuid(&self) -> String {
        self.uuid.borrow().clone()
    }

    pub fn set_uuid(&mut self, uuid: String) {
        *self.uuid.borrow_mut() = uuid;
    }
}