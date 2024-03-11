
struct Emp{
    name:String
}
fn print_it(data:&str){
    println!("{data}");
}

pub fn function(){
    let owned = "owned string".to_owned();
    let other_owned = String::from("owned string");
    let bowrrowed = "borrowed string";
    print_it(&owned);
    print_it(&other_owned);
    print_it(bowrrowed);
    print_it(bowrrowed);


}