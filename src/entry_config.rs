use std::collections::HashMap;

use proc_macro2::token_stream::IntoIter;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree::*;

macro_rules! create_EntryConfig_from_map {
    ($source_map: ident, [$($name: ident), +]) => {
        {
            let default = EntryConfig::default();

            EntryConfig {
                $($name: $source_map
                    .get(stringify!($name))
                    .map(|s| s.to_string())
                    .get_or_insert(default.$name)
                    .to_string(),)+
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntryConfig {
    pub filename: String,
    another: String,
}

impl EntryConfig {
    fn default() -> EntryConfig {
        EntryConfig {
            filename: "route.yml".to_string(),
            another: "".to_string(),
        }
    }
    pub fn extract(entry: TokenStream) -> EntryConfig {
        let extractor = Extractor(entry.into_iter());
        let map_config: HashMap<String, String> = HashMap::from_iter(extractor);

        create_EntryConfig_from_map!(map_config, [filename, another])
    }
}
struct Extractor(IntoIter);

impl Iterator for Extractor {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.0.next(), self.0.next(), self.0.next(), self.0.next()) {
            (Some(Ident(key)), Some(Punct(sep)), Some(Literal(value)), p)
                if sep.to_string() == ":"
                    && p.clone().map(|x| x.to_string() == ",").unwrap_or(true) =>
            {
                Some((
                    key.to_string(),
                    value
                        .to_string()
                        .trim_start_matches('"')
                        .trim_end_matches('"')
                        .to_string(),
                ))
            }
            (None, None, None, None) => None,
            _ => panic!("canot parse entry"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn empty_entry_should_return_default_entry_config() {
        let entry = proc_macro2::TokenStream::from_str("").unwrap();

        assert_eq!(EntryConfig::extract(entry), EntryConfig::default());
    }

    #[test]
    fn set_file_should_return_entry_config() {
        let entry = proc_macro2::TokenStream::from_str("filename: \"filename.yml\"").unwrap();

        assert_eq!(
            EntryConfig::extract(entry),
            EntryConfig {
                filename: "filename.yml".to_string(),
                ..EntryConfig::default()
            }
        );
    }
}
