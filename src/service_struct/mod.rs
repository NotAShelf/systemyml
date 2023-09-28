use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceConfig {
    pub my_service: MyService,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyService {
    pub name: String,
    pub install: Option<HashMap<String, String>>,
    pub unit: Option<HashMap<String, String>>,
    pub service: Option<ServiceProperties>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceProperties {
    pub Type: String,
    pub ExecStart: String,
    pub Restart: String,
    pub User: String,
    pub Group: String,
    pub WorkingDirectory: String,
    pub Nice: i32,
    pub UMask: String,
    pub RestartSec: String,
    pub TimeoutStartSec: String,
    pub TimeoutStopSec: String,
    pub SuccessExitStatus: String,
    pub LimitNOFILE: i32,
    pub LimitCORE: String,
    pub LimitAS: String,
    pub LimitCPU: String,
    pub LimitFSIZE: String,
    pub LimitNPROC: i32,
    pub LimitSTACK: String,
    pub LimitRTPRIO: i32,
    pub CPUSet: String,
    pub Delegate: String,
    pub ProtectHome: String,
    pub ProtectSystem: String,
    pub PrivateTmp: String,
    pub PrivateDevices: String,
    pub ProtectKernelModules: String,
    pub ProtectKernelTunables: String,
    pub ProtectControlGroups: String,
    pub MemoryDenyGroup: String,
    pub IOWeight: i32,
    pub IOWeightDevice: Vec<String>,
    pub BlockIOWeight: i32,
    pub BlockIOWeightDevice: Vec<String>,
    pub TasksMax: i32,
    pub Slice: String,
    pub NUMANode: i32,
    pub NUMAAffinity: Vec<i32>,
}
