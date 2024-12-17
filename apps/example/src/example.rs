#![no_std]

use anyhow::Result;
use log::info;
use msg::{ExampleCmd, ExampleHk, ExampleOutData, Instance, Msg, MsgKind, TargetMsg};
use rfe::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct ExampleData {
    hk: ExampleHk,
    out_data: ExampleOutData,
}

pub struct Example {
    data: ExampleData,
}

impl Example {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl App for Example {
    fn init(&mut self, rfe: &mut rfe::Rfe) -> Result<()> {
        self.data = Default::default();
        rfe.subscribe(TargetMsg::new(Instance::Other, MsgKind::ExampleHk));
        rfe.subscribe(TargetMsg::new(rfe.get_instance(), MsgKind::ExampleCmd));
        return Ok(());
    }

    fn run(&mut self, rfe: &mut rfe::Rfe) {
        self.data.hk.perf.enter(rfe);
        self.data.out_data.counter += 1;
        info!("example running {:?}", self.data);
        while let Some(msg) = rfe.recv() {
            match msg.msg {
                Msg::ExampleCmd(cmd) => match cmd {
                    ExampleCmd::Noop => info!("NOOP command received"),
                    ExampleCmd::Reset => {
                        info!("RESET command received");
                        self.data = Default::default();
                    }
                },
                _ => {
                    info!("example got msg {:?}", msg);
                }
            }
        }

        if self.data.out_data.counter > 10 {
            rfe.send(Msg::ExampleCmd(ExampleCmd::Reset));
        };
        self.data.hk.perf.exit(rfe);
    }

    fn hk(&mut self, rfe: &mut rfe::Rfe) {
        self.data.hk.counter = self.data.out_data.counter;
        rfe.send(Msg::ExampleHk(self.data.hk));
    }

    fn out_data(&mut self, rfe: &mut rfe::Rfe) {
        rfe.send(Msg::ExampleOutData(self.data.out_data));
    }

    fn get_app_rate(&self) -> Rate {
        Rate::Hz1
    }
}
