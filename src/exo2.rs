pub fn function(){
    let my_nums= vec![10,20,30,40];
    for num in &my_nums{
        if num.eq(&30){
            println!("thirty");
        }else{
            println!("{num}");
        }
    }
    println!("vector lengh: {:?}",my_nums.len());

} 