/// predict if two matrices contains same value
/// takes two slices, a and b, as input and returns a boolean value. The slices can contain elements of any type T
///  that implements the Eq trait (which allows elements of the type to be compared for equality).
///The function first checks if the lengths of a and b are equal. If they are not, the function returns false immediately. 
///If the lengths are equal, the function enters a loop that iterates over the elements of a. For each element, it checks 
///if the element is equal to the corresponding element in b using the != operator. If any elements are not equal, the function 
///returns false.
///If the function makes it through the loop without returning false, it means that all the elements of a and b are equal,
///and the function returns true.
///This function can be used to determine if two slices contain the same elements in the same order.
pub fn equal_slice<T>(a: &[T], b: &[T]) -> bool where T: Eq {
  if a.len() != b.len() {
    return false;
  } else {
    for (i, x) in a.iter().enumerate() {
      if *x != b[i] {
        return false;
      }
    }
  }

  true
  
}