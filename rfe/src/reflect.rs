extern crate alloc;
use std::string::ToString;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

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
            ReflectValue::Str(_) => 0,
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
            ReflectValue::Str(_) => 0,
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
            _ => "".to_string(),
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

    pub fn vec(self) -> Vec<ReflectValue> {}
}

pub trait Reflect: std::fmt::Debug {
    fn reflect_type(&self) -> ReflectType;
    fn type_name(&self) -> &str;
    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)>;
    fn set_value(&mut self, value: ReflectValue);
    fn get_value(&self) -> ReflectValue;
    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)>;
    fn as_variant(&mut self, i: usize) -> Option<&mut dyn Reflect>;
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
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
        *self = value.vec();
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::Bool(*self)
    }

    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
        Vec::new()
    }

    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
        None
    }
}
