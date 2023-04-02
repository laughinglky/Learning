fn main() {
    
    let str = "hello rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p}", ptr);
    println!("{:?}", len);

    let arr1: [u32; 5] = [1,1,1,1,1];
    reset1(arr1);
    println!("{:?}", arr1);
    println!("==========");

    let mut arr2 = [2,2,2,2,2];
    reset2(&mut arr2);
    println!("{:?}", arr2);
    println!("==========");

}

fn reset1(mut arr: [u32; 5]) {
    arr[0] = 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;
    println!("{:?}", arr);
}

fn reset2(arr: &mut [u32]) {
    arr[0] = 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;
    println!("{:?}", arr);
}
