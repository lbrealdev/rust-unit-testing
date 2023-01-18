pub fn mid(left: i32, right: i32) -> i32 {
    if left == right {
        return left;
    } else if right > left {
        return mid(right, left);
    }

    let diff = right - left;
    left + (diff / 2)
}



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn mid_works() {
      let result = mid(1, 3);
      assert_eq!(result, 2);
    }
    
    #[test]
    fn mid_works2() {
      let result = mid(1, 3);
      assert!(result > 1);
      assert!(result < 3);
      assert_eq!(result, 2);
    }
    
    #[test]
    fn mid_works3() {
      let result = mid(40, 20);
      assert_eq!(result, 30);
    }
    
}
