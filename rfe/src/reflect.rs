extern crate alloc;
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
        }
    }
}

pub trait Reflect: std::fmt::Debug {
    fn reflect_type(&self) -> ReflectType;
    fn type_name(&self) -> &str;
    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)>;
    fn set_value(&mut self, value: ReflectValue);
    fn get_value(&self) -> ReflectValue;
    fn variants(&self) -> Vec<Box<dyn Reflect>>;
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

    fn variants(&self) -> Vec<Box<dyn Reflect>> {
        Vec::new()
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

    fn variants(&self) -> Vec<Box<dyn Reflect>> {
        Vec::new()
    }
}

// pub trait ToCsvClean {
//     fn to_csv_clean(&self) -> Vec<String>;
//     fn enumerate_clean(&self) -> Vec<String>;
// }

// impl ToCsv for u8 {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }

//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = u8")]
//     }
// }

// impl ToCsv for i8 {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }

//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = i8")]
//     }
// }

// impl ToCsv for u16 {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }

//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = u16")]
//     }
// }

// impl ToCsv for i16 {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = i16")]
//     }
// }

// impl ToCsv for u32 {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = u32")]
//     }
// }

// impl ToCsv for i32 {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = i32")]
//     }
// }

// impl ToCsv for u64 {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = u64")]
//     }
// }

// impl ToCsv for i64 {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = i64")]
//     }
// }

// impl ToCsv for &str {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = String")]
//     }
// }

// impl ToCsv for String {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = String")]
//     }
// }

// impl ToCsv for bool {
//     fn to_csv(&self) -> Vec<String> {
//         vec![format!(" = {}", self)]
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(" = bool")]
//     }
// }

// impl<T: ToCsv + Default> ToCsv for Vec<T> {
//     fn to_csv(&self) -> Vec<String> {
//         self.iter()
//             .enumerate()
//             .map(|(i, x)| {
//                 x.to_csv()
//                     .iter()
//                     .map(|a| format!("[{}].{}", i, a))
//                     .collect::<Vec<String>>()
//             })
//             .flatten()
//             .collect()
//     }
//     fn enumerate(&self) -> Vec<String> {
//         T::default()
//             .enumerate()
//             .iter()
//             .map(|x| format!("[{}].{}", std::any::type_name::<T>(), x))
//             .collect::<Vec<String>>()
//     }
// }

// impl<T: ToCsv + Default, const N: usize> ToCsv for [T; N] {
//     fn to_csv(&self) -> Vec<String> {
//         self.iter()
//             .enumerate()
//             .map(|(i, x)| {
//                 x.to_csv()
//                     .iter()
//                     .map(|a| format!("[{}].{}", i, a))
//                     .collect::<Vec<String>>()
//             })
//             .flatten()
//             .collect()
//     }
//     fn enumerate(&self) -> Vec<String> {
//         T::default()
//             .enumerate()
//             .iter()
//             .map(|x| format!("[{}].{}", std::any::type_name::<T>(), x))
//             .collect::<Vec<String>>()
//     }
// }

// impl<T1: ToCsv, T2: ToCsv> ToCsv for (T1, T2) {
//     fn to_csv(&self) -> Vec<String> {
//         let mut csvs = Vec::new();
//         csvs.extend(self.0.to_csv().iter().map(|x| format!("[0].{}", x)));
//         csvs.extend(self.1.to_csv().iter().map(|x| format!("[1].{}", x)));
//         csvs
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(
//             " = ({}, {})",
//             std::any::type_name::<T1>(),
//             std::any::type_name::<T2>()
//         )]
//     }
// }

// impl<T1: ToCsv, T2: ToCsv, T3: ToCsv> ToCsv for (T1, T2, T3) {
//     fn to_csv(&self) -> Vec<String> {
//         let mut csvs = Vec::new();
//         csvs.extend(self.0.to_csv().iter().map(|x| format!("[0].{}", x)));
//         csvs.extend(self.1.to_csv().iter().map(|x| format!("[1].{}", x)));
//         csvs.extend(self.2.to_csv().iter().map(|x| format!("[2].{}", x)));
//         csvs
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(
//             " = ({}, {}, {})",
//             std::any::type_name::<T1>(),
//             std::any::type_name::<T2>(),
//             std::any::type_name::<T2>()
//         )]
//     }
// }

// impl<T1: ToCsv, T2: ToCsv, T3: ToCsv, T4: ToCsv> ToCsv for (T1, T2, T3, T4) {
//     fn to_csv(&self) -> Vec<String> {
//         let mut csvs = Vec::new();
//         csvs.extend(self.0.to_csv().iter().map(|x| format!("[0].{}", x)));
//         csvs.extend(self.1.to_csv().iter().map(|x| format!("[1].{}", x)));
//         csvs.extend(self.2.to_csv().iter().map(|x| format!("[2].{}", x)));
//         csvs.extend(self.3.to_csv().iter().map(|x| format!("[3].{}", x)));
//         csvs
//     }
//     fn enumerate(&self) -> Vec<String> {
//         vec![format!(
//             " = ({}, {}, {}, {})",
//             std::any::type_name::<T1>(),
//             std::any::type_name::<T2>(),
//             std::any::type_name::<T2>(),
//             std::any::type_name::<T3>()
//         )]
//     }
// }

// impl<T: ToCsv> ToCsvClean for T {
//     fn to_csv_clean(&self) -> Vec<String> {
//         self.to_csv().iter().map(|x| to_csv_clean(x)).collect()
//     }

//     fn enumerate_clean(&self) -> Vec<String> {
//         self.enumerate().iter().map(|x| to_csv_clean(x)).collect()
//     }
// }

// fn to_csv_clean(s: &String) -> String {
//     let s = s.replace(".[", "[");
//     let s = s.replace(". =", " =");
//     s
// }
