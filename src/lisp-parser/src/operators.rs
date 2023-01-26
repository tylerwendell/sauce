use std::fmt;
use std::error::Error;

#[derive(PartialEq, Debug, Clone)]
pub enum Primitive {
    Integer(i32),
    DoublePrecisionFloat(f64),
    Str(String),
    Vector(Vec<Primitive>),
}
impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Primitive::Integer(item) => write!(f, "{}", item),
            Primitive::DoublePrecisionFloat(item) => write!(f, "{}", item),
            Primitive::Str(item) => write!(f, "\'{}\'", item),
            Primitive::Vector(item) => {
                write!(f, "[")?;
                for (i, e) in item.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            },
        }
    }
}


use std::ops;

#[derive(Debug)]
pub struct OperationError {
    message: String,
}


impl Error for OperationError {}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl From<&str> for OperationError {
    fn from(message: &str) -> Self {
        OperationError { message: message.to_string() }
    }
}

impl ops::Add for Primitive {
	type Output = Primitive;

    fn add(self, other: Primitive) -> Primitive {
        match (self, other) {
            (Primitive::Integer(x), Primitive::Integer(y)) => Primitive::Integer(x + y),
            (Primitive::DoublePrecisionFloat(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x + y),
            (Primitive::Str(x), Primitive::Str(y)) => Primitive::Str(x + &y),
            (Primitive::Vector(a), Primitive::Vector(b)) => {
                let mut result = Vec::new();
                let mut longer = a.clone();
                let mut shorter = b.clone();
                if a.len() < b.len() {
                    longer = b.clone();
                    shorter = a.clone();
                }
                for (x, y) in shorter.iter().zip(longer.iter()) {
                    result.push(x.clone() + y.clone());
                }
                result.extend(longer[shorter.len()..].iter().cloned());
                Primitive::Vector(result)
            },
            (Primitive::Integer(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x as f64 + y),
            (Primitive::DoublePrecisionFloat(x), Primitive::Integer(y)) => Primitive::DoublePrecisionFloat(x + y as f64),
            (Primitive::Integer(x), Primitive::Str(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::Str(x), Primitive::Integer(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::DoublePrecisionFloat(x), Primitive::Str(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::Str(x), Primitive::DoublePrecisionFloat(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::Integer(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() + Primitive::Integer(value)).collect()),
            (Primitive::DoublePrecisionFloat(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() + Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Str(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() + Primitive::Str(value.clone())).collect()),
            (Primitive::Vector(y), Primitive::Integer(value)) => Primitive::Vector(y.iter().map(|x| x.clone() + Primitive::Integer(value)).collect()),
            (Primitive::Vector(y), Primitive::DoublePrecisionFloat(value)) => Primitive::Vector(y.iter().map(|x| x.clone() + Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Vector(y), Primitive::Str(value)) => Primitive::Vector(y.iter().map(|x| x.clone() + Primitive::Str(value.clone())).collect()),
    }
    }
}

impl ops::Sub for Primitive {
	type Output = Primitive;

    fn sub(self, other: Primitive) -> Primitive {
        match (self, other) {
            (Primitive::Integer(x), Primitive::Integer(y)) => Primitive::Integer(x - y),
            (Primitive::DoublePrecisionFloat(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x - y),
            (Primitive::Str(x), Primitive::Str(y)) => Primitive::Str(x.replace(&y,"")),
            (Primitive::Vector(a), Primitive::Vector(b)) => {
                let mut result = Vec::new();
                let mut longer = a.clone();
                let mut shorter = b.clone();
                if a.len() < b.len() {
                    longer = b.clone();
                    shorter = a.clone();
                }
                for (x, y) in shorter.iter().zip(longer.iter()) {
                    result.push(x.clone() - y.clone());
                }
                result.extend(longer[shorter.len()..].iter().cloned());
                Primitive::Vector(result)
            },
            (Primitive::Integer(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x as f64 - y),
            (Primitive::DoublePrecisionFloat(x), Primitive::Integer(y)) => Primitive::DoublePrecisionFloat(x - y as f64),
            (Primitive::Integer(x), Primitive::Str(y)) => Primitive::Str((x.to_string()).replace(&y,"")),
            (Primitive::Str(x), Primitive::Integer(y)) => Primitive::Str(x.replace(&y.to_string(),"")),
            (Primitive::DoublePrecisionFloat(x), Primitive::Str(y)) => Primitive::Str((x.to_string()).replace(&y,"")),
            (Primitive::Str(x), Primitive::DoublePrecisionFloat(y)) => Primitive::Str(x.replace(&y.to_string(),"")),
            (Primitive::Integer(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() - Primitive::Integer(value)).collect()),
            (Primitive::DoublePrecisionFloat(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() - Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Str(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() - Primitive::Str(value.clone())).collect()),
            (Primitive::Vector(y), Primitive::Integer(value)) => Primitive::Vector(y.iter().map(|x| x.clone() - Primitive::Integer(value)).collect()),
            (Primitive::Vector(y), Primitive::DoublePrecisionFloat(value)) => Primitive::Vector(y.iter().map(|x| x.clone() - Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Vector(y), Primitive::Str(value)) => Primitive::Vector(y.iter().map(|x| x.clone() - Primitive::Str(value.clone())).collect()),
    }
    }
}

impl ops::Mul for Primitive {
	type Output = Primitive;

    fn mul(self, other: Primitive) -> Primitive {
        match (self, other) {
            (Primitive::Integer(x), Primitive::Integer(y)) => Primitive::Integer(x * y),
            (Primitive::DoublePrecisionFloat(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x * y),
            (Primitive::Str(a), Primitive::Str(b)) => {
                let mut result = String::new();
                for (x, y) in a.chars().zip(b.chars()) {
                    result.push(x);
                    result.push(y);
                }
                if a.len() > b.len() {
                    result.push_str(&a[b.len()..]);
                }
                if b.len() > a.len() {
                    result.push_str(&a[b.len()..]);
                }
                
                Primitive::Str(result)
            },
            (Primitive::Vector(a), Primitive::Vector(b)) => {
                let mut result = Vec::new();
                let mut longer = a.clone();
                let mut shorter = b.clone();
                if a.len() < b.len() {
                    longer = b.clone();
                    shorter = a.clone();
                }
                for (x, y) in shorter.iter().zip(longer.iter()) {
                    result.push(x.clone() * y.clone());
                }
                result.extend(longer[shorter.len()..].iter().cloned());
                Primitive::Vector(result)
            },
            (Primitive::Integer(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x as f64 * y),
            (Primitive::DoublePrecisionFloat(x), Primitive::Integer(y)) => Primitive::DoublePrecisionFloat(x * y as f64),
            (Primitive::Integer(x), Primitive::Str(y)) => Primitive::Str(y.repeat(x as usize)),
            (Primitive::Str(x), Primitive::Integer(y)) => Primitive::Str(x.replace(&y.to_string(),"")),
            (Primitive::DoublePrecisionFloat(x), Primitive::Str(y)) => {
                let i = x.ceil() as i32;
                let result = y.repeat(i as usize);
                let fraction = (x/x.ceil()) * result.len() as f64;
                let final_pos = fraction.floor() as i32;
                let final_u = final_pos as usize;
                Primitive::Str(result[0..final_u].to_string())
            },
            (Primitive::Str(y), Primitive::DoublePrecisionFloat(x)) => {
                let i = x.ceil() as i32;
                let result = y.repeat(i as usize);
                let fraction = (x/x.ceil()) * result.len() as f64;
                let final_pos = fraction.floor() as i32;
                let final_u = final_pos as usize;
                Primitive::Str(result[0..final_u].to_string())
            },
            (Primitive::Integer(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() * Primitive::Integer(value)).collect()),
            (Primitive::DoublePrecisionFloat(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() * Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Str(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() * Primitive::Str(value.clone())).collect()),
            (Primitive::Vector(y), Primitive::Integer(value)) => Primitive::Vector(y.iter().map(|x| x.clone() * Primitive::Integer(value)).collect()),
            (Primitive::Vector(y), Primitive::DoublePrecisionFloat(value)) => Primitive::Vector(y.iter().map(|x| x.clone() * Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Vector(y), Primitive::Str(value)) => Primitive::Vector(y.iter().map(|x| x.clone() * Primitive::Str(value.clone())).collect()),
    }
    }
}

impl ops::Div for Primitive {
	type Output = Primitive;

    fn div(self, other: Primitive) -> Primitive {
        match (self, other) {
            (Primitive::Integer(x), Primitive::Integer(y)) => Primitive::Integer(x / y),
            (Primitive::DoublePrecisionFloat(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x / y),
            (Primitive::Str(a), Primitive::Str(b)) => {
                let mut result = Vec::new();
                let words: Vec<&str> = a.split(&b).collect();
                for s in words {
                    result.push(Primitive::Str(s.to_string()));
                }
                Primitive::Vector(result)
            },
            (Primitive::Vector(a), Primitive::Vector(b)) => {
                let mut result = Vec::new();
                let mut longer = a.clone();
                let mut shorter = b.clone();
                if a.len() < b.len() {
                    longer = b.clone();
                    shorter = a.clone();
                }
                for (x, y) in shorter.iter().zip(longer.iter()) {
                    result.push(x.clone() / y.clone());
                }
                result.extend(longer[shorter.len()..].iter().cloned());
                Primitive::Vector(result)
            },
            (Primitive::Integer(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x as f64 / y),
            (Primitive::DoublePrecisionFloat(x), Primitive::Integer(y)) => Primitive::DoublePrecisionFloat(x / y as f64),
            (Primitive::Integer(x), Primitive::Str(y)) => {
                let sub_len = y.len() / x as usize;
                let mut result = String::new();
                for (i, c) in y.chars().enumerate() {
                    if i <= sub_len {
                        result.push(c);
                    }
                }
                Primitive::Str(result)
            },
            (Primitive::Str(y), Primitive::Integer(x)) => {
                let sub_len = y.len() / x as usize;
                let mut result = String::new();
                for (i, c) in y.chars().enumerate() {
                    if i <= sub_len {
                        result.push(c);
                    }
                }
                Primitive::Str(result)
            },
            (Primitive::DoublePrecisionFloat(x), Primitive::Str(y)) => {
                let l = y.len() as i32;
                let sub_len = l as f64 / x;
                let size = sub_len.ceil() as i32 as usize;
                let mut result = String::new();
                for (i, c) in y.chars().enumerate() {
                    if i <= size {
                        result.push(c);
                    }
                }
                Primitive::Str(result)
            },
            (Primitive::Str(y), Primitive::DoublePrecisionFloat(x)) => {
                let l = y.len() as i32;
                let sub_len = l as f64 / x;
                let size = sub_len.ceil() as i32 as usize;
                let mut result = String::new();
                for (i, c) in y.chars().enumerate() {
                    if i <= size {
                        result.push(c);
                    }
                }
                Primitive::Str(result)
            },
            (Primitive::Integer(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() / Primitive::Integer(value)).collect()),
            (Primitive::DoublePrecisionFloat(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() / Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Str(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| x.clone() / Primitive::Str(value.clone())).collect()),
            (Primitive::Vector(y), Primitive::Integer(value)) => Primitive::Vector(y.iter().map(|x| x.clone() / Primitive::Integer(value)).collect()),
            (Primitive::Vector(y), Primitive::DoublePrecisionFloat(value)) => Primitive::Vector(y.iter().map(|x| x.clone() / Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Vector(y), Primitive::Str(value)) => Primitive::Vector(y.iter().map(|x| x.clone() / Primitive::Str(value.clone())).collect()),
    }
    }
}