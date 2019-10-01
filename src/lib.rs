// serde-strings
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an AS IS BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Display;
use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// Wraps a value Display and/or FromStr value to be used as a field in a derive(Serialize) or
/// derive(Deserialize) struct/enum.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct SerdeStr<T> {
    /// The inner value
    pub value: T,
}

impl<T> SerdeStr<T> {
    /// Gets a reference of the inner value.
    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Gets a mutable reference of the inner value.
    #[inline]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Sets the inner value.
    #[inline]
    pub fn set_value(&mut self, t: T) {
        self.value = t;
    }

    /// Moves out the inner value.
    #[inline]
    pub fn unwrap(self) -> T {
        self.value
    }
}

/// Creates a SerdeStr from its inner value.
impl<T> From<T> for SerdeStr<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self { value }
    }
}

impl<'de, T: FromStr> Deserialize<'de> for SerdeStr<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    #[inline]
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            value: String::deserialize(de)?
                .parse()
                .map_err(|err| Error::custom(err))?,
        })
    }
}

impl<T> Serialize for SerdeStr<T>
where
    T: Display,
{
    #[inline]
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.to_string().serialize(ser)
    }
}

#[cfg(test)]
mod test_de {
    use std::collections::HashMap;

    use serde_derive::Deserialize;

    use crate::SerdeStr;

    #[derive(Debug, PartialEq)]
    struct IsParsed(i32);

    impl std::str::FromStr for IsParsed {
        type Err = &'static str;

        fn from_str(str: &str) -> Result<Self, Self::Err> {
            Ok(IsParsed(str.len() as i32))
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Schema {
        data: SerdeStr<IsParsed>,
    }

    #[test]
    fn test_parse() {
        let mut map = HashMap::new();
        map.insert("data", "abc");
        let json = serde_json::to_string(&map).unwrap();
        let parsed = serde_json::from_str::<Schema>(&json);
        assert_eq!(
            parsed.unwrap(),
            Schema {
                data: SerdeStr { value: IsParsed(3) }
            }
        );
    }
}

#[cfg(test)]
mod test_ser {
    use std::fmt::{Display, Formatter, Result};

    use serde_derive::Serialize;

    use crate::SerdeStr;

    #[derive(Debug)]
    struct CanDisplay(&'static str);

    impl Display for CanDisplay {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{}", self.0.len())
        }
    }

    #[derive(Debug, Serialize)]
    struct Schema {
        data: SerdeStr<CanDisplay>,
    }

    #[test]
    fn test_display() {
        let json = serde_json::to_string(&Schema {
            data: SerdeStr {
                value: CanDisplay("abc"),
            },
        });
        assert_eq!(json.unwrap(), r##"{"data":"3"}"##);
    }
}
