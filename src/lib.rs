
#[derive(Debug)]
pub struct ULimInt {
    limit: usize,
    value: usize,
    overflow: usize,
    true_value: usize // Just for debug, might be removed
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
            true_value: overflow * limit + value
        }

    }
}

#[cfg(test)]
mod tests {
    use std::process::id; // Use procid for prng (reason below)

    use crate::ToLimInt;

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
        let ulimint = value.to_limint(limit);
        println!("{:?}", ulimint)
    }

    #[test]
    fn test_true_value() {
        let limit = cheap_rand(id() as usize);
        let value = cheap_rand(limit);
        let ulimint = value.to_limint(limit);
        assert_eq!(ulimint.true_value, value);
        println!("{:?}\n\n {}", ulimint, value);
    }

}