// Attempting to access the element at index 100 in a vector containing five elements

fn main() { 

    let v = vec![1, 2, 3, 4, 5]; 

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
