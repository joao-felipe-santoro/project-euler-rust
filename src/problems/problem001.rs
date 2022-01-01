/** 
* https://projecteuler.net/problem=1
* If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
* Find the sum of all the multiples of 3 or 5 below 1000.
*/
pub fn solve(num: i32 ) -> i32 {
    let mut sum = 0;

    for i in 3..num {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::problems::problem001;
    #[test]
    fn solve10() {
        assert_eq!(problem001::solve(10), 23);
    }
    #[test]
    fn solve1000() {
        assert_eq!(problem001::solve(1000), 233168);
    }
}