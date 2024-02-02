// api 정의
pub mod regist_device;
pub use regist_device::regist_device_controller;

pub mod regist_group;
pub use regist_group::regist_group_controller;

pub mod regist_temperature;
pub use regist_temperature::set_temperature_controller;

pub mod get_stat;
pub use get_stat::get_temperature_device_controller;
pub use get_stat::get_temperature_group_controller;
