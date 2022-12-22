
#[derive(PartialEq, Debug, Clone)]
pub enum Primitive {
    Integer(i32),
    DoublePrecisionFloat(f64),
    Str(String),
    Vector(Vec<Primitive>),
}

use std::ops;
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
                    result.push(*x + *y);
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
            (Primitive::Integer(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| *x + Primitive::Integer(value)).collect()),
            (Primitive::DoublePrecisionFloat(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| *x + Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Str(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| *x + Primitive::Str(value)).collect()),
            (Primitive::Vector(y), Primitive::Integer(value)) => Primitive::Vector(y.iter().map(|x| *x + Primitive::Integer(value)).collect()),
            (Primitive::Vector(y), Primitive::DoublePrecisionFloat(value)) => Primitive::Vector(y.iter().map(|x| *x + Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Vector(y), Primitive::Str(value)) => Primitive::Vector(y.iter().map(|x| *x + Primitive::Str(value)).collect()),
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
                    result.push(*x - *y);
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
            (Primitive::Integer(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| *x - Primitive::Integer(value)).collect()),
            (Primitive::DoublePrecisionFloat(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| *x + Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Str(value), Primitive::Vector(y)) => Primitive::Vector(y.iter().map(|x| *x - Primitive::Str(value)).collect()),
            (Primitive::Vector(y), Primitive::Integer(value)) => Primitive::Vector(y.iter().map(|x| *x - Primitive::Integer(value)).collect()),
            (Primitive::Vector(y), Primitive::DoublePrecisionFloat(value)) => Primitive::Vector(y.iter().map(|x| *x - Primitive::DoublePrecisionFloat(value)).collect()),
            (Primitive::Vector(y), Primitive::Str(value)) => Primitive::Vector(y.iter().map(|x| *x - Primitive::Str(value)).collect()),
    }
    }
}

impl ops::Mul for Primitive {
	type Output = Primitive;

    fn add(self, other: Primitive) -> Primitive {
        match (self, other) {
            (Primitive::Integer(x), Primitive::Integer(y)) => Primitive::Integer(x + y),
            (Primitive::DoublePrecisionFloat(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x + y),
            (Primitive::Str(x), Primitive::Str(y)) => Primitive::Str(x + &y),
            (Primitive::Integer(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x as f64 + y),
            (Primitive::DoublePrecisionFloat(x), Primitive::Integer(y)) => Primitive::DoublePrecisionFloat(x + y as f64),
            (Primitive::Integer(x), Primitive::Str(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::Str(x), Primitive::Integer(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::DoublePrecisionFloat(x), Primitive::Str(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::Str(x), Primitive::DoublePrecisionFloat(y)) => Primitive::Str(format!("{}{}", x, y)),
    }
    }
}

impl ops::Div for Primitive {
	type Output = Primitive;

    fn add(self, other: Primitive) -> Primitive {
        match (self, other) {
            (Primitive::Integer(x), Primitive::Integer(y)) => Primitive::Integer(x + y),
            (Primitive::DoublePrecisionFloat(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x + y),
            (Primitive::Str(x), Primitive::Str(y)) => Primitive::Str(x + &y),
            (Primitive::Integer(x), Primitive::DoublePrecisionFloat(y)) => Primitive::DoublePrecisionFloat(x as f64 + y),
            (Primitive::DoublePrecisionFloat(x), Primitive::Integer(y)) => Primitive::DoublePrecisionFloat(x + y as f64),
            (Primitive::Integer(x), Primitive::Str(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::Str(x), Primitive::Integer(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::DoublePrecisionFloat(x), Primitive::Str(y)) => Primitive::Str(format!("{}{}", x, y)),
            (Primitive::Str(x), Primitive::DoublePrecisionFloat(y)) => Primitive::Str(format!("{}{}", x, y)),
    }
    }
}