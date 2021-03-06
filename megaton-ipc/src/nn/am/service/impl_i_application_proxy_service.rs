
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IApplicationProxyService(Session);

impl IApplicationProxyService {
	pub fn new() -> Result<IApplicationProxyService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"appletOE").map(|s| unsafe { IApplicationProxyService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IApplicationProxyService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IApplicationProxyService {
	pub fn open_application_proxy(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::IApplicationProxy> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IApplicationProxyService {
	unsafe fn from_kobject(obj: KObject) -> IApplicationProxyService {
		IApplicationProxyService(Session::from_kobject(obj))
	}
}
