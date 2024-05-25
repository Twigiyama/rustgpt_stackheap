const MY_INTEGER: u8 = 10;

fn main() {
// Stack

let x: u8 = 70;
println!("x is {}", x);


//Heap

let mut arr: Vec<u8> = vec![1,2,3,4,5];
arr.push(10);
println!("vec is {:?}", arr);

//A Reference on the stack pointing to the heap.
let arr_2 = &arr[0..3];
println!["arr_2 is {:?}", arr_2];

//Heap
let mut s:String = String::from("Asitha Rodrigo");
s.push_str(" is a Software Engineer");
s.push('!');
println!("My name is {}", s);

//A Reference on the stack pointing to the heap.
let s2 = &s[0..5];
println!("s2 is {}", s2);
let s3: &str = "Banger";
println!("What is the {}", s3);

println!("MY_INTEGER is {:?}", MY_INTEGER);

}
