#![no_std]

use bincode::encode_to_vec;

mod file;
pub use file::*;

extern crate alloc;

use anyhow::Result;
use hashbrown::HashMap;
use log::*;
use msg::{DsCmd, DsHk, DsOutData, DsTlmSet, Instance, Msg, TlmSetId};
use rfe::*;

#[derive(Debug, Default)]
pub struct DsData<F: DsFile> {
    hk: DsHk,
    out_data: DsOutData,
    file_list: HashMap<TlmSetId, F>,
    enabled: bool,
}

pub struct DsFileSettings {
    pub max_size: u32,
    pub max_age: u32,
    pub enabled: bool,
}

pub struct Ds<F: DsFile> {
    data: DsData<F>,
    tlm_sets: HashMap<TlmSetId, DsTlmSet>,
    start_enabled: bool,
}

impl<F: DsFile> Ds<F> {
    pub fn new(tlm_sets: HashMap<TlmSetId, DsTlmSet>, start_enabled: bool) -> Self {
        Self {
            data: Default::default(),
            tlm_sets,
            start_enabled,
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
}

impl<F: DsFile> App for Ds<F> {
    fn init(&mut self, rfe: &mut rfe::Rfe) -> Result<()> {
        self.data = Default::default();
        self.data.enabled = self.start_enabled;

        self.update_subscriptions(rfe);
        return Ok(());
    }

    fn run(&mut self, rfe: &mut rfe::Rfe) {
        self.data.out_data.counter += 1;
        self.data.out_data.bytes_written_this_cycle = 0;
        while let Some(msg) = rfe.recv() {
            match msg.msg {
                Msg::DsCmd(cmd) => match cmd {
                    DsCmd::Noop => info!("Noop command received"),
                    DsCmd::Reset => {
                        info!("Reset command received");
                        self.data = Default::default();
                    }
                    DsCmd::CloseAll => {
                        info!("CloseAll command received");
                        for f in self.data.file_list.values_mut() {
                            f.close();
                        }
                    }
                    DsCmd::Close(f) => {
                        info!("Close command received");
                        if let Some(file) = self.data.file_list.get_mut(&f) {
                            file.close();
                        } else {
                            error!("Cannot close file {f}, file doesn't exist");
                        }
                    }
                    DsCmd::AddTlmSet(ds_tlm_set) => {
                        info!("received AddTlmSet");
                        if let Err(e) = self.tlm_sets.try_insert(ds_tlm_set.id, ds_tlm_set.clone())
                        {
                            error!("Could not add tlm set {} {e}", ds_tlm_set.id);
                        } else {
                            info!("TlmSet {} added", ds_tlm_set.id);
                            self.update_subscriptions(rfe);
                        }
                    }
                    DsCmd::RemoveTlmSet(set_id) => {
                        info!("received RemoveTlmSet");
                        if let Some(_set) = self.tlm_sets.remove(&set_id) {
                            info!("removed tlm set {}", set_id);
                            self.update_subscriptions(rfe);
                        } else {
                            warn!("Cannot remove tlm set {}, does not exist", set_id);
                        }
                    }
                    DsCmd::DisableTlmSet(set_id) => {
                        info!("received DisableTlmSet");
                        if let Some(set) = self.tlm_sets.get_mut(&set_id) {
                            info!("set {set_id} is now disabled");
                            set.enabled = false;
                            self.update_subscriptions(rfe);
                        } else {
                            warn!("could not disable set {set_id}, does not exist");
                        }
                    }
                    DsCmd::EnablTlmSet(set_id) => {
                        info!("received EnablTlmSet");
                        if let Some(set) = self.tlm_sets.get_mut(&set_id) {
                            info!("set {set_id} is now enabled");
                            set.enabled = true;
                            self.update_subscriptions(rfe);
                        } else {
                            warn!("could not enable set {set_id}, does not exist");
                        }
                    }
                },
                _ => {
                    if !self.data.enabled {
                        continue;
                    }

                    for tlm_set in self.tlm_sets.values_mut().filter(|x| x.enabled) {
                        for item in &mut tlm_set.items {
                            let msg_target = msg.to_target();
                            if msg_target == item.target
                                || (msg_target.msg == item.target.msg
                                    && item.target.instance == Instance::All)
                            {
                                if item.counter % (item.decimation + 1) == 0 {
                                    let file =
                                        if let Some(f) = self.data.file_list.get_mut(&tlm_set.id) {
                                            f
                                        } else {
                                            let f = F::new(tlm_set.path.clone());
                                            self.data.file_list.insert(tlm_set.id, f);
                                            self.data.file_list.get_mut(&tlm_set.id).unwrap()
                                        };

                                    let bytes = encode_to_vec(&msg, BINCODE_CONFIG)
                                        .expect("failed serialize ds packet");
                                    if let Err(e) = file.write(&bytes) {
                                        error!("file write error: {e}");
                                    } else {
                                        self.data.out_data.bytes_written_this_cycle +=
                                            bytes.len() as u32;
                                    }
                                }
                                item.counter += 1;
                            }
                        }
                    }
                }
            }
        }

        self.data.out_data.bytes_written += self.data.out_data.bytes_written_this_cycle;
    }

    fn hk(&mut self, rfe: &mut rfe::Rfe) {
        self.data.hk.counter = self.data.out_data.counter;
        rfe.send(Msg::DsHk(self.data.hk));
    }

    fn out_data(&mut self, rfe: &mut rfe::Rfe) {
        rfe.send(Msg::DsOutData(self.data.out_data));
    }

    fn get_app_rate(&self) -> Rate {
        Rate::Hz1
    }
}
