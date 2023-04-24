use super::ts::Ts;
use crate::utils::def;
/// # Site struct
/// Site struct is used to store the information of the site.
#[derive(Debug)]
pub struct Site {
    dev_name: String,
    dev_id: String,
    ts: Option<Vec<Ts>>,
}

impl Site {
    pub fn new(dev_name: String, dev_id: String, ts: Option<Vec<Ts>>) -> Self {
        Site {
            dev_name,
            dev_id,
            ts,
        }
    }

    pub fn set_dev_name(&mut self, dev_name: String) {
        self.dev_name = dev_name;
    }

    pub fn dev_name(&self) -> &str {
        self.dev_name.as_ref()
    }

    pub fn dev_id(&self) -> &str {
        self.dev_id.as_ref()
    }

    pub fn set_dev_id(&mut self, dev_id: String) {
        self.dev_id = dev_id;
    }

    pub fn ts(&self) -> Option<&Vec<Ts>> {
        self.ts.as_ref()
    }

    pub fn set_ts(&mut self, ts: Option<Vec<Ts>>) {
        self.ts = ts;
    }
}

/// tranform the site to the site id
pub fn get_site_id(site: String) -> Result<String, Box<dyn std::error::Error>> {
    let _floor = &site[0..4];
    match &site[4..].parse() {
        Ok(_site) => {
            let floor = def::ROOMS.get(_floor);
            match floor {
                Some(floor) => {
                    let id = floor.dev_start() + _site - 1;

                    if id >= floor.dev_start() && id <= floor.dev_end() {
                        Ok(id.to_string())
                    } else {
                        Err("parse room id error".into())
                    }
                }
                None => Err("parse room id error".into()),
            }
        }
        Err(_) => Err("parse site id error".into()),
    }
}