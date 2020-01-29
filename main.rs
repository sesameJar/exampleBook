
#[derive(Debug)]
struct Person<'a> {
    name : &'a str,
    age:u8
}

fn main(){

    let name = "Mehrad";
    let age = 26;
    let mehrad = Person{name,age};
    println!("{:?}",mehrad);

}