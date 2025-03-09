// TODO: Define a function named `sum` that takes a reference to a slice of `u32` and returns the sum of all
//  elements in the slice.

fn sum(A : &[u32]) -> u32{
    let mut aux: u32 = 0;
    for i in A {
        aux = aux + i;
    }
    aux
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
