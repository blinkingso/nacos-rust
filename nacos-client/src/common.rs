use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use nacos_core::error::{NacosError, NacosResult};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GroupKey {
    group_id: String,
    data_id: String,
    tenant: Option<String>,
}

fn require_nonnull(name: &str, value: &str) -> NacosResult<()> {
    if value.trim().is_empty() {
        return Err(NacosError::msg(format!(
            "invalid value {}, please check",
            name
        )));
    }
    Ok(())
}

impl GroupKey {
    /// A function to create a GroupKey without tenant.
    pub fn new_without_tenant(data_id: &str, group_id: &str) -> NacosResult<Self> {
        require_nonnull("data_id", data_id)?;
        require_nonnull("group_id", group_id)?;

        Ok(GroupKey {
            group_id: group_id.to_string(),
            data_id: data_id.to_string(),
            tenant: None,
        })
    }

    /// A function to create a GroupKey.
    pub fn new(data_id: &str, group_id: &str, tenant: &str) -> NacosResult<Self> {
        require_nonnull("data_id", data_id)?;
        require_nonnull("group_id", group_id)?;
        require_nonnull("tenant", tenant)?;
        Ok(GroupKey {
            group_id: group_id.to_string(),
            data_id: data_id.to_string(),
            tenant: Some(tenant.to_string()),
        })
    }
}

impl Display for GroupKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("{}+{}", self.data_id, self.group_id);
        if self.tenant.is_some() {
            result = format!("{}+{}", result, self.tenant.as_ref().unwrap());
        }
        write!(f, "{}", result)
    }
}

impl TryFrom<String> for GroupKey {
    type Error = NacosError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split("+");
        let data_id = if let Some(data) = split.next() {
            data
        } else {
            return Err(NacosError::msg("data id parse error"));
        };
        let group_id = if let Some(data) = split.next() {
            data
        } else {
            return Err(NacosError::msg("group parse error"));
        };

        let tenant = if let Some(t) = split.next() {
            Some(t.to_string())
        } else {
            None
        };

        Ok(GroupKey {
            data_id: data_id.to_string(),
            group_id: group_id.to_string(),
            tenant,
        })
    }
}

#[test]
fn test_group_key() {
    let g1 = GroupKey::new("003", "pay", "dev").unwrap();
    let g2 = GroupKey::new("002", "pay2", "dev").unwrap();
    println!("{}", g1);
    println!("{:?}", g1);
    let mut m = HashMap::new();
    m.insert(g1, "hello groupkey1");
    m.insert(g2, "hello 22222");
    println!("{:?}", m);
}
