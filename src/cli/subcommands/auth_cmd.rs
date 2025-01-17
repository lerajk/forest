// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::auth::*;
use crate::libp2p::{Multiaddr, Protocol};
use crate::rpc_api::auth_api::AuthNewParams;
use crate::rpc_client::auth_new;
use clap::Subcommand;
use jsonrpc_v2::Error as JsonRpcError;

use super::{handle_rpc_err, print_rpc_res_bytes, Config};

#[derive(Debug, Subcommand)]
pub enum AuthCommands {
    /// Create a new Authentication token with given permission
    CreateToken {
        /// permission to assign to the token, one of: read, write, sign, admin
        #[arg(short, long)]
        perm: String,
    },
    /// Get RPC API Information
    ApiInfo {
        /// permission to assign the token, one of: read, write, sign, admin
        #[arg(short, long)]
        perm: String,
    },
}

fn process_perms(perm: String) -> Result<Vec<String>, JsonRpcError> {
    Ok(match perm.as_str() {
        "admin" => ADMIN,
        "sign" => SIGN,
        "write" => WRITE,
        "read" => READ,
        _ => return Err(JsonRpcError::INVALID_PARAMS),
    }
    .iter()
    .map(ToString::to_string)
    .collect())
}

impl AuthCommands {
    pub async fn run(self, config: Config) -> anyhow::Result<()> {
        match self {
            Self::CreateToken { perm } => {
                let perm: String = perm.parse()?;
                let perms = process_perms(perm).map_err(handle_rpc_err)?;
                let token_exp = config.client.token_exp;
                let auth_params = AuthNewParams { perms, token_exp };
                print_rpc_res_bytes(auth_new(auth_params, &config.client.rpc_token).await)
            }
            Self::ApiInfo { perm } => {
                let perm: String = perm.parse()?;
                let perms = process_perms(perm).map_err(handle_rpc_err)?;
                let token_exp = config.client.token_exp;
                let auth_params = AuthNewParams { perms, token_exp };
                let token = auth_new(auth_params, &config.client.rpc_token)
                    .await
                    .map_err(handle_rpc_err)?;
                let mut addr = Multiaddr::empty();
                addr.push(config.client.rpc_address.ip().into());
                addr.push(Protocol::Tcp(config.client.rpc_address.port()));
                addr.push(Protocol::Http);
                println!(
                    "FULLNODE_API_INFO=\"{}:{}\"",
                    String::from_utf8(token).map_err(|e| handle_rpc_err(e.into()))?,
                    addr
                );
                Ok(())
            }
        }
    }
}
