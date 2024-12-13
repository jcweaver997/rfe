extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

pub trait ToCsv {
    fn to_csv(&self) -> Vec<String>;
}

pub trait ToCsvClean {
    fn to_csv_clean(&self) -> Vec<String>;
}

impl ToCsv for u8 {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for i8 {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for u16 {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for i16 {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for u32 {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for i32 {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for u64 {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for i64 {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for &str {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for String {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl ToCsv for bool {
    fn to_csv(&self) -> Vec<String> {
        vec![format!(" = {}", self)]
    }
}

impl<T: ToCsv> ToCsv for Vec<T> {
    fn to_csv(&self) -> Vec<String> {
        self.iter()
            .enumerate()
            .map(|(i, x)| {
                x.to_csv()
                    .iter()
                    .map(|a| format!("[{}].{}", i, a))
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect()
    }
}

impl<T: ToCsv, const N: usize> ToCsv for [T; N] {
    fn to_csv(&self) -> Vec<String> {
        self.iter()
            .enumerate()
            .map(|(i, x)| {
                x.to_csv()
                    .iter()
                    .map(|a| format!("[{}].{}", i, a))
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect()
    }
}

impl<T1: ToCsv, T2: ToCsv> ToCsv for (T1, T2) {
    fn to_csv(&self) -> Vec<String> {
        let mut csvs = Vec::new();
        csvs.extend(self.0.to_csv().iter().map(|x| format!("[0].{}", x)));
        csvs.extend(self.1.to_csv().iter().map(|x| format!("[1].{}", x)));
        csvs
    }
}

impl<T1: ToCsv, T2: ToCsv, T3: ToCsv> ToCsv for (T1, T2, T3) {
    fn to_csv(&self) -> Vec<String> {
        let mut csvs = Vec::new();
        csvs.extend(self.0.to_csv().iter().map(|x| format!("[0].{}", x)));
        csvs.extend(self.1.to_csv().iter().map(|x| format!("[1].{}", x)));
        csvs.extend(self.2.to_csv().iter().map(|x| format!("[2].{}", x)));
        csvs
    }
}

impl<T1: ToCsv, T2: ToCsv, T3: ToCsv, T4: ToCsv> ToCsv for (T1, T2, T3, T4) {
    fn to_csv(&self) -> Vec<String> {
        let mut csvs = Vec::new();
        csvs.extend(self.0.to_csv().iter().map(|x| format!("[0].{}", x)));
        csvs.extend(self.1.to_csv().iter().map(|x| format!("[1].{}", x)));
        csvs.extend(self.2.to_csv().iter().map(|x| format!("[2].{}", x)));
        csvs.extend(self.3.to_csv().iter().map(|x| format!("[3].{}", x)));
        csvs
    }
}

impl<T: ToCsv> ToCsvClean for T {
    fn to_csv_clean(&self) -> Vec<String> {
        self.to_csv().iter().map(|x| to_csv_clean(x)).collect()
    }
}

fn to_csv_clean(s: &String) -> String {
    let s = s.replace(".[", "[");
    let s = s.replace(". =", " =");
    s
}
