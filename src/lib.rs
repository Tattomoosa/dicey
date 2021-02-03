use rand::{ thread_rng, Rng };
use std::cmp::{ max, min };

pub fn roll(count: usize) -> Roll {
    Roll { count }
}

/// Represents some count of die being rolled (faces on die determined later)
pub struct Roll {
    /// How many die are rolled in this roll
    count: usize,
}

impl Roll {
    /// Represents a die roll.
    pub fn d(&self, sides: usize) -> usize {
        if sides == 0 || self.count == 0 {
            0
        } else {
            (0..self.count)
                .map(|_| thread_rng().gen_range(0..sides) + 1)
                .sum()
        }
    }

    /// Represents a critical success (e.g. 20 on one d20, 40 on 2d20)
    pub fn crit_success(&self, sides: usize) -> usize {
        if sides == 0 || self.count == 0 {
            0
        } else {
            (0..self.count)
                .map(|_| sides)
                .sum()
        }
    }

    /// Represents a critical failure (e.g. 1 on one d20, 2 on 2d20)
    pub fn crit_fail(&self, sides: usize) -> usize {
        if sides == 0 || self.count == 0 {
            0
        } else {
            (0..self.count)
                .map(|_| 1)
                .sum()
        }
    }

    pub fn advantage(&self, sides: usize) -> usize {
        if sides == 0 || self.count == 0 {
            0
        } else {
            let roll1 = self.d(sides);
            let roll2 = self.d(sides);
            max(roll1, roll2)
        }
    }

    pub fn disadvantage(&self, sides: usize) -> usize {
        if sides == 0 || self.count == 0 {
            0
        } else {
            let roll1 = self.d(sides);
            let roll2 = self.d(sides);
            min(roll1, roll2)
        }
    }


    // convenience
    pub fn d4(&self) -> usize { return self.d(4) }

    pub fn d6(&self) -> usize { return self.d(6) }

    pub fn d8(&self) -> usize { return self.d(8) }

    pub fn d10(&self) -> usize { return self.d(10) }

    pub fn d12(&self) -> usize { return self.d(12) }

    pub fn d20(&self) -> usize { return self.d(20) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_dice() {
        let result = roll(0).d(6);
        assert_eq!(result, 0);
        let result = roll(0).crit_success(6);
        assert_eq!(result, 0);
        let result = roll(0).crit_fail(6);
        assert_eq!(result, 0);
    }

    #[test]
    fn no_sides() {
        let result = roll(10).d(0);
        assert_eq!(result, 0);
        let result = roll(10).crit_success(0);
        assert_eq!(result, 0);
        let result = roll(10).crit_fail(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn roll_d2_to_d20() {
        for _ in 0..1000 {
            for n in 1..20 {
                for x in 0..10 {
                    let result = roll(x).d(n);
                    assert!(result >= x && result <= n * x);
                }
            }
        }
    }

    #[test]
    fn crit_success_d0_to_d20() {
        for _ in 0..1000 {
            for n in 0..20 {
                for x in 0..10 {
                    let result = roll(x).crit_success(n);
                    assert_eq!(result, n * x);
                }
            }
        }
    }
    #[test]
    fn crit_fail_d1_to_d20() {
        for _ in 0..1000 {
            // function works for n=0 (= 0) but test would require special case
            for n in 1..20 {
                for x in 0..10 {
                    let result = roll(x).crit_fail(n);
                    assert_eq!(result, x);
                }
            }
        }
    }
}
