
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IApplicationRootService(Session);

impl IApplicationRootService {
	pub fn get_service() -> Result<IApplicationRootService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"vi:u\0\0\0\0").map(|s| unsafe { IApplicationRootService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IApplicationRootService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IApplicationRootService {
	pub fn GetDisplayService(&self, unk0: u32) -> Result<::nn::visrv::sf::IApplicationDisplayService> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IApplicationRootService {
	unsafe fn from_kobject(obj: KObject) -> IApplicationRootService {
		IApplicationRootService(Session::from_kobject(obj))
	}
}
