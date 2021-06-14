// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
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

use super::*;
use assert_matches::assert_matches;
use substrate_test_runtime_client::{
	prelude::*,
	sp_consensus::BlockOrigin,
	runtime::{H256, Block, Header},
};
use sp_rpc::list::ListOrValue;
use sc_block_builder::BlockBuilderProvider;
use crate::testing::{TaskExecutor, Chain};
use jsonrpsee::{
	ws_server::WsServerBuilder,
	ws_client::{WsClient, WsClientBuilder},
	types::traits::SubscriptionClient,
};

async fn connect_client_chain_api(test_node: Arc<TestClient>) -> WsClient {
	let chain_api = new_full(
		test_node,
		Arc::new(SubscriptionTaskExecutor::new(TaskExecutor))
	)
	.into_rpc_module().unwrap();

	let mut server = WsServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	server.register_module(chain_api).unwrap();
	let addr = format!("ws://{}", server.local_addr().unwrap());

	tokio::spawn(Box::pin(server.start()));

	WsClientBuilder::default().build(&addr).await.unwrap()
}

#[tokio::test]
async fn should_return_header() {
	let test_node = Arc::new(substrate_test_runtime_client::new());
	let rpc_client = connect_client_chain_api(test_node.clone()).await;

	let h1 = Chain::header(&rpc_client, Some(test_node.genesis_hash())).await.unwrap();
	let h2 = Chain::header(&rpc_client, None).await.unwrap();

	let exp = Header {
		parent_hash: H256::from_low_u64_be(0),
		number: 0,
		state_root: h1.state_root.clone(),
		extrinsics_root:
			"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314".parse().unwrap(),
		digest: Default::default(),
	};

	assert_eq!(h1, exp);
	assert_eq!(h2, exp);

	assert!(Chain::header(&rpc_client, Some(H256::from_low_u64_be(5))).await.is_err());
}

#[tokio::test]
async fn should_return_a_block() {
	let mut backend = Arc::new(substrate_test_runtime_client::new());
	let rpc_client = connect_client_chain_api(backend.clone()).await;

	let block = backend.new_block(Default::default()).unwrap().build().unwrap().block;
	let block_hash = block.hash();
	backend.import(BlockOrigin::Own, block).await.unwrap();

	// Genesis block is not justified
	assert_matches!(
		Chain::block(&rpc_client, Some(backend.genesis_hash())).await,
		Ok(SignedBlock { justifications: None, .. })
	);

	assert_matches!(
		Chain::block(&rpc_client, Some(block_hash)).await,
		Ok(x) if x.block == Block {
			header: Header {
				parent_hash: backend.genesis_hash(),
				number: 1,
				state_root: x.block.header.state_root.clone(),
				extrinsics_root:
					"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314".parse().unwrap(),
				digest: Default::default(),
			},
			extrinsics: vec![],
		}
	);

	assert_matches!(
		Chain::block(&rpc_client, None).await,
		Ok(x) if x.block == Block {
			header: Header {
				parent_hash: backend.genesis_hash(),
				number: 1,
				state_root: x.block.header.state_root.clone(),
				extrinsics_root:
					"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314".parse().unwrap(),
				digest: Default::default(),
			},
			extrinsics: vec![],
		}
	);

	assert!(Chain::block(&rpc_client, Some(H256::from_low_u64_be(5))).await.is_err());
}

