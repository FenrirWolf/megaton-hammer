
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IArbitrationManager(Session);

impl IArbitrationManager {
	pub fn new() -> Result<IArbitrationManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"pcv:arb\0").map(|s| unsafe { IArbitrationManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IArbitrationManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IArbitrationManager {
	pub fn release_control(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IArbitrationManager {
	unsafe fn from_kobject(obj: KObject) -> IArbitrationManager {
		IArbitrationManager(Session::from_kobject(obj))
	}
}
