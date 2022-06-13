use std::fs::{File, self};
use std::io::{prelude::*, stdin};
fn main() {
    let org_arr = [1,2,3,5,6,8,10,11];
    let sub_arr = [6,8,10];

    let bai1 = checkArray(&org_arr, &sub_arr);
    let bai2 = searchSlice();
    
    println!("Bai1: {}", bai1);
    print!("Bai2: {}", bai2);

}
pub fn checkArray(org_arr:&[i32],sub_arr:&[i32]) ->bool {
    let mut a = 0;
    if sub_arr.len()==0 {
        return true;
    } 
    if sub_arr.len() > org_arr.len() {
        return false;
    }
    for  i in sub_arr.iter() {
        for j in org_arr.iter() {
            if i == j {
                a = a +1;
            }
        }
    }
    if a == sub_arr.len() {
        return true; 
    }
    return false; 
}

pub fn searchSlice() -> usize {
    let mut file = File::open("bai2.txt")
    .expect("File not found");

    let mut data = String::new();
    file.read_to_string(&mut data)
    .expect("Error while reading file");

    println!("Please input your keyword :");
    
    let mut keyword = String::new();
            stdin().read_line(&mut keyword).unwrap();

    let count = data.matches(&keyword.trim()).count();
    return count; 
}