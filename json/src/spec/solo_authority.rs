// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use super::super::hash::Address;

/// Authority params deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct SoloAuthorityParams {
    /// Valid authorities
    pub validators: Vec<Address>,
}

/// Authority engine deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct SoloAuthority {
    pub params: SoloAuthorityParams,
}

#[cfg(test)]
mod tests {
    use ctypes::H160;
    use serde_json;

    use super::SoloAuthority;
    use super::super::super::hash::Address;

    #[test]
    fn basic_authority_deserialization() {
        let s = r#"{
			"params": {
				"durationLimit": "0x0d",
				"validators" : ["0xc6d9d2cd449a754c494264e1809c50e34d64562b"]
			}
		}"#;

        let deserialized: SoloAuthority = serde_json::from_str(s).unwrap();

        let vs = vec![Address(H160::from("0xc6d9d2cd449a754c494264e1809c50e34d64562b"))];
        assert_eq!(deserialized.params.validators, vs);
    }
}

