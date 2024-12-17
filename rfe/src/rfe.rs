extern crate alloc;
use core::cell::RefCell;

use alloc::rc::Rc;
use alloc::{collections::vec_deque::VecDeque, vec::Vec};
use anyhow::{anyhow, Result};
use hashbrown::{HashMap, HashSet};
use log::*;

use crate::{
    connector::Connector,
    msg::{Instance, Msg, MsgPacket, SubList, TargetMsg},
    time::{TimeData, TimeDriver},
};

pub trait Hk: Sized + Clone + Copy + 'static + Send + Sync {}
impl<T> Hk for T where T: Sized + Clone + Copy + 'static + Send + Sync {}

pub trait OutData: Sized + Clone + Copy + 'static + Send + Sync {}
impl<T> OutData for T where T: Sized + Clone + Copy + 'static + Send + Sync {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rate {
    Hz1,
    Hz5,
    Hz10,
    Hz20,
    Hz50,
    Hz100,
}

type RfeTimeRef<'a> = Rc<RefCell<RfeTime<'a>>>;

pub struct RfeTime<'a> {
    time_data: TimeData,
    time_driver: &'a dyn TimeDriver,
}

pub struct RfeInstance<'a> {
    app_list: HashMap<&'a str, AppRef<'a>>,
    time: RfeTimeRef<'a>,
    #[allow(dead_code)]
    instance: Instance,
    connectors: Vec<ConnectorState<'a>>,
    sch_counter: u64,
}

pub struct AppRef<'a> {
    app: &'a mut dyn App,
    app_rate: Rate,
    out_data_rate: Rate,
    hk_rate: Rate,
    rfe: Rfe<'a>,
}

pub struct Rfe<'a> {
    subscriptions: HashSet<TargetMsg>,
    msgs_to_send: Vec<MsgPacket>,
    msgs_recevied: VecDeque<MsgPacket>,
    instance: Instance,
    time: RfeTimeRef<'a>,
    subs_updated: bool,
}

#[derive(Debug)]
pub struct ConnectorState<'a> {
    connector: &'a mut dyn Connector,
    subscriptions: HashSet<TargetMsg>,
    subs_received: bool,
    subs_last_requested: u64,
}

impl<'a> Rfe<'a> {
    pub fn new(instance: Instance, time: RfeTimeRef<'a>) -> Self {
        Self {
            subscriptions: HashSet::new(),
            msgs_to_send: Vec::new(),
            msgs_recevied: VecDeque::new(),
            instance,
            subs_updated: false,
            time,
        }
    }

    pub fn get_instance(&self) -> Instance {
        return self.instance;
    }

    pub fn subscribe(&mut self, msg: TargetMsg) {
        self.subscriptions.insert(msg);
        self.subs_updated = true;
    }

    pub fn subscribe_all<T: IntoIterator<Item = TargetMsg>>(&mut self, msgs: T) {
        self.subscriptions.extend(msgs.into_iter());
        self.subs_updated = true;
    }

    pub fn unsubscribe(&mut self, msg: &TargetMsg) {
        self.subscriptions.remove(msg);
        self.subs_updated = true;
    }

    pub fn unsubscribe_all(&mut self) {
        self.subscriptions.clear();
        self.subs_updated = true;
    }

    pub fn send(&mut self, msg: Msg) {
        self.msgs_to_send.push(MsgPacket::new(
            self.get_instance(),
            msg,
            self.get_system_time(),
        ));
    }

    pub fn send_cmd(&mut self, msg: Msg, target: Instance) {
        self.msgs_to_send
            .push(MsgPacket::new(target, msg, self.get_system_time()));
    }

    pub fn post_message(&mut self, msg: MsgPacket) {
        self.msgs_recevied.push_back(msg);
    }

    pub fn recv(&mut self) -> Option<MsgPacket> {
        self.msgs_recevied.pop_front()
    }

    /// time starting from power on or program start
    pub fn get_met_time(&self) -> u64 {
        let time = self.time.borrow();
        time.time_driver.get_monotonic_time(time.time_data)
    }

