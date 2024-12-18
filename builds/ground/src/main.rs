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
use log::*;
use msg::DsTlmSet;
use reflect::*;
use rfe::macros::Reflect;
use rfe::*;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreeNode {
    name: String,
    children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new<T: ToString>(name: T, children: Vec<TreeNode>) -> Self {
        Self {
            name: name.to_string(),
            children,
        }
    }

    pub fn add_child<T: ToString>(&mut self, name: T) -> &mut TreeNode {
        self.children.push(TreeNode::new(name, Vec::new()));
        let i = self.children.len() - 1;
        return &mut self.children[i];
    }

    pub fn get_add_path(&mut self, path: &str) -> &mut TreeNode {
        if !path.starts_with(&format!("{}.", self.name)) {
            return self;
        }
        let path = path.replace(&format!("{}.", self.name), "");
        let names = path.split(".");
        let mut working_tree = self;

        for name in names {
            let mut index = None;
            for (i, child) in &mut working_tree.children.iter().enumerate() {
                if child.name == name {
                    index = Some(i);
                    break;
                }
            }
            if let Some(index) = index {
                working_tree = &mut working_tree.children[index];
            } else {
                working_tree = working_tree.add_child(name);
            }
        }

        working_tree
    }

    pub fn is_last(&self) -> bool {
        self.children.len() == 0
    }
}

fn to_tree(msg_enum: Vec<String>) -> TreeNode {
    let mut t = TreeNode::new("Msg", Vec::new());
    for p in msg_enum {
        let mut x = p
            .split(" = ")
            .map(|x| x.split("."))
            .flatten()
            .collect::<Vec<_>>()
            .join(".");

        if x.starts_with(".") {
            x.remove(0);
        } else {
            x = format!("Msg.{}", x);
        }
        let s = x.rsplitn(2, ".").collect::<Vec<_>>();

        if s.len() == 2 {
            t.get_add_path(s[1]).add_child(s[0]);
        } else {
            t.add_child(s[0]);
        }
    }

    t
}

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

    info!(
        "{:?}",
        msg::Msg::DsCmd(msg::DsCmd::AddTlmSet(DsTlmSet::default()))
    );
    info!(
        "json {}",
        serde_json::to_string(&msg::Msg::DsCmd(msg::DsCmd::Noop)).unwrap()
    );

    let mut test = TestStruct::default();
    info!("{:?}", test.fields());
    reflect::path_set(&mut test, "counter", ReflectValue::I64(1));
    info!("{:?}", test.fields());

    let mut test2 = EnumTest::None;

    // info!("{:?}", test2);

    // let testref = test2.as_variant(1).unwrap();

    // test2.as_variant(1).unwrap().fields()[0]
    //     .1
    //     .set_value(ReflectValue::I64(3));
    // info!("{:?}", test2);

    // spawn(|| {
    //     let mut udp = UdpConnector::new("127.0.0.1", 7011, "127.0.0.1", 7010).unwrap();

    //     let mut next_time = Instant::now() + Duration::from_millis(10);
    //     loop {
    //         sleep(next_time - Instant::now());

    //         while let Some(msgs) = udp.recv() {
    //             for msg in msgs {
    //                 info!("got msg {:?}", msg);
    //             }
    //         }
    //         next_time += Duration::from_millis(10);
    //     }
    // });

    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([560.0, 480.0]),
    //     ..Default::default()
    // };
    // eframe::run_native(
    //     "RFE Ground",
    //     options,
    //     Box::new(|_cc| Ok(Box::<MyApp>::default())),
    // )
    // .or(Err(anyhow!("eframe failed")))?;
    return Ok(());
}

// struct MyApp {
//     command_path: String,
//     tree: TreeNode,
//     values: HashMap<String, String>,
// }

// impl Default for MyApp {
//     fn default() -> Self {
//         let tree = to_tree(msg::Msg::None.enumerate_clean());
//         Self {
//             command_path: "Msg".to_string(),
//             tree,
//             values: HashMap::new(),
//         }
//     }
// }

// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading(format!("Commands: {}", self.command_path));
//             if ui.button("X").clicked() {
//                 self.command_path = "Msg".to_string();
//             }
//             if self
//                 .command_path
//                 .as_bytes()
//                 .iter()
//                 .filter(|x| **x == b'.')
//                 .count()
//                 >= 2
//             {
//                 for item in &self.tree.get_add_path(&self.command_path).children {
//                     for c in &item.children {
//                         if c.is_last() {
//                             if !self.values.contains_key(&c.name) {
//                                 self.values.insert(c.name.clone(), String::new());
//                             }
//                             let sref = self.values.get_mut(&c.name).unwrap();
//                             ui.horizontal(|ui| {
//                                 ui.label(format!("{} : {}", item.name, c.name.clone()));
//                                 ui.text_edit_singleline(sref);
//                             });
//                         }
//                     }
//                 }
//                 if ui.button("send").clicked() {
//                     info!("sending {}", self.command_path);
//                 }
//             } else {
//                 for item in &self.tree.get_add_path(&self.command_path).children {
//                     if ui.button(item.name.clone()).clicked() {
//                         self.command_path = format!("{}.{}", self.command_path, item.name);
//                     }
//                 }
//             }
//         });
//     }
// }
