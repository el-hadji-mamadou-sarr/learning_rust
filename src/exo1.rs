struct Box {
    dim: (i32, i32),
    weight: f64,
    color: Color,
}

impl Box{
   fn create()->Self{
        Self{
            dim:(500,200),
            weight:15.5,
            color:Color::Green
        }
   }

   fn show_box(&self){
        println!("dim:({:?},{:?})",self.dim.0,self.dim.1);
        println!("weigh:{:?}",self.weight);
        match self.color{
            Color::Green=>{
                println!("color: green");
            },
            Color::Yellow=>{
                println!("color:yellow");
            },
            Color::Red=>{
                println!("color:red");
            },
        }
   }

}
#[derive(Debug)]
enum Color {
    Green,
    Yellow,
    Red,
}

pub fn function(){
    let my_box = Box{
        dim:(400,200),
        weight:12.65,
        color:Color::Red
    };

    my_box.show_box();
    let new_box = Box::create();
    new_box.show_box();

    println!("{:?}", my_box.color);
}


