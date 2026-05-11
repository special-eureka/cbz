#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::comicinfo::comic_page_type::ComicPageType;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct TestStruct {
        #[serde(rename = "@ComicPageType")]
        comic_page_type: Vec<ComicPageType>,
    }

    #[test]
    fn test_list_xml_ser() -> anyhow::Result<()> {
        let val = serde_xml_rs::to_string(&TestStruct {
            comic_page_type: vec![ComicPageType::Story, ComicPageType::Other],
        })?;
        assert_eq!(
            val.as_str(),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><TestStruct ComicPageType=\"Story Other\" />"
        );
        Ok(())
    }
    #[test]
    fn test_list_xml_ser_empty() -> anyhow::Result<()> {
        let val = serde_xml_rs::to_string(&TestStruct {
            comic_page_type: vec![],
        })?;
        assert_eq!(
            val.as_str(),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><TestStruct ComicPageType=\"\" />"
        );
        Ok(())
    }
    #[test]
    fn test_list_xml_deser() -> anyhow::Result<()> {
        let val: TestStruct = serde_xml_rs::from_str(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><TestStruct ComicPageType=\"Story Other\" />",
        )?;
        assert_eq!(
            val,
            TestStruct {
                comic_page_type: vec![ComicPageType::Story, ComicPageType::Other],
            }
        );
        Ok(())
    }
    #[test]
    fn test_list_xml_deser_empty() -> anyhow::Result<()> {
        let val: TestStruct = serde_xml_rs::from_str(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><TestStruct ComicPageType=\"\" />",
        )?;
        assert_eq!(
            val,
            TestStruct {
                comic_page_type: vec![],
            }
        );
        Ok(())
    }
}
