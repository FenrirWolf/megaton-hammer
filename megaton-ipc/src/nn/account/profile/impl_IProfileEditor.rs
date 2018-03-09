
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IProfileEditor(Session);

impl AsRef<Session> for IProfileEditor {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IProfileEditor {
	// fn Get(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetBase(&self, ) -> Result<::nn::account::profile::ProfileBase> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::account::profile::ProfileBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetImageSize(&self, ) -> Result<u32> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn LoadImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Store(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn StoreWithImage(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IProfileEditor {
	unsafe fn from_kobject(obj: KObject) -> IProfileEditor {
		IProfileEditor(Session::from_kobject(obj))
	}
}
