
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IParentalControlServiceFactory(Session);

impl IParentalControlServiceFactory {
	pub fn new() -> Result<IParentalControlServiceFactory> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"pctl:s\0\0").map(|s| unsafe { IParentalControlServiceFactory::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"pctl:r\0\0").map(|s| unsafe { IParentalControlServiceFactory::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"pctl:a\0\0").map(|s| unsafe { IParentalControlServiceFactory::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"pctl\0\0\0\0").map(|s| unsafe { IParentalControlServiceFactory::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IParentalControlServiceFactory {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IParentalControlServiceFactory {
	pub fn get_service(&self, unk0: u64) -> Result<::nn::pctl::detail::ipc::IParentalControlService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IParentalControlServiceFactory {
	unsafe fn from_kobject(obj: KObject) -> IParentalControlServiceFactory {
		IParentalControlServiceFactory(Session::from_kobject(obj))
	}
}
