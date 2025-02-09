/// Return `true` if `n` is even, `false` otherwise.
fn is_even(n: u32) -> bool {
    //todo!()
    let aux1: u32 = n/2u32;
    let aux2: u32 = (n+1u32)/2u32;
    let mut resp;
    if aux1 == aux2 {
        resp=true;
    }else{
        resp=false;
    }
    return resp
}

#[cfg(test)]
mod tests {
    use crate::is_even;

    #[test]
    fn one() {
        assert!(!is_even(1));
    }

    #[test]
    fn two() {
        assert!(is_even(2));
    }

    #[test]
    fn high() {
        assert!(!is_even(231));
    }
}
