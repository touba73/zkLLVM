#![no_main]

use ark_pallas::Fq as PallasBase;

fn fib100(a: PallasBase, b: PallasBase) -> PallasBase {
    let mut prev_target = a;
    let mut cur_target = b;

    for _ in 0..99 {
        (prev_target, cur_target) = (cur_target, prev_target + cur_target);
    }

    cur_target
}

#[circuit]
pub fn verify_f100(a: PallasBase, b: PallasBase, f100: PallasBase) -> bool {
    fib100(a,b) == f100
}
