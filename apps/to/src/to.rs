#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use anyhow::Result;
use connector::Connector;
use hashbrown::HashMap;
use log::*;
use msg::{Instance, Msg, MsgKind, TargetMsg, TlmSetId, ToCmd, ToHk, ToOutData, ToTlmSet};
use rfe::*;

#[derive(Debug, Clone, Default)]
pub struct ToData {
    out_data: ToOutData,
    hk: ToHk,
}

pub struct To<'a> {
    data: ToData,
    connector: &'a mut dyn Connector,
    tlm_sets: HashMap<TlmSetId, ToTlmSet>,
}

impl<'a> To<'a> {
    pub fn new(connector: &'a mut dyn Connector, tlm_sets: HashMap<TlmSetId, ToTlmSet>) -> Self {
        Self {
            connector,
            data: Default::default(),
            tlm_sets,
        }
    }

    pub fn update_subscriptions(&mut self, rfe: &mut Rfe) {
        rfe.unsubscribe_all();
        rfe.subscribe_all(
            self.tlm_sets
                .values()
                .filter(|x| x.enabled)
                .map(|x| &x.items)
                .flatten()
                .map(|x| x.target),
        );
    }

    pub fn handle_cmd(&mut self, rfe: &mut Rfe, cmd: &ToCmd) {
        match cmd {
            ToCmd::Noop => info!("received Noop"),
            ToCmd::Reset => {
                info!("received Reset");
                self.data = Default::default();
            }
            ToCmd::AddTlmSet(to_tlm_set) => {
                info!("received AddTlmSet");
                if let Err(e) = self.tlm_sets.try_insert(to_tlm_set.id, to_tlm_set.clone()) {
                    error!("Could not add tlm set {} {e}", to_tlm_set.id);
                } else {
                    info!("TlmSet {} added", to_tlm_set.id);
                    self.update_subscriptions(rfe);
                }
            }
            ToCmd::RemoveTlmSet(set_id) => {
                info!("received RemoveTlmSet");
                if let Some(_set) = self.tlm_sets.remove(set_id) {
                    info!("removed tlm set {}", set_id);
                    self.update_subscriptions(rfe);
                } else {
                    warn!("Cannot remove tlm set {}, does not exist", set_id);
                }
            }
            ToCmd::DisableTlmSet(set_id) => {
                info!("received DisableTlmSet");
                if let Some(set) = self.tlm_sets.get_mut(set_id) {
                    info!("set {set_id} is now disabled");
                    set.enabled = false;
                    self.update_subscriptions(rfe);
                } else {
                    warn!("could not disable set {set_id}, does not exist");
                }
            }
            ToCmd::EnablTlmSet(set_id) => {
                info!("received EnablTlmSet");
                if let Some(set) = self.tlm_sets.get_mut(set_id) {
                    info!("set {set_id} is now enabled");
                    set.enabled = true;
                    self.update_subscriptions(rfe);
                } else {
                    warn!("could not enable set {set_id}, does not exist");
                }
            }
        }
        info!("got cmd {:?}", cmd);
    }
}

impl App for To<'_> {
    fn init(&mut self, rfe: &mut Rfe) -> Result<()> {
        rfe.subscribe(TargetMsg::new(rfe.get_instance(), MsgKind::ToCmd));
        self.update_subscriptions(rfe);
        return Ok(());
    }

    fn run(&mut self, rfe: &mut Rfe) {
        self.data.out_data.counter += 1;
        let mut msgs = Vec::new();
        while let Some(msg) = rfe.recv() {
            let mut is_cmd = false;
            if let Msg::ToCmd(cmd) = &msg.msg {
                if msg.instance == rfe.get_instance() {
                    is_cmd = true;
                    self.handle_cmd(rfe, cmd);
                }
            }
            if !is_cmd {
                for tlm_set in self.tlm_sets.values_mut().filter(|x| x.enabled) {
                    for item in &mut tlm_set.items {
                        let msg_target = msg.to_target();
                        if msg_target == item.target
                            || (msg_target.msg == item.target.msg
                                && item.target.instance == Instance::All)
                        {
                            if item.counter % (item.decimation + 1) == 0 {
                                msgs.push(msg.clone());
                            }
                            item.counter += 1;
                        }
                    }
                }
            }
        }
        if msgs.len() > 0 {
            self.connector.send(msgs);
        }

        while let Some(msgs) = self.connector.recv() {
            for msg in msgs {
                rfe.post_message(msg);
            }
        }
    }

    fn hk(&mut self, rfe: &mut Rfe) {
        rfe.send(Msg::ToHk(self.data.hk));
    }

    fn out_data(&mut self, rfe: &mut Rfe) {
        rfe.send(Msg::ToOutData(self.data.out_data));
    }

    fn get_app_rate(&self) -> Rate {
        Rate::Hz50
    }
}
