/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use super::ResponseMethod;
use crate::request::Call;
use serde::{Serialize, ser::SerializeSeq};

impl Serialize for Call<ResponseMethod<'_>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(3.into())?;
        seq.serialize_element(&self.name.to_string())?;
        seq.serialize_element(&self.method)?;
        seq.serialize_element(&self.id)?;
        seq.end()
    }
}

pub fn serialize_hex<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    format!("{:x}", value).serialize(serializer)
}
