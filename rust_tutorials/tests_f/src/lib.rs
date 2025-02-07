pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}

// --snip--
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    } 

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    } 

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        }; 

        assert!(!smaller.can_hold(&larger));
    }
}

/*
   - Here we're testing to see what happens to our test results when we introduce a bug in our code
   - We've change the implementation of the can_hold method by replacing the greater-than sign with a less
     than sign when it compares the widths: 
   - Our tests caught the bug! Because larger.width is 8 and smaller.width is 5, the comparison of the widths
     in can_hold now returns false:8 is not less than 5.
*/
