use crate::api::conn::Method;
use crate::api::conn::Param;
use crate::api::conn::Router;
use crate::api::opt::auth::Jwt;
use crate::api::Connection;
use crate::api::Result;
use crate::sql::Value;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::future::IntoFuture;
use std::marker::PhantomData;
use std::pin::Pin;

/// An authentication future
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Authenticate<'r, C: Connection, R> {
	pub(super) router: Result<&'r Router<C>>,
	pub(super) token: Jwt,
	pub(super) credentials: Result<Value>,
	pub(super) response_type: PhantomData<R>,
}

impl<'r, Client, R> IntoFuture for Authenticate<'r, Client, R>
where
	Client: Connection,
	R: DeserializeOwned,
{
	type Output = Result<()>;
	type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync + 'r>>;

	fn into_future(self) -> Self::IntoFuture {
		Box::pin(async move {
			let router = self.router?;
			let mut conn = Client::new(Method::Authenticate);
			conn.execute(router, Param::new(vec![self.token.0.into(),self.credentials?])).await
		})
	}
}
