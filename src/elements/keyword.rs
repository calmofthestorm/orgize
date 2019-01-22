#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Keyword;

impl Keyword {
    // return (key, value, offset)
    pub fn parse(src: &str) -> Option<(&str, &str, usize)> {
        if cfg!(test) {
            starts_with!(src, "#+");
        }

        let key = until_while!(src, 2, b':', |c: u8| c.is_ascii_alphabetic() || c == b'_')?;

        // includes the eol character
        let end = memchr::memchr(b'\n', src.as_bytes())
            .map(|i| i + 1)
            .unwrap_or_else(|| src.len());

        Some((&src[2..key], &src[key + 1..end].trim(), end))
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct AffKeyword<'a> {
    pub key: AffKeywordKey<'a>,
    pub option: Option<&'a str>,
    pub value: &'a str,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum AffKeywordKey<'a> {
    Caption,
    Header,
    Name,
    Plot,
    Results,
    AttrBackend(&'a str),
}

// impl<'a> AffKeyword<'a> {
//     pub fn parse(src: &'a str) -> Option<AffKeyword<'a>> {
//         if src.len() < 3 && !src.starts_with("#+") {
//             return None;
//         }

//         let end = src.nextline();
//         let colon = src[2..end].until(b':');
//         let key_index = src[2..end]
//             .as_bytes()
//             .iter()
//             .position(|&c| !(c.is_ascii_alphanumeric() || c == b'-' || c == b'_'));
//         // .unwrap_or(2);

//         // let key = match parse_key(&src[2..key_index]) {

//         // }

//         // if key.is_none() {
//         //     return None;
//         // }

//         if let Some(key_index) = key {
//             // if src.as_bytes()[key_index] = b':'
//             parse_key(&src[2..key_index])
//                 .filter(|_| src.as_bytes()[colon + 1] == b' ')
//                 .map(|key| {
//                     if src.as_bytes()[key_index + 1] == b'[' && src.as_bytes()[colon - 1] == b']' {
//                         AffKeyword {
//                             key,
//                             value: &s[colon + 2..end],
//                             option: Some(&s[key_index + 2..colon - 1]),
//                         }
//                     } else {
//                         AffKeyword {
//                             key,
//                             value: &s[colon + 2..end],
//                             option: None,
//                         }
//                     }
//                 })
//         } else {
//             None
//         }
//     }
// }

fn parse_key<'a>(key: &'a str) -> Option<AffKeywordKey<'a>> {
    match key {
        "CAPTION" => Some(AffKeywordKey::Caption),
        "HEADER" => Some(AffKeywordKey::Header),
        "NAME" => Some(AffKeywordKey::Name),
        "PLOT" => Some(AffKeywordKey::Plot),
        "RESULTS" => Some(AffKeywordKey::Results),
        k => {
            if k.starts_with("ATTR_")
                && k[5..]
                    .as_bytes()
                    .iter()
                    .all(|&c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
            {
                Some(AffKeywordKey::AttrBackend(&k[5..]))
            } else {
                None
            }
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Keyword::parse("#+KEY:").unwrap(),
        ("KEY", "", "#+KEY:".len())
    );
    assert_eq!(
        Keyword::parse("#+KEY: VALUE").unwrap(),
        ("KEY", "VALUE", "#+KEY: VALUE".len())
    );
    assert_eq!(
        Keyword::parse("#+K_E_Y: VALUE").unwrap(),
        ("K_E_Y", "VALUE", "#+K_E_Y: VALUE".len())
    );
    assert_eq!(
        Keyword::parse("#+KEY:VALUE\n").unwrap(),
        ("KEY", "VALUE", "#+KEY:VALUE\n".len())
    );
    assert!(Keyword::parse("#+KE Y: VALUE").is_none());
    assert!(Keyword::parse("#+ KEY: VALUE").is_none());
    assert!(Keyword::parse("# +KEY: VALUE").is_none());
    assert!(Keyword::parse(" #+KEY: VALUE").is_none());
}

// #[test]
// fn parse_affiliated_keyword() {
//     assert_eq!(AffKeyword::parse("#+KEY: VALUE"), None);
//     assert_eq!(AffKeyword::parse("#+CAPTION: VALUE"), None);
// }
