use blight::{Device, Direction, Delay};

use super::{BrightnessBackend, BrightnessBackendConstructor};

pub(super) struct Blight {
	device: Device,
}

impl BrightnessBackendConstructor for Blight {
	fn try_new(device_name: Option<String>) -> anyhow::Result<Self> {
		Ok(Self {
			device: Device::new(device_name.map(Into::into))?,
		})
	}
}

impl BrightnessBackend for Blight {
	fn get_current(&mut self) -> u32 {
		self.device.current()
	}

	fn get_max(&mut self) -> u32 {
		self.device.max()
	}

	fn lower(&mut self, by: u32) -> anyhow::Result<()> {
		let val = self.device.calculate_change(by, Direction::Dec);
		Ok(self.device.write_value(val)?)
	}

	fn lower_gradual(&mut self, by: u32) -> anyhow::Result<()> {
		let val = self.device.calculate_change(by, Direction::Dec);
		Ok(self.device.sweep_write(val, Delay::default())?)
	}

	fn raise(&mut self, by: u32) -> anyhow::Result<()> {
		let val = self.device.calculate_change(by, Direction::Inc);
		Ok(self.device.write_value(val)?)
	}

	fn raise_gradual(&mut self, by: u32) -> anyhow::Result<()> {
		let val = self.device.calculate_change(by, Direction::Inc);
		Ok(self.device.sweep_write(val, Delay::default())?)
	}

	fn set(&mut self, val: u32) -> anyhow::Result<()> {
		Ok(self.device.write_value(val)?)
	}

	fn set_gradual(&mut self, val: u32) -> anyhow::Result<()> {
		Ok(self.device.sweep_write(val, Delay::default())?)
	}
}
