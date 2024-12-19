extern crate alloc;
use core::mem::zeroed;
use std::string::ToString;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;

pub trait Reflect: std::fmt::Debug {
    fn reflect_type(&self) -> ReflectType;
    fn type_name(&self) -> &str;
    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)>;
    fn set_value(&mut self, value: ReflectValue);
    fn get_value(&self) -> ReflectValue;
    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)>;
    fn convert_variant(&mut self, i: usize);
    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)>;
    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectType {
    Value,
    Enumeration,
    Structure,
}

pub enum ReflectValue {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Vec(Vec<ReflectValue>),
    Str(String),
    Bool(bool),
}

impl ReflectValue {
    pub fn signed(&self) -> i64 {
        match self {
            ReflectValue::None => 0,
            ReflectValue::U8(v) => *v as i64,
            ReflectValue::U16(v) => *v as i64,
            ReflectValue::U32(v) => *v as i64,
            ReflectValue::U64(v) => *v as i64,
            ReflectValue::I8(v) => *v as i64,
            ReflectValue::I16(v) => *v as i64,
            ReflectValue::I32(v) => *v as i64,
            ReflectValue::I64(v) => *v as i64,
            ReflectValue::Vec(_) => 0,
            ReflectValue::Str(s) => {
                if let Ok(v) = s.parse() {
                    v
                } else {
                    0
                }
            }
            ReflectValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
        }
    }

    pub fn unsigned(&self) -> u64 {
        match self {
            ReflectValue::None => 0,
            ReflectValue::U8(v) => *v as u64,
            ReflectValue::U16(v) => *v as u64,
            ReflectValue::U32(v) => *v as u64,
            ReflectValue::U64(v) => *v as u64,
            ReflectValue::I8(v) => *v as u64,
            ReflectValue::I16(v) => *v as u64,
            ReflectValue::I32(v) => *v as u64,
            ReflectValue::I64(v) => *v as u64,
            ReflectValue::Vec(_) => 0,
            ReflectValue::Str(s) => {
                if let Ok(v) = s.parse() {
                    v
                } else {
                    0
                }
            }
            ReflectValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
        }
    }

    pub fn str(self) -> String {
        match self {
            ReflectValue::Str(s) => s,
            ReflectValue::None => "".to_string(),
            ReflectValue::U8(v) => v.to_string(),
            ReflectValue::U16(v) => v.to_string(),
            ReflectValue::U32(v) => v.to_string(),
            ReflectValue::U64(v) => v.to_string(),
            ReflectValue::I8(v) => v.to_string(),
            ReflectValue::I16(v) => v.to_string(),
            ReflectValue::I32(v) => v.to_string(),
            ReflectValue::I64(v) => v.to_string(),
            ReflectValue::Vec(_v) => "".to_string(),
            ReflectValue::Bool(v) => v.to_string(),
        }
    }

    pub fn bool(&self) -> bool {
        match self {
            ReflectValue::None => false,
            ReflectValue::U8(v) => *v == 0,
            ReflectValue::U16(v) => *v == 0,
            ReflectValue::U32(v) => *v == 0,
            ReflectValue::U64(v) => *v == 0,
            ReflectValue::I8(v) => *v == 0,
            ReflectValue::I16(v) => *v == 0,
            ReflectValue::I32(v) => *v == 0,
            ReflectValue::I64(v) => *v == 0,
            ReflectValue::Vec(_) => false,
            ReflectValue::Str(_) => false,
            ReflectValue::Bool(b) => *b,
        }
    }

    pub fn vec(self) -> Vec<ReflectValue> {
        if let ReflectValue::Vec(v) = self {
            v
        } else {
            Vec::new()
        }
    }
}

pub fn path_set(reflect: &mut dyn Reflect, path: &str, value: ReflectValue) {
    let mut next = reflect;
    for p in path.split(".") {
        let mut index = None;
        for (i, f) in next.fields().iter().enumerate() {
            if f.0 == p {
                index = Some(i);
            }
        }
        if let Some(i) = index {
            next = next.fields().remove(i).1;
        } else {
            return;
        }
    }
    next.set_value(value);
}

