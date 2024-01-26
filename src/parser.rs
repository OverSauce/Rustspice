#[derive(Debug)]
enum SpiceSrc {
    DCValue(f64), JustValue(f64)
}

#[derive(Debug)]
#[allow(dead_code)]
struct SpiceLine {
  name: String,
  nets: Vec<u8>,
  value: SpiceSrc
}

#[derive(Debug)]
pub struct SpiceContext {
  line: Vec<SpiceLine>,
  finalized: bool,
}

impl SpiceContext {
    pub fn new() -> Self {
        Self {line: Vec::new(), finalized: false}
    }
    fn add_line(&mut self, line: SpiceLine) {
        self.line.push(line);
    }
    pub fn add_multiple(mut self, lines: String) -> Self {
        let tokens: Vec<&str> = lines.split(" ").collect();

        let mut idx: usize = 0;
        loop {
            let tks = &tokens.get(idx..);
            let tokens = {
                match tks {
                    Some(v) => v.to_vec(),
                    None => break,
                }
            };

            idx += {
                match self.from_tokens(tokens) {
                    Ok(i) => i,
                    Err(_) => break,
                }
            };
        }
        Self {..self}
    }
    fn from_tokens(&mut self, parsed: Vec<&str>) -> Result<usize, Box<dyn std::error::Error>> {
        let skip: usize;
        
        if parsed.len() < 4 { 
            if parsed.len() > 0 {
                let token = parsed[0];
                match token {
                    "end" => {
                        self.finalized = true;
                    },
                    _ => panic!("parsed must be at least 4 elements long or include 'end' token"),
                }
            }
            return Ok(1);
        }

        let name = parsed[0];
        let net1 = parsed[1].parse()?;
        let net2 = parsed[2].parse()?;

        let value = match parsed[3] {
            "dc" => {
                skip = 5;
                SpiceSrc::DCValue(parsed[4].parse()?)
            },
            _ => {
                skip = 4;
                SpiceSrc::JustValue(parsed[3].parse()?)
            },
        };
        
        self.add_line(
            SpiceLine::new()
                .name(name)
                .nets(net1, net2)
                .value(value)
        );
        
        Ok(skip)
    }
}

impl SpiceLine {
    pub fn new() -> Self {
        Self {name: "v1".into(), nets: Vec::from([1, 0]), value: SpiceSrc::DCValue(1.0)}
    }
    pub fn name(self, name: impl ToString) -> Self {
        Self {name: name.to_string(), ..self}
    }
    pub fn nets(self, net1: u8, net2: u8) -> Self {
        Self {nets: Vec::from([net1, net2]), ..self}
    }
    pub fn value(self, value: SpiceSrc) -> Self {
        Self {value, ..self}
    }
}

#[macro_export]
macro_rules! spice_line {
    ($name:ident $net1:literal $net2:literal $value:expr) => {
        SpiceLine::new()
            .name(stringify!($name))
            .nets($net1, $net2)
            .value(SpiceSrc::JustValue($value))
    };
    ($name:ident $net1:literal $net2:literal dc $value:expr) => {
        SpiceLine::new()
            .name(stringify!($name))
            .nets($net1, $net2)
            .value(SpiceSrc::DCValue($value))
    };
}

#[macro_export]
macro_rules! spice {
    ($($pat:pat)+) => {
        SpiceContext::new()
            .add_multiple(
                stringify!($($pat)+).to_string()
        )
    };
}