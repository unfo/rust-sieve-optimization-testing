#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub fn sieve_u8() -> usize {
    let mut count: usize = 0;
    const MAX: usize = 8192;
    let mut flags: [u8; MAX] = [0; MAX];
    for i in 2..MAX {
        flags[i] = 1;
    }
    for num in 2..MAX {
        if flags[num] == 1 {
            // for k in ((i+i)..MAX).step_by(i) {
            //     flags[k] = 0;
            // }
            'mul: for factor in num .. {
                let product = num * factor;
                if product >= MAX {
                    break 'mul;
                }
                flags[product] = 0;
            }  
            count += 1;
        }
    }
    count
    // println!("Count: {}", count);
}
pub fn sieve_u8_init()  -> usize {
    let mut count: usize = 0;
    const MAX: usize = 8192;
    let mut flags: [u8; MAX] = [1; MAX];
    for num in 2..MAX {
        if flags[num] == 1 {
            'mul: for factor in num .. {
                let product = num * factor;
                if product >= MAX {
                    break 'mul;
                }
                flags[product] = 0;
            }  
            count += 1;
        }
    }
    count
    // println!("Count: {}", count);
}

pub fn sieve_u8_d()  -> usize {
    let mut count: usize = 0;
    const MAX: usize = 8192;
    let mut flags: [u8; MAX + 1] = [0; MAX+1];
    for i in &mut flags[2..MAX] {
        *i = 1;
    }
    for num in 2..MAX {
        if flags[num] == 1 {
            let mut product = num * num;
            while product < MAX {
                flags[product] = 0;
                product += num;
            }
            count += 1;
        }
    }
    count
    // println!("Count: {}", count);
}

pub fn sieve_bools()  -> usize {
    let mut count: usize = 0;
    const MAX: usize = 8192;
    let mut flags: [bool; MAX ] = [true; MAX];
    for num in 2..MAX {
        if flags[num] {
            'mul: for factor in num .. {
                let product = num * factor;
                if product >= MAX {
                    break 'mul;
                }
                flags[product] = false;
            }  
            count += 1;
        }
    }
    count
    // println!("Count: {}", count);
}

pub fn sieve_bools_dist()  -> usize {
    let mut count: usize = 0;
    const MAX: usize = 8192;
    let mut flags: [bool; MAX ] = [true; MAX];
    for num in 2..MAX {
        if flags[num] {
            let mut product = num * num;
            while product < MAX {
                flags[product] = false;
                product += num;
            } 
            count += 1;
        }
    }
    count
    // println!("Count: {}", count);
}