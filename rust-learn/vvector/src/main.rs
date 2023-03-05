




fn main() {
    
    let v = vec![1,2,3,4,5,6];

    let third: &i32 = &v[2];
    println!("The third number id {third}");


    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third number id {third}"),
        None => println!("There is no third number"),
    }
}
