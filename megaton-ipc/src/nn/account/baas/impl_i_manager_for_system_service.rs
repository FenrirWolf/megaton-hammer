
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IManagerForSystemService(Session);

impl AsRef<Session> for IManagerForSystemService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManagerForSystemService {
	pub fn check_availability(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_account_id(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ensure_id_token_cache_async(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn load_id_token_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_system_program_identification(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_nintendo_account_id(&self, ) -> Result<::nn::account::NintendoAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(120)
			.args(())
			;
		let res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_nintendo_account_user_resource_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn refresh_nintendo_account_user_resource_cache_async(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(131)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn refresh_nintendo_account_user_resource_cache_async_if_seconds_elapsed(&self, unk0: u32) -> Result<(bool, ::nn::account::detail::IAsyncContext)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(132)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}

	// fn create_authorization_request(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IManagerForSystemService {
	unsafe fn from_kobject(obj: KObject) -> IManagerForSystemService {
		IManagerForSystemService(Session::from_kobject(obj))
	}
}
