
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IManager(Session);

impl IManager {
	pub fn new() -> Result<IManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"nsd:a\0\0\0").map(|s| unsafe { IManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"nsd:u\0\0\0").map(|s| unsafe { IManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManager {
	// fn get_setting_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_environment_identifier(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_device_id(&self, ) -> Result<u128> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn delete_settings(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn import_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn resolve(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn resolve_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nas_service_setting(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nas_service_setting_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nas_request_fqdn(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nas_request_fqdn_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nas_api_fqdn(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nas_api_fqdn_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_current_setting(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_save_data_from_fs_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_save_data_to_fs_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn delete_save_data_of_fs_for_test(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(62)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IManager {
	unsafe fn from_kobject(obj: KObject) -> IManager {
		IManager(Session::from_kobject(obj))
	}
}
