#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    collections::HashMap,
    fs::write,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

use anyhow::anyhow;
use anyhow::Result;
use connector::{Connector, UdpConnector};
use egui::Ui;
use log::*;
use msg::{DsTlmSet, Msg, TargetMsg, TlmSetItem};
use reflect::*;
use rfe::macros::Reflect;
use rfe::*;
extern crate alloc;
use simple_logger::SimpleLogger;

#[derive(Debug, Default, Reflect)]
struct TestStruct {
    counter: i8,
}

#[derive(Debug, Default, Reflect)]
enum EnumTest {
    #[default]
    None,
    Test(TestStruct),
}

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    spawn(|| {
        let mut udp = UdpConnector::new("127.0.0.1", 7011, "127.0.0.1", 7010).unwrap();

        let mut next_time = Instant::now() + Duration::from_millis(10);
        loop {
            sleep(next_time - Instant::now());

            while let Some(msgs) = udp.recv() {
                for msg in msgs {
                    info!("got msg {:?}", msg);
                }
            }
            next_time += Duration::from_millis(10);
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([560.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "RFE Ground",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
    .or(Err(anyhow!("eframe failed")))?;
    return Ok(());
}

struct MyApp {
    command: Msg,
    values: HashMap<String, String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            command: Msg::None,
            values: HashMap::new(),
        }
    }
}

fn build_command_ui_value_number(ui: &mut Ui, name: &str, cmd: &mut dyn Reflect) {
    let mut r = cmd.get_value().str();
    ui.horizontal(|ui| {
        ui.label(name);
        ui.text_edit_singleline(&mut r);
        cmd.set_value(ReflectValue::Str(r));
    });
}

fn build_command_ui(ui: &mut Ui, name: &str, cmd: &mut dyn Reflect) {
    match cmd.reflect_type() {
        ReflectType::Value => match cmd.get_value() {
            ReflectValue::None => {}
            ReflectValue::U8(_) => build_command_ui_value_number(ui, name, cmd),
            ReflectValue::U16(_) => build_command_ui_value_number(ui, name, cmd),
            ReflectValue::U32(_) => build_command_ui_value_number(ui, name, cmd),
            ReflectValue::U64(_) => build_command_ui_value_number(ui, name, cmd),
            ReflectValue::I8(_) => build_command_ui_value_number(ui, name, cmd),
            ReflectValue::I16(_) => build_command_ui_value_number(ui, name, cmd),
            ReflectValue::I32(_) => build_command_ui_value_number(ui, name, cmd),
            ReflectValue::I64(_) => build_command_ui_value_number(ui, name, cmd),
            ReflectValue::Vec(_vec) => {}
            ReflectValue::Str(_) => {}
            ReflectValue::Bool(_) => {}
        },
        ReflectType::Enumeration => {
            // variants

            for (i, (name, _v)) in cmd.variants().into_iter().enumerate() {
                if ui.button(name).clicked() {
                    cmd.convert_variant(i);
                }
            }

            // values
            if let Some((name, cmd)) = cmd.unwrap_variant() {
                build_command_ui(ui, name, cmd);
            }
        }
        ReflectType::Structure => {
            for (name, cmd) in cmd.fields().into_iter() {
                build_command_ui(ui, name, cmd);
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Command: {:?}", self.command));
            if ui.button("X").clicked() {
                self.command = Msg::None;
            }

            build_command_ui(ui, "Msg", &mut self.command);

            // if self
            //     .command_path
            //     .as_bytes()
            //     .iter()
            //     .filter(|x| **x == b'.')
            //     .count()
            //     >= 2
            // {
            //     for item in &self.tree.get_add_path(&self.command_path).children {
            //         for c in &item.children {
            //             if c.is_last() {
            //                 if !self.values.contains_key(&c.name) {
            //                     self.values.insert(c.name.clone(), String::new());
            //                 }
            //                 let sref = self.values.get_mut(&c.name).unwrap();
            //                 ui.horizontal(|ui| {
            //                     ui.label(format!("{} : {}", item.name, c.name.clone()));
            //                     ui.text_edit_singleline(sref);
            //                 });
            //             }
            //         }
            //     }
            //     if ui.button("send").clicked() {
            //         info!("sending {}", self.command_path);
            //     }
            // } else {
            //     for item in &self.tree.get_add_path(&self.command_path).children {
            //         if ui.button(item.name.clone()).clicked() {
            //             self.command_path = format!("{}.{}", self.command_path, item.name);
            //         }
            //     }
            // }
        });
    }
}