#[tokio::test]
async fn should_return_block_hash() {
	let mut backend = Arc::new(substrate_test_runtime_client::new());
	let rpc_client = connect_client_chain_api(backend.clone()).await;

	assert_matches!(
		Chain::block_hash(&rpc_client, None).await,
		Ok(ListOrValue::Value(Some(ref x))) if x == &backend.genesis_hash()
	);

	assert_matches!(
		Chain::block_hash(&rpc_client, Some(ListOrValue::Value(0_u64.into()))).await,
		Ok(ListOrValue::Value(Some(ref x))) if x == &backend.genesis_hash()
	);

	assert_matches!(
		Chain::block_hash(&rpc_client, Some(ListOrValue::Value(1_u64.into()))).await,
		Ok(ListOrValue::Value(None))
	);

	let block = backend.new_block(Default::default()).unwrap().build().unwrap().block;
	backend.import(BlockOrigin::Own, block.clone()).await.unwrap();

	assert_matches!(
		Chain::block_hash(&rpc_client, Some(ListOrValue::Value(0_u64.into()))).await,
		Ok(ListOrValue::Value(Some(ref x))) if x == &backend.genesis_hash()
	);
	assert_matches!(
		Chain::block_hash(&rpc_client, Some(ListOrValue::Value(1_u64.into()))).await,
		Ok(ListOrValue::Value(Some(ref x))) if x == &block.hash()
	);
	assert_matches!(
		Chain::block_hash(&rpc_client, Some(ListOrValue::Value(sp_core::U256::from(1u64).into()))).await,
		Ok(ListOrValue::Value(Some(ref x))) if x == &block.hash()
	);

	assert_matches!(
		Chain::block_hash(&rpc_client, Some(vec![0u64.into(), 1u64.into(), 2u64.into()].into())).await,
		Ok(ListOrValue::List(list)) if list == &[backend.genesis_hash().into(), block.hash().into(), None]
	);
}

#[tokio::test]
async fn should_return_finalized_hash() {
	let mut backend = Arc::new(substrate_test_runtime_client::new());
	let rpc_client = connect_client_chain_api(backend.clone()).await;

	assert_matches!(
		Chain::finalized_head(&rpc_client).await,
		Ok(ref x) if x == &backend.genesis_hash()
	);

	// import new block
	let block = backend.new_block(Default::default()).unwrap().build().unwrap().block;
	backend.import(BlockOrigin::Own, block).await.unwrap();
	// no finalization yet
	assert_matches!(
		Chain::finalized_head(&rpc_client).await,
		Ok(ref x) if x == &backend.genesis_hash()
	);

	// finalize
	backend.finalize_block(BlockId::number(1), None).unwrap();
	assert_matches!(
		Chain::finalized_head(&rpc_client).await,
		Ok(ref x) if x == &backend.block_hash(1).unwrap().unwrap()
	);
}

#[tokio::test]
async fn should_notify_about_best_block() {
	let mut new_heads = {
		let mut backend = Arc::new(substrate_test_runtime_client::new());
		let rpc_client = connect_client_chain_api(backend.clone()).await;
		let new_heads = rpc_client.subscribe::<Header>(
			"chain_subscribeNewHead",
			vec![].into(),
			"chain_unsubscribeNewHead"
		).await.unwrap();
		let block = backend.new_block(Default::default()).unwrap().build().unwrap().block;
		backend.import(BlockOrigin::Own, block).await.unwrap();
		new_heads
	};

	// assert initial head sent.
	let h1 = new_heads.next().await.unwrap();
	assert!(h1.is_some());

	// assert new block head sent.
	let h2 = new_heads.next().await.unwrap();
	assert!(h2.is_some());

	// TODO(niklasad1): no way stop the server here and it won't terminate the stream then.
	// assert_matches!(new_heads.next().await, Ok(None));
}

#[tokio::test]
async fn should_notify_about_finalized_block() {
	let mut finalized_heads = {
		let mut backend = Arc::new(substrate_test_runtime_client::new());
		let rpc_client = connect_client_chain_api(backend.clone()).await;
		let new_heads = rpc_client.subscribe::<Header>(
			"chain_subscribeFinalizedHeads",
			vec![].into(),
			"chain_unsubscribeFinalizedHeads"
		).await.unwrap();
		let block = backend.new_block(Default::default()).unwrap().build().unwrap().block;
		backend.import(BlockOrigin::Own, block).await.unwrap();
		backend.finalize_block(BlockId::number(1), None).unwrap();
		new_heads
	};

	// assert initial head sent.
	let h1 = finalized_heads.next().await.unwrap();
	assert!(h1.is_some());

	// assert new block head sent.
	let h2 = finalized_heads.next().await.unwrap();
	assert!(h2.is_some());

	// TODO(niklasad1): no way stop the server here and it won't terminate the stream then.
	// assert_matches!(new_heads.next().await, Ok(None));
}
