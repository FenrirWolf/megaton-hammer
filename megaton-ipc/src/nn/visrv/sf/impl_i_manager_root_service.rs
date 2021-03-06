
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IManagerRootService(Session);

impl IManagerRootService {
	pub fn new() -> Result<IManagerRootService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"vi:m\0\0\0\0").map(|s| unsafe { IManagerRootService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IManagerRootService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManagerRootService {
	pub fn get_display_service(&self, unk0: u32) -> Result<::nn::visrv::sf::IApplicationDisplayService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_display_service_with_proxy_name_exchange(&self, unk0: ::nn::vi::ProxyName, unk1: u32) -> Result<::nn::visrv::sf::IApplicationDisplayService> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::vi::ProxyName,
			unk1: u32,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IManagerRootService {
	unsafe fn from_kobject(obj: KObject) -> IManagerRootService {
		IManagerRootService(Session::from_kobject(obj))
	}
}
