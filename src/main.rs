use std::fmt::Display;

macro_rules! add_as{
    (
  // repeated block
  $($a:expr)
 // seperator
   ,
// zero or more
   *
   )=>{
       { 
   // to handle the case without any arguments
   0
   // block to be repeated
   $(+$a)*
     }
    }
}
#[derive(Debug)]
struct Developer {
    pub age: i32,
}

macro_rules! debug_display {
    ($t:ident) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    }
}

debug_display!(Developer);
// add a macro that identfies a trait by returning a function pointer


fn main(){
    let dev : Developer = Developer{age: 12};
    println!("{}",add_as!(1,2,3,4)); // => println!("{}",{0+1+2+3+4})
    println!("{}", dev);
}
