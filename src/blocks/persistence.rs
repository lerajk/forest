// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::utils::{db::file_backed_obj::FileBackedObject, encoding::from_slice_with_fallback};
use fvm_ipld_encoding::to_vec;

use crate::blocks::*;

impl FileBackedObject for TipsetKeys {
    fn serialize(&self) -> anyhow::Result<Vec<u8>> {
        Ok(to_vec(self)?)
    }

    fn deserialize(bytes: &[u8]) -> anyhow::Result<Self> {
        from_slice_with_fallback(bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::utils::db::file_backed_obj::FileBacked;

    use super::*;

    #[test]
    fn tipset_keys_round_trip() {
        let path = Path::new("src/blocks/tests/calibnet/HEAD");
        let obj1: FileBacked<TipsetKeys> =
            FileBacked::load_from_file_or_create(path.into(), Default::default).unwrap();
        let serialized = obj1.inner().serialize().unwrap();
        let deserialized = TipsetKeys::deserialize(&serialized).unwrap();

        assert_eq!(obj1.inner(), &deserialized);
    }
}
