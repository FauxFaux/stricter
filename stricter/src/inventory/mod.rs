use std::collections::HashMap;
use std::str::FromStr;

use failure::err_msg;
use failure::format_err;
use failure::Error;

use super::connector::Connector;

#[derive(Debug, Clone, Deserialize)]
pub struct HostTemplate {
    pub connectors: Vec<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct InventoryTemplate {
    pub hosts: Vec<HostTemplate>,
}

pub fn load() -> Result<InventoryTemplate, Error> {
    Ok(InventoryTemplate {
        hosts: vec![HostTemplate {
            connectors: vec!["docker-test://ubuntu/18.04".to_string()],
            tags: maplit::hashmap! {
                "name".to_string() => "potato".to_string(),
            },
        }],
    })
}

pub fn pick_connector(host: &HostTemplate) -> Result<Connector, Error> {
    let a_url = host
        .connectors
        .iter()
        .next()
        .ok_or_else(|| err_msg("no connectors"))?;

    let url = url::Url::from_str(a_url)?;
    Ok(match url.scheme() {
        "docker-test" => Connector::DockerTest {
            base_image: format!(
                "{}:{}",
                url.host_str()
                    .ok_or_else(|| format_err!("no host in: {:?}", url))?,
                // leading slash
                &url.path()[1..],
            ),
        },
        other => Err(format_err!("bad scheme: {:?}", other))?,
    })
}