    /// Time in microseconds relative to system epoch
    pub fn get_system_time(&self) -> u64 {
        let time = self.time.borrow();
        time.time_driver.get_system_time(time.time_data)
    }
}

pub trait App {
    fn init(&mut self, rfe: &mut Rfe) -> Result<()>;
    fn run(&mut self, rfe: &mut Rfe);
    fn hk(&mut self, rfe: &mut Rfe);
    fn out_data(&mut self, rfe: &mut Rfe);
    fn get_app_rate(&self) -> Rate;
}

impl<'a> RfeInstance<'a> {
    pub fn new(instance: Instance, time_driver: &'a dyn TimeDriver) -> Self {
        let time = Rc::new(RefCell::new(RfeTime {
            time_data: TimeData {
                sch_counter: 0,
                time_offset: 0,
            },
            time_driver,
        }));
        Self {
            app_list: HashMap::new(),
            instance,
            connectors: Vec::new(),
            time,
            sch_counter: 0,
        }
    }

    pub fn add_app(&mut self, name: &'a str, app: &'a mut dyn App) -> Result<()> {
        if self.app_list.contains_key(name) {
            return Err(anyhow!(
                "failed to add app {name}, already added an app with that name"
            ));
        }
        let app_rate = app.get_app_rate();
        self.app_list.insert(
            name,
            AppRef {
                app: app,
                app_rate: app_rate,
                hk_rate: Rate::Hz1,
                out_data_rate: app_rate,
                rfe: Rfe::new(self.instance, self.time.clone()),
            },
        );

        let appref = self.app_list.get_mut(name).unwrap();
        if let Err(e) = appref.app.init(&mut appref.rfe) {
            error!("app {name} failed to initialize {e}");
        }

        return Ok(());
    }

    pub fn add_connector(&mut self, connector: &'a mut dyn Connector) {
        self.connectors.push(ConnectorState {
            connector,
            subs_last_requested: 0,
            subs_received: false,
            subscriptions: HashSet::new(),
        });
    }

