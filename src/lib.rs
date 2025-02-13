use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct ULimInt {
    limit: usize,
    value: usize,
    overflow: usize,
    true_value: usize, // Just for debug, might be removed
}

impl ULimInt {
    pub fn to_int(&self) -> usize {
        self.limit * self.overflow + self.value
    }
}

// Very basic system - just add the true value
impl Add<ULimInt> for ULimInt {
    type Output = ULimInt;
    fn add(self, other: ULimInt) -> ULimInt {
        (self.true_value + other.true_value).to_limint(self.limit)
    }
}

impl Sub<ULimInt> for ULimInt {
    type Output = ULimInt;
    fn sub(self, other: ULimInt) -> ULimInt {
        (self.true_value - other.true_value).to_limint(self.limit)
    }
}

impl Mul<ULimInt> for ULimInt {
    type Output = ULimInt;
    fn mul(self, other: ULimInt) -> ULimInt {
        (self.true_value * other.true_value).to_limint(self.limit)
    }
}

impl Div<ULimInt> for ULimInt {
    type Output = ULimInt;
    fn div(self, other: ULimInt) -> ULimInt {
        (self.true_value / other.true_value).to_limint(self.limit)
    }
}

impl PartialEq for ULimInt {
    fn eq(&self, other: &ULimInt) -> bool {
        self.true_value == other.true_value
            && self.limit == other.limit
            && self.value == other.value
            && self.overflow == other.overflow
            && self.true_value == other.true_value
    }
}

pub trait ToLimInt {
    fn to_limint(&self, limit: usize) -> ULimInt;
}

impl ToLimInt for usize {
    fn to_limint(&self, limit: usize) -> ULimInt {
        let overflow = self / limit;
        let value = self % limit;

        ULimInt {
            limit,
            value,
            overflow,
            true_value: overflow * limit + value,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::process::id; // Use procid for prng (reason below)

    use crate::{ToLimInt, ULimInt};

    // Quick and dirty prng for testing using xorshift and the procid
    fn cheap_rand(seed: usize) -> usize {
        let mut state: usize = seed;

        for _ in 1..10 {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
        }
        return state % 1000;
    }

    #[test]
    fn test_to_limint() {
        let limit = 5usize;
        let value = 10usize;
        let _ulimint = value.to_limint(limit);
    }

    #[test]
    fn test_true_value() {
        let limit = cheap_rand(id() as usize);
        let value = cheap_rand(limit);
        let ulimint = value.to_limint(limit);
        assert_eq!(ulimint.true_value, value);
    }
    #[test]
    fn to_int() {
        let mut rand = id() as usize;

        for _ in 1..1000 {
            rand = cheap_rand(rand);
            let limit = cheap_rand(rand);
            rand = cheap_rand(rand);
            let value = cheap_rand(rand);
            let ulimint = value.to_limint(limit);
            assert_eq!(ulimint.true_value, ulimint.to_int());
        }
    }

    fn create_limint(seed: Option<usize>) -> ULimInt {
        let mut rand = match seed {
            Some(val) => val,
            None => id() as usize,
        };
        rand = cheap_rand(rand);
        let limit = cheap_rand(rand);
        rand = cheap_rand(rand);
        let value = cheap_rand(rand);
        value.to_limint(limit)
    }
    #[test]
    fn test_add_limint() {
        let mut ulimints: Vec<ULimInt> = vec![];

        ulimints.push(create_limint(None));
        ulimints.push(create_limint(Some(ulimints[0].value)));


        let resultant = ulimints[0] + ulimints[1];
        println!("{:?}", ulimints);
        println!("{:?}", resultant);
    }

}
