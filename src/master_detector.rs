use rustc_serialize::json;
use zookeeper::{Watcher, WatchedEvent, ZooKeeper};
use std::time::Duration;

static ZK_SESSION_TIMEOUT: u64 = 10000;
static MASTER_INFO_JSON_LABEL: &'static str = "json.info";

#[derive(RustcDecodable)]
pub struct MasterInfo {
    pid: String
}

impl MasterInfo {
    pub fn address(&self) -> &str {
        self.pid.split('@').last().unwrap()
    }
}

struct ZkMasterDetectorWatcher;

impl Watcher for ZkMasterDetectorWatcher {
    fn handle(&self, e: &WatchedEvent) {
        debug!("{:?}", e);
    }
}

pub trait MasterDetector {
    fn get_master(&self) -> Result<MasterInfo, String>;
}

pub struct ZkMasterDetector {
    zk: ZooKeeper
}

impl ZkMasterDetector {
    pub fn new(connect_string: &str) -> Result<ZkMasterDetector, String> {
        let zk = try!(ZooKeeper::connect(connect_string,
                                         Duration::from_secs(ZK_SESSION_TIMEOUT),
                                         ZkMasterDetectorWatcher).map_err(|_| "connect".to_string()));
        Ok(ZkMasterDetector{zk: zk})
    }
}

impl MasterDetector for ZkMasterDetector {

    fn get_master(&self) -> Result<MasterInfo, String> {
        let children = try!(self.zk.get_children("/", true).map_err(|_| "get_children".to_string()));

        let mut contenders: Vec<&String> = children
                                            .iter()
                                            .filter(|child| child.starts_with(MASTER_INFO_JSON_LABEL))
                                            .collect();
        contenders.sort();

        debug!("Contenders: {:?}", contenders);

        match contenders.first() {
            Some(leader) => {
                let leader_path = "/".to_string() + leader;
                let (data, _acl) = try!(self.zk.get_data(&leader_path, true).map_err(|_| "get_children".to_string()));
                match String::from_utf8(data) {
                    Ok(json) => match json::decode(&json) {
                        Ok(master_info) => Ok(master_info),
                        Err(_) => Err("ZkError::MarshallingError".to_string())
                    },
                    Err(_) => Err("ZkError::MarshallingError".to_string())
                }
            }
            None => Err("ZkError::NoNode".to_string())
        }
    }
}