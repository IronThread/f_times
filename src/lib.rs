//! A simple rust library offering a way to do something float-like number of times.

pub struct FTimes {
    trunc_part: f64,
    fract_part: f64,
    x: f64,
    y: f64,
}

impl FTimes {
    pub fn new(times: f64) -> Self {
        let x =  times.trunc();

        Self {
            trunc_part: x,
            fract_part: times.fract(),
            x,
            y: 0.0,
        }
    }

    pub fn next(&mut self) -> bool {
        if self.x < 1.0 {
            self.x = self.trunc_part;
            true
        } else {
            self.x -= self.trunc_part;
            self.y += self.fract_part;

            if self.y >= 1.0 {
                self.y -= 1.0;
                self.x += 1.0;
            } 

            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn a() {
        let mut num = 0;
    
        let mut e = FTimes::new(2.5);

        for _ in 0..5 {
            if e.next() {
                num += 1;
            }
        }

        assert_eq!(num, 2);

        for _ in 0..2 {
            if e.next() {
                num += 1;
            }
        }

        assert_eq!(num, 3);

        for _ in 0..3 {
            if e.next() {
                num += 1;                
            }
        }

        assert_eq!(num, 4);
    }

    #[test]
    fn b() {
        let mut num = 0;       
    
        let mut e = FTimes::new(PI);

        for _ in 0..100 {
            if e.next() {
                num += 1;
            }
        }

        assert_eq!(num, 53);
    }

}
