// mod a_rule_of_divisibility_by_7;
// mod build_a_pile_of_cubes;
// mod build_a_square;
// mod count_the_digit;
// mod deodorant_evaporator;
// mod exes_and_ohs;
// mod find_the_smallest_integer_in_the_array;
// mod forgot_name_1;
// mod forgot_name_2;
// mod give_me_a_diamond;
// mod how_many_lightsabers_do_you_own;
// mod parabolic_arc_length;
// mod string_ends_with;
// mod two_pair_sum;
// mod color_choice;

fn epidemic(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64) -> i32 {
    let interval = (tm as f64) / (n as f64);
    let mut s: f64 = s0 as f64;
    let mut i: f64 = i0 as f64;
    let mut r: f64 = 0f64;
    let mut maxn = 0f64;
    for j in 0..n {
        let ta = s - interval * b * s * i;
        let tb = i + interval * (b * s * i - a * i);
        let tc = r + interval * i * a;
        s = ta;
        i = tb;
        r = tc;
        maxn = maxn.max(i);
    }
    println!("{}", maxn);
    return maxn as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64, exp: i32) -> () {
        assert_eq!(exp, epidemic(tm, n, s0, i0, b, a))
    }
    #[test]
    fn basic_tests() {
        dotest(18, 432, 1004, 1, 0.00209, 0.51, 420);
        dotest(12, 288, 1007, 2, 0.00206, 0.45, 461);
        dotest(13, 312, 999, 1, 0.00221, 0.55, 409);
    }
}
