
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IFirmwareDebugSettingsServer(Session);

impl IFirmwareDebugSettingsServer {
	pub fn new() -> Result<IFirmwareDebugSettingsServer> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"set:fd\0\0").map(|s| unsafe { IFirmwareDebugSettingsServer::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFirmwareDebugSettingsServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFirmwareDebugSettingsServer {
	// fn set_settings_item_value(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn reset_settings_item_value(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_settings_item_key_iterator(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-4.0.0")]
	pub fn read_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn reset_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_web_inspector_flag(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_allowed_ssl_hosts(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_host_fs_mount_point(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IFirmwareDebugSettingsServer {
	unsafe fn from_kobject(obj: KObject) -> IFirmwareDebugSettingsServer {
		IFirmwareDebugSettingsServer(Session::from_kobject(obj))
	}
}
