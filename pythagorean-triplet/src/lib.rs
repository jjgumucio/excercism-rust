// Struct to generate numbers
#[derive(Copy, Clone)]
struct Generator {
    number: u32,
    limit: u32,
}

impl Generator {
    fn new(limit: u32) -> Generator {
        Generator {
            number: 1,
            limit
        }
    }
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.number += 1;

        if self.number < self.limit {
            Some(self.number)
        } else {
            None
        }
    }
}

pub fn find() -> Option<u32> {
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;

    let generator_a = Generator::new(1000);
    let generator_b = Generator::new(1000);
    let generator_c = Generator::new(1000);

    for x in generator_a {
        for y in generator_b {
            for z in generator_c {
                if x + y + z == 1000 {
                    if x.pow(2) + y.pow(2) == z.pow(2) {
                        a = x;
                        b = y;
                        c = z;
                        break;
                    }
                }
            }
        }
    }
    Some(a * b * c)
}
