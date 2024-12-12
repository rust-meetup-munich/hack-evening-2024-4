#[derive(Debug,Clone)]
pub struct Weather {
    pub count: u64,
    pub sum: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl Weather {
    pub fn new() -> Self {
        Weather { count: 0, sum: 0.0, min: None, max: None }
    }

    pub fn add(&mut self, value: f64) {
        match self.min {
            Some(min) => if value < min { self.min = Some(value) }
            None => self.min = Some(value)
        }

        match self.max {
            Some(max) => if value > max { self.max = Some(value) }
            None => self.max = Some(value)
        }

        self.sum += value;
        self.count += 1;
    }

    pub fn summarize(&self, name: &str) -> String {
        let min = self.min.unwrap_or(0.0);
        let max = self.max.unwrap_or(0.0);
        let mean = self.sum / (self.count as f64);

        format!("{}={:.1}/{:.1}/{:.1}", name, min, mean, max)
    }
}