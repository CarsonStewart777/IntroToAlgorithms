// implement of merge sort
// Divide: Divide the n-element sequence to be sorted into two subsequences of n/2 elements each
// Conquer: Sort the two subseqeunces recursively using merge sort
// Combine: Merge the two sorted subsequenced to produce the sorted answer


fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        vec.to_vec()
        
    } else {
        let size = vec.len() /2; // divide the vector in half
        let left = merge_sort(&vec[0..size].to_vec()); // take a vector slice from start to center
        let right = merge_sort(&vec[size..].to_vec()); // take a vector slice from center to end
        let merged = merge(&left, & right);

        merged 

    }
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut i = 0; // Index for left vector
    let mut j = 0; // Index for right vector
    let mut merged: Vec<i32> = Vec::new(); // New vector we are building

    while i < left.len() && j < right.len() { // Checking if vectors still have elements to compare

     if left[i] < right[j] { // compare the current elements (left and right)
        merged.push(left[i]); // if left is less than right, push it to our new vector
        i+=1; // move index to next element
    } else {
        merged.push(right[j]); // j is smaller or equal so we add it to the vector.
        j+=1;
    }

}

    // loops for cleanup/leftovers
    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i+=1;
        }
    }
    if j < right.len() {
        while j < right.len() {
        merged.push(right[j]);
        j+=1; }
    }
    merged // output the merged vector ! nyaa
}

fn main() {
    let numbers: Vec<i32> = vec![5, 1, 6, 7, 1, 3, 5, 9, 0, 1, 2, 4, 8, 2, 3, 1, 4, 9, 0];
    println!("Original numbers: {:?}", numbers);

    let sorted_numbers = merge_sort(&numbers);

    println!("Sorted numbers: {:?}", sorted_numbers);

}