use anyhow::Result;
use connector::UdpConnector;
use ds::*;
use example::*;
use hashbrown::HashMap;
use hs::*;
use msg::{DsTlmSet, Instance, MsgKind, TargetMsg, TlmSetItem, ToTlmSet};
use rfe::*;
use simple_logger::SimpleLogger;
use time::UnixTimeDriver;
use to::*;

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();
    let mut record = HashMap::new();
    record.insert(
        0,
        DsTlmSet {
            enabled: true,
            path: "log/example".to_string(),
            items: vec![
                TlmSetItem {
                    counter: 0,
                    target: TargetMsg::new(Instance::All, MsgKind::ExampleOutData),
                    decimation: 0,
                },
                TlmSetItem {
                    counter: 0,
                    target: TargetMsg::new(Instance::All, MsgKind::ExampleHk),
                    decimation: 0,
                },
            ],
            id: 0,
        },
    );
    record.insert(
        1,
        DsTlmSet {
            enabled: true,
            path: "log/ds".to_string(),
            items: vec![
                TlmSetItem {
                    counter: 0,
                    target: TargetMsg::new(Instance::All, MsgKind::DsOutData),
                    decimation: 0,
                },
                TlmSetItem {
                    counter: 0,
                    target: TargetMsg::new(Instance::All, MsgKind::DsHk),
                    decimation: 0,
                },
            ],
            id: 1,
        },
    );
    record.insert(
        2,
        DsTlmSet {
            enabled: true,
            path: "log/hs".to_string(),
            items: vec![
                TlmSetItem {
                    counter: 0,
                    target: TargetMsg::new(Instance::All, MsgKind::HsOutData),
                    decimation: 0,
                },
                TlmSetItem {
                    counter: 0,
                    target: TargetMsg::new(Instance::All, MsgKind::HsHk),
                    decimation: 0,
                },
            ],
            id: 2,
        },
    );
    record.insert(
        3,
        DsTlmSet {
            enabled: true,
            path: "log/to".to_string(),
            items: vec![
                TlmSetItem {
                    counter: 0,
                    target: TargetMsg::new(Instance::All, MsgKind::ToOutData),
                    decimation: 0,
                },
                TlmSetItem {
                    counter: 0,
                    target: TargetMsg::new(Instance::All, MsgKind::ToHk),
                    decimation: 0,
                },
            ],
            id: 3,
        },
    );

    let mut example = Example::new();
    let mut ds = Ds::new(record, false);
    // let mut wd = LinuxWatchdog::new().unwrap();
    let mut grabber = StdSystemInfoGrabber::new();
    let mut hs = Hs::new(
        HsConfig {
            cpu_checks: true,
            mem_checks: true,
            fs_checks: true,
            temp_checks: true,
            watchdog_enable: true,
            watchdog_timeout: 10,
        },
        &mut grabber,
        // Some(&mut wd),
        None,
    );
    let mut tcp = UdpConnector::new("127.0.0.1", 7412, "127.0.0.1", 7413)?;
    let mut ground_connector = UdpConnector::new("127.0.0.1", 7010, "127.0.0.1", 7011)?;
    let mut dl_sets = HashMap::new();
    dl_sets.insert(
        0,
        ToTlmSet {
            items: vec![TlmSetItem {
                counter: 0,
                decimation: 9,
                target: TargetMsg::new(Instance::All, MsgKind::HsHk),
            }],
            id: 0,
            enabled: true,
        },
    );
    let mut to = To::new(&mut ground_connector, dl_sets);
    let time_driver = UnixTimeDriver::new();
    let mut instance = RfeInstance::new(Instance::Example, &time_driver);
    instance.add_app("example", &mut example)?;
    instance.add_app("to", &mut to)?;
    instance.add_app("DS", &mut ds)?;
    instance.add_app("HS", &mut hs)?;
    instance.add_connector(&mut tcp);

    instance.start();
    return Ok(());
}
