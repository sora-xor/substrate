// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::pin::Pin;

use futures::channel::oneshot;
use futures::prelude::*;
use futures::task;

/// Wraps around `oneshot::Receiver` and adjusts the error type to produce an internal error if the
/// sender gets dropped.
pub struct Receiver<T>(pub oneshot::Receiver<T>);

impl<T> Future for Receiver<T> {
	type Output = jsonrpc_core::Result<T>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
		Pin::new(&mut self.0)
			.poll(cx)
			.map_err(|_| jsonrpc_core::Error::internal_error())
	}
}
impl<T: Send + 'static> jsonrpc_core::WrapFuture<T, jsonrpc_core::Error> for Receiver<T> {
	fn into_future(self) -> jsonrpc_core::BoxFuture<jsonrpc_core::Result<T>> {
		Box::pin(self)
	}
}
