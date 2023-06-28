use crate::op::Operation;

#[derive(Debug, Clone)]
pub struct Warrior {
    pub start: usize,
    pub body: Vec<Operation>,
}

impl Warrior {
    pub fn random_create(size: isize, core_size: isize) -> Self {
        let mut body = vec![];

        for i in 0..size {
            body.push(Operation::get_random(i.min(size - i), core_size))
        }

        Warrior {
            start: rand::random::<usize>() % size as usize,
            body,
        }
    }

    pub fn parse(str: String) -> Result<Self, (usize, String)> {
        let mut str = str.to_uppercase();

        let mut body = vec![];
        let mut start = None;

        for (i, line) in str.split('\n').enumerate() {
            if line == "ORG" {
                if let None = start {
                    start = Some(i);
                } else {
                    return Err((i, "multiple ORG pseudoinstructions found".into()));
                }
            } else {
                match Operation::parse(line.into()) {
                    Ok(op) => body.push(op),
                    Err(err) => return Err((i, err)),
                }
            }
        }

        Ok(Self {
            start: start.unwrap_or_else(|| 0),
            body,
        })
    }
}
