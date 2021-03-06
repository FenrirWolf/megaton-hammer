
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISessionObject(Session);

impl AsRef<Session> for ISessionObject {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISessionObject {
	pub fn dummy(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(999)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISessionObject {
	unsafe fn from_kobject(obj: KObject) -> ISessionObject {
		ISessionObject(Session::from_kobject(obj))
	}
}
