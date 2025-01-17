// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::rpc_api::db_api::*;
use jsonrpc_v2::Error;

use crate::rpc_client::call;

pub async fn db_gc((): DBGCParams, auth_token: &Option<String>) -> Result<DBGCResult, Error> {
    call(DB_GC, (), auth_token).await
}
