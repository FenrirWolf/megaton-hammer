
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IHardwareOpusDecoderManager(Session);

impl IHardwareOpusDecoderManager {
	pub fn new() -> Result<IHardwareOpusDecoderManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"hwopus\0\0").map(|s| unsafe { IHardwareOpusDecoderManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IHardwareOpusDecoderManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHardwareOpusDecoderManager {
	pub fn unknown0(&self, unk0: u64, unk1: u32, unk2: &KObject) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn unknown1(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IHardwareOpusDecoderManager {
	unsafe fn from_kobject(obj: KObject) -> IHardwareOpusDecoderManager {
		IHardwareOpusDecoderManager(Session::from_kobject(obj))
	}
}