pub fn path_get<'a>(reflect: &'a mut dyn Reflect, path: &str) -> Option<&'a mut dyn Reflect> {
    let mut next = reflect;
    for p in path.split(".") {
        let mut index = None;
        for (i, f) in next.fields().iter().enumerate() {
            if f.0 == p {
                index = Some(i);
            }
        }
        if let Some(i) = index {
            next = next.fields().remove(i).1;
        } else {
            return None;
        }
    }
    Some(next)
}

fn _flatten<'a>(reflect: &'a mut dyn Reflect) -> HashMap<String, &'a mut dyn Reflect> {
    let mut list = HashMap::new();

    match reflect.reflect_type() {
        ReflectType::Value => match reflect.get_value() {
            ReflectValue::Vec(_) => {
                for (i, r) in reflect.as_vec().unwrap().into_iter().enumerate() {
                    let map = _flatten(r);
                    for (sn, v) in map {
                        list.insert(alloc::format!("[{i}].{}", sn), v);
                    }
                }
            }
            _ => {
                list.insert("".to_string(), reflect);
            }
        },
        ReflectType::Enumeration => {
            if let Some((name, r)) = reflect.unwrap_variant() {
                let map = _flatten(r);
                for (sn, v) in map {
                    list.insert(alloc::format!("{}.{}", name, sn), v);
                }
            }
        }
        ReflectType::Structure => {
            for (name, r) in reflect.fields() {
                let map = _flatten(r);
                for (sn, v) in map {
                    list.insert(alloc::format!("{}.{}", name, sn), v);
                }
            }
        }
    }

    list
}

pub fn flatten<'a>(reflect: &'a mut dyn Reflect) -> HashMap<String, &'a mut dyn Reflect> {
    _flatten(reflect)
        .into_iter()
        .map(|(k, v)| (k[..k.len() - 1].to_string().replace(".[", "["), v))
        .collect()
}

impl Reflect for u8 {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "u8"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.unsigned() as Self;
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::U8(*self)
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl Reflect for u16 {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "u16"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.unsigned() as Self;
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::U16(*self)
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl Reflect for u32 {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "u32"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.unsigned() as Self;
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::U32(*self)
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl Reflect for u64 {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "u64"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.unsigned() as Self;
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::U64(*self)
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl Reflect for i8 {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "i8"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.signed() as Self;
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::I8(*self)
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl Reflect for i16 {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "i16"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.signed() as Self;
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::I16(*self)
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl Reflect for i32 {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "i32"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.signed() as Self;
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::I32(*self)
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl Reflect for i64 {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "i64"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.signed() as Self;
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::I64(*self)
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl Reflect for String {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "str"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.str();
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::Str(self.to_string())
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl Reflect for bool {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "bool"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        *self = value.bool();
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::Bool(*self)
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec(&mut self) -> Option<Vec<&mut dyn Reflect>> {
        None
    }
}

impl<T: Reflect> Reflect for Vec<T> {
    fn reflect_type(&self) -> ReflectType {
        ReflectType::Value
    }

    fn type_name(&self) -> &str {
        "vec[]"
    }

    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
        Vec::new()
    }

    fn set_value(&mut self, value: ReflectValue) {
        if let ReflectValue::Vec(v) = value {
            *self = v
                .into_iter()
                .map(|x| {
                    let mut t: T = unsafe { zeroed() };
                    t.set_value(x);
                    t
                })
                .collect();
        }
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::Vec(self.iter().map(|x| x.get_value()).collect())
    }

    fn variants(&self) -> Vec<(&'static str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn convert_variant(&mut self, _i: usize) {}

    fn unwrap_variant(&mut self) -> Option<(&str, &mut dyn Reflect)> {
        None
    }

    fn as_vec<'a>(&'a mut self) -> Option<Vec<&'a mut dyn Reflect>> {
        Some(
            self.iter_mut()
                .map(|x| {
                    let v: &mut dyn Reflect = x;
                    v
                })
                .collect(),
        )
    }
}
