
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ILibraryAppletAccessor(Session);

impl AsRef<Session> for ILibraryAppletAccessor {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILibraryAppletAccessor {
	pub fn GetAppletStateChangedEvent(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn IsCompleted(&self, ) -> Result<bool> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Start(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RequestExit(&self, ) -> Result<()> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Terminate(&self, ) -> Result<()> {
		let req = Request::new(25)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetResult(&self, ) -> Result<()> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetOutOfFocusApplicationSuspendingEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(50)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PushInData(&self, unk0: &::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(100)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PopOutData(&self, ) -> Result<::nn::am::service::IStorage> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn PushExtraStorage(&self, unk0: &::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(102)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PushInteractiveInData(&self, unk0: &::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(103)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PopInteractiveOutData(&self, ) -> Result<::nn::am::service::IStorage> {
		let req = Request::new(104)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetPopOutDataEvent(&self, ) -> Result<KObject> {
		let req = Request::new(105)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn GetPopInteractiveOutDataEvent(&self, ) -> Result<KObject> {
		let req = Request::new(106)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn NeedsToExitProcess(&self, ) -> Result<bool> {
		let req = Request::new(110)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetLibraryAppletInfo(&self, ) -> Result<::nn::am::service::LibraryAppletInfo> {
		let req = Request::new(120)
			.args(())
			;
		let mut res : Response<::nn::am::service::LibraryAppletInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn RequestForAppletToGetForeground(&self, ) -> Result<()> {
		let req = Request::new(150)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetIndirectLayerConsumerHandle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<u64> {
		let req = Request::new(160)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for ILibraryAppletAccessor {
	unsafe fn from_kobject(obj: KObject) -> ILibraryAppletAccessor {
		ILibraryAppletAccessor(Session::from_kobject(obj))
	}
}