    /// Expected to be called at 100Hz
    pub fn run(&mut self) {
        let mut msgs = Vec::new();
        for app in self.app_list.values_mut() {
            if app.app_rate == Rate::Hz100
                || (self.sch_counter % 2 == 0 && app.app_rate == Rate::Hz50)
                || (self.sch_counter % 5 == 0 && app.app_rate == Rate::Hz20)
                || (self.sch_counter % 10 == 0 && app.app_rate == Rate::Hz10)
                || (self.sch_counter % 20 == 0 && app.app_rate == Rate::Hz5)
                || (self.sch_counter % 100 == 0 && app.app_rate == Rate::Hz1)
            {
                app.app.run(&mut app.rfe);
            }

            if app.hk_rate == Rate::Hz100
                || (self.sch_counter % 2 == 0 && app.hk_rate == Rate::Hz50)
                || (self.sch_counter % 5 == 0 && app.hk_rate == Rate::Hz20)
                || (self.sch_counter % 10 == 0 && app.hk_rate == Rate::Hz10)
                || (self.sch_counter % 20 == 0 && app.hk_rate == Rate::Hz5)
                || (self.sch_counter % 100 == 0 && app.hk_rate == Rate::Hz1)
            {
                app.app.hk(&mut app.rfe);
            }

            if app.out_data_rate == Rate::Hz100
                || (self.sch_counter % 2 == 0 && app.out_data_rate == Rate::Hz50)
                || (self.sch_counter % 5 == 0 && app.out_data_rate == Rate::Hz20)
                || (self.sch_counter % 10 == 0 && app.out_data_rate == Rate::Hz10)
                || (self.sch_counter % 20 == 0 && app.out_data_rate == Rate::Hz5)
                || (self.sch_counter % 100 == 0 && app.out_data_rate == Rate::Hz1)
            {
                app.app.out_data(&mut app.rfe);
            }

            let new_msgs = core::mem::take(&mut app.rfe.msgs_to_send);
            msgs.extend(new_msgs);
        }

        for app in self.app_list.values_mut() {
            for msg in &msgs {
                if app
                    .rfe
                    .subscriptions
                    .contains(&TargetMsg::new(self.instance, msg.msg.kind()))
                    || app
                        .rfe
                        .subscriptions
                        .contains(&TargetMsg::new(Instance::All, msg.msg.kind()))
                {
                    app.rfe.post_message(msg.clone());
                }
            }
        }

        // send message to connectors
        for connector_state in &mut self.connectors {
            let mut to_send = Vec::new();
            for msg in &msgs {
                if connector_state
                    .subscriptions
                    .contains(&TargetMsg::new(self.instance, msg.msg.kind()))
                    || connector_state
                        .subscriptions
                        .contains(&TargetMsg::new(Instance::All, msg.msg.kind()))
                    || connector_state
                        .subscriptions
                        .contains(&TargetMsg::new(Instance::Other, msg.msg.kind()))
                {
                    to_send.push(msg.clone());
                }
            }
            if to_send.len() > 0 {
                connector_state.connector.send(to_send);
            }
        }

        drop(msgs);
        let mut connector_msgs = Vec::new();

        // receive messages from connectors
        for connector_state in &mut self.connectors {
            if let Some(msgs) = connector_state.connector.recv() {
                // check for sublist/sub request
                for msg in &msgs {
                    if let Msg::SubList(list) = &msg.msg {
                        connector_state.subs_received = true;
                        connector_state.subscriptions.clear();
                        connector_state.subscriptions.extend(list.subs.clone());
                    }
                    if let Msg::SetTimeCmd(new_time) = &msg.msg {
                        self.time.borrow_mut().time_data.time_offset = *new_time;
                    }
                    if let Msg::SubRequest = msg.msg {
                        let mut subs = Vec::new();
                        for app in self.app_list.values() {
                            subs.extend(app.rfe.subscriptions.clone());
                        }
                        let mut sublist = Vec::new();
                        sublist.push(MsgPacket {
                            instance: self.instance,
                            msg: Msg::SubList(SubList { subs }),
                            timestamp: 0,
                        });
                        connector_state.connector.send(sublist);
                    }
                }

                connector_msgs.extend(msgs);
            }
        }

        // send messages to apps
        for app in self.app_list.values_mut() {
            for msg in &connector_msgs {
                if app
                    .rfe
                    .subscriptions
                    .contains(&TargetMsg::new(msg.instance, msg.msg.kind()))
                    || app
                        .rfe
                        .subscriptions
                        .contains(&TargetMsg::new(Instance::All, msg.msg.kind()))
                    || app
                        .rfe
                        .subscriptions
                        .contains(&TargetMsg::new(Instance::Other, msg.msg.kind()))
                {
                    app.rfe.post_message(msg.clone());
                }
            }
        }

        // clear connector subs if subs changed
        for rfe in self.app_list.values_mut() {
            if rfe.rfe.subs_updated {
                for connector_state in &mut self.connectors {
                    connector_state.subs_received = false;
                }
                rfe.rfe.subs_updated = false;
            }
        }

        // handle connector subscriptions
        for connector_state in &mut self.connectors {
            if (!connector_state.subs_received
                && self.sch_counter - connector_state.subs_last_requested >= 10)
                || (connector_state.subs_received
                    && self.sch_counter - connector_state.subs_last_requested >= 10000)
            {
                // request subs
                let mut request = Vec::new();
                request.push(MsgPacket {
                    instance: self.instance,
                    msg: Msg::SubRequest,
                    timestamp: 0,
                });
                connector_state.connector.send(request);
                connector_state.subs_last_requested = self.sch_counter;
            }
        }

        self.sch_counter += 1;
    }

    #[cfg(feature = "std")]
    pub fn start(&mut self) {
        use core::time::Duration;
        use std::{thread::sleep, time::Instant};

        let mut next_time = Instant::now() + Duration::from_millis(10);
        loop {
            sleep(next_time - Instant::now());
            self.run();
            next_time += Duration::from_millis(10);
            if next_time < Instant::now() {
                next_time = Instant::now() + Duration::from_millis(10);
            }
        }
    }
}
