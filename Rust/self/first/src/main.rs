/* Merge Sort
 * @fileoverview
 * Currently works on a single layer of sorted integers
 * Follows DRY principle
 * Orthogonalization left
 */

fn main() {
    let mut list = [2, 3, 4, 1, 2];
    println!("Hello, {:?}", m_sort(&mut list));
}

fn m_sort(list: &mut [i32]) -> &mut [i32] {
    // TODO: Account for if the list size is 1 and 2
    if list.len() == 1 {
        return list;
    }

    if list.len() == 2 {
        if list[0] > list[1] {
            list.swap(0, 1);
        }
        return list;
    }

    // If the length is more than one element
    let middle_index = get_middle_index(&list);
    let left_half = &list[0..middle_index+1];
    let right_half = &list[middle_index+1..list.len()];

    // TODO: recursively call `m_sort` here

    // Get an array sized like the original one
    let mut copy: Box<[i32]> = list.to_vec().into_boxed_slice();
    let finlist = copy.as_mut();

    // Merge the left half with the right half
    let mut left_index = 0;
    let mut right_index = 0;
    let mut fin_index: usize = 0;

    let mut fin_elem: i32;
    while (left_index < left_half.len()) && (right_index < right_half.len()) {
        if left_half[left_index] < right_half[right_index] {
            fin_elem = left_half[left_index];
            left_index += 1;
        } else {
            fin_elem = right_half[right_index];
            right_index += 1;
        }
        finlist[fin_index] = fin_elem;
        fin_index += 1;
    }
    
    // Add remaining values
    if left_index >= left_half.len() {
        while right_index < right_half.len() {
            finlist[fin_index] = right_half[right_index];
            fin_index += 1;
            right_index += 1;
        }
    }

    else {
        while left_index < left_half.len() {
            println!("left_half[left_index]: {}", left_half[left_index]);
            finlist[fin_index] = left_half[left_index];
            fin_index += 1;
            left_index += 1;
        }
    }

    // Put it in the `list` variable
    for index in 0..list.len() {
        list[index] = finlist[index];
    }

    list
}

fn get_middle_index(list: &[i32]) -> usize {
    let is_even: bool = if list.len()%2 == 0 {true} else {false};
    let middle_index = if is_even {(list.len()/2) - 1} else {list.len()/2};
    middle_index
}

