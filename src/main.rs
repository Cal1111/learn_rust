//use std::io;
// use std::net::Shutdown;
use std::collections::HashMap;
// #[derive(Debug)]
// struct Content {
//     content: String,
// }

fn main() {
    let mut furni = HashMap::new();

    furni.insert( "Chairs",5);
    furni.insert("Beds",3);
    furni.insert("Tables", 2);
    furni.insert("Couches",0);

    let mut total_stock = 0;

    for (item, qty) in furni.iter(){
        total_stock = total_stock + qty;
        println!("item: {:?}, qty: {:?}",item,qty);
      let stock_count = if qty == &0 {
            "out of stock!".to_owned()
        } else {
            format!("{:?}", qty)
        };
        println!("item={:?}, stock={:?}", item, stock_count);
    } 
    println!("total stock={:?}", total_stock);
    // let mut lockers = HashMap::new();
    // lockers.insert(1, Content {content: "stuff".to_owned()});
    // lockers.insert(2, Content {content: "shirts".to_owned()});
    // lockers.insert(3, Content {content: "pants".to_owned()});

    // for (locker_numbers, content) in lockers.iter() {
    //     println!("number: {:?}, content: {:?}",locker_numbers,content);
    // }
    //let name: i8  = 20;
    //aspas duplas é uma string 
    //aspas simples é um char
    //um número inteiro é um inteiro
    //um número quebrado é um float
    //true ou false é um bool
    //declara a variável 

    //& nome da variável 
    //& tipo da variável
    //& valor que recebe

    //anotações
    //a variavel não pode ser reatribuiada
    //se atribuir "mut"
    //pode mutar a variavel
    
   // const VALOR1:i16 = 20; //declarar o tipo da constante

    //operadores aritméticos
    /*let a: i32 = 20;
    let b: i32 = 5;
    let soma: i32 = a+b;
    let subtracao: i32 = a-b;
    let divisão: i32 = a/b;    
    println!("Hello, {}!", soma);
    println!("{}", subtracao);
    println!("{}", divisão);
    */

    //tupla
    //let tupla:(&str, i16, f64) = ("Carlos",10, 1.25); //define o tipo da var, possui 3 ou 4 elementos
    //println!("{} - {} - {}", tupla.0,tupla.1,tupla.2);
 
    //array/vetor
    //let _array: [i32;5] = [0,1,2,3,4];
    //println!("{}", _array[2]);

    //if else
    /* let a :i32 = 10;
    let b :i32 = 5;
    if a == 10 && b == 5 {
        println!("{}-{}",a,b);
    } else {
        println!("{}-{}",a,b);
    }*/ 

   /* let _fruta: &str = "Banana";
    if _fruta == "Banana" || _fruta == "Morango" {
        println!("{}",_fruta);
    } else {
        println!("Está errado");
    }*/

    //if ! negação (NÃO)

    //for é quantidade
    //while é condição 
    //loop é infinito
/*for
     for _x in 0..5{
        println!("Textos {}",_x);
    } 
*/
/* while 
    let mut x:i32 = 0;
    while x < 5 
    { 
        println!("{}",x);
        x = x + 1;
    }
*/
/* loop
let mut _x: i32 = 20;
loop {
    println!("{}",_x);
    _x=_x+1;
}
*/

//Guessing Games - Book Rust
    // println!("Guess the number!");

    // println!("Please input your guess.");

    // let mut guess = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // println!("You guessed: {guess}");

    //struct
    /*let minha_casa = Casa {
        quartos: 4,
        cor: String::from("Azul"),
    };

    println!("Tenho {} quartos e é {}",minha_casa.quartos, minha_casa.cor);
    */

    //método
   /*  let minha_casa = Casa{
        largura: 6,
        comprimento: 35,
        
    };

    println!("metros quadrados {} ", minha_casa.area());
    */
    

    //match / "switch"
    // let fruta:&str = "banana"; 
    // match fruta {
    //     _goaba => println!("Sua fruta é uma goaba"),
    //     _manga => println!("Sua fruta é uma manga"),
    //     _banana => println!("Sua fruta é uma banana"),
    //     _ => println!("Não conheço essa fruta"), //caso não tenha a resposta
    // };

    //enum = é como uma variavel que só passa os que vocÊ determinou
  /*   let meu_pet: Animal = Animal::Cachorro; //dois pontos = operador de escopo, como entre no enum Animal e pegue o valor Cachorro
    match meu_pet{
        Animal::Cachorro => println!("é um cachorro"),
        Animal::Panda => println!("é uma panda"),
        Animal::Gato => print!("é um gato"),
        _ => print!("não existe"),
    }*/

    //vetor
    //let v = vec![23,24,25];
    //println!("{}",v[1]);//0...1...2
    /*let mut v: Vec<u32> = Vec::new(); //vetor vazio
    v.push(10);//insere dentro do vetor
    v.push(11);//insere dentro do vetor
    v.push(12);//insere dentro do vetor

    v.remove(2);//remove de dentro do vetor, de acordo com o indice
    println!("{}",v[1]);*/
    

    //exercises in rust
//exercise 1 function
    /*  fn first_name(){
        println!("carlos");
    }
    
    fn last_name(){
        println!("jose");
    }

first_name(); // função prototipada
last_name();
*/

//exercice 2 - basic math
//display the result of the sum of two numbers;

//res();

//function arithmetic
// let sum = 2 + 2;
// let value = 10 - 5;
// let divison = 10 / 2;
// let mult = 5 * 5;

// let rem = 6 % 3;
// let rem2 = 6 % 4;

//control flow using if, else if & else

/*let age: i8 = 18;
//using if & else
if age == 18 {
    println!("Ele tem {} anos",age);
} else {
    println!("Ele não tem {}",age);
}
//using if , else if & else
if age != 18 {
            println!("Ele tem {} anos",age);
        } else if age <= 18 {
            println!("Ele tem menos de {} anos",age);
        } else if age > 10 {
            println!("Ele tem mais de 10 anos");
        } else {
            println!("Ele não tem a idade necessária");
}
*/
//exercise using if (FLOW)
// let quiz: bool = true;

// if quiz != true {
//     println!("hello");
// } else {
//     println!("goodbye");
// }

//exercise using if..else if..else (FLOW)

// let a_number:i8 = 5;

// if a_number > 5 {
//     println!(">5");
//         } else if a_number < 5 {
//             println!("<5");
//         } else if a_number == 5 {
//             println!("=5");
// } else {
//     println!("does not exist");
// }


//exercise using match (expressions)
// let ydk = false;
// match ydk {
//     true => println!("It's true"),
//     false => println!("It's false"),
//     _ => println!("Not exist")
// }

//exercise 2 using match
// let number: i8 = 1;
// match number {
//     1 => println!("one"),
//     2 => println!("two"),
//     3 => println!("three"),
//     _ => println!("not exist")
// }

//loop
// let mut i:i8=1;
// loop {
//     println!("Hello, World! {}",i);
//     i = i + 1;
//     if i == 5 {
//         break;
//     }
// }
// println!("Done!");

//while
//exerscise using while loop
// let mut i:i8 = 5;
// while i >= 0 {
//     println!("{}",i);
//     i = i - 1;
//}
//println!("Done");

// EXAMPLE FOR CONTROLL IN GAMING.
// fn which_way(go: Direction){
//     match go  {
//         Direction::Up => "up",
//         Direction::Down => "down",
//         Direction::Right => "right",
//         Direction::Left => "left",
//     };
// }
// let ac = Colors::Black;
// match ac
// {
//     Colors::Black => println!("Black"),
//     Colors::Blue => println!("Blue"),
//     Colors::Green => println!("Green"),
//     _ => println!("Not exist"),
// };
// choose_color(Colors::Black);

//struct 
// let cereal = GroceryItem {
//     stock: 10,
//     price: 5.5,
// };

// println!("Stock: {}. Price is {}",
// cereal.stock,
// cereal.price);

/* 
let can1 = Flavors::Guava{
    liquid: 2,
    price: 6,
};
let can2 = Flavors::Guava{
    liquid: 2,
    price: 6,
};*/

// let go: Flavors;
// match go {
//     Flavors::Sprite => println!("{}, {}", can.liquid,can.price),
//     Flavors::Orange => println!("{}, {}",can.liquid,can.price),
//     Flavors::Guava => println!("{}, {}",can.liquid,can.price),
// }
//exercise using struct, functuion,enum & match

// flavors_can(Flavors::Orange);
// flavors_can(Flavors::Sprite);
// flavors_can(Flavors::Guava);

//tuples 
//mode 1
// let coord:(i8,i8) = (2,3);
// println!("{},{}",coord.0,coord.1);
// //mode 2
// let (x,y) = (2,3);
// println!("{},{}",x,y);
// //mode 3
// let (name, last, age) = ("Carlos","José",10);
// println!("{} {} {}", name,last,age);

// let (x,y) = tuples();
//     if y > 5{
//         println!("Greater than 5!");
//     } else if y < 5 {
//         println!("Less than 5!");
//     } else {
//         println!("Equal the 5!");
//     }

//expression
//secret_file:admin=only;
// let acess_level = Acess::Guest;
// let can_acess_file = match acess_level{
//     Acess::Admin => true,
//         _ => false,
// };
// println!("{}",can_acess_file);


//use match expression to determine which message
//to print
// show();display_page_count(book);

//expression - exercise 
// fn show(){
// let my_bool = 100;
// let _value = if my_bool > 100 {
//     true
// } else {
//     false 
//};
// // ownership
// let book = Book {
//     pages: 30,
//     rating: 4,
// };
// display_page_count(&book);
// display_rating(&book);
// }

// struct Book {
//     pages:i16,
//     rating:i16
// }

// fn display_page_count(book: &Book){
//     println!("{}",book.pages);
// }

// fn display_rating(book: &Book){
//     println!("{}",book.rating);
// }


//ownership - exercise
//var show_items
// let show_items = GroseryItem {
//     quantity: 10,
//     id:42369,
//  };

//  display_id(&show_items);
//  display_quantity(&show_items);

//impl - exercise
// let t_box = Charact::value_box(BoxColor::Blue);
// t_box.print_box();

//vector-exercise 
// let my_scores = vec![
//     Test { score: 20},
//     Test { score: 40 },
// ];

// for test in my_scores{
//     println!("Value {}",test.score);
// }

// example - vector 
// let my_numbers = vec![1,2,3];

// let mut my_numbers = Vec::new();
// my_numbers.push(1);
// my_numbers.push(2);
// my_numbers.push(3);
// my_numbers.pop();
// my_numbers.len(); //this is 2

//vector - exercise
//vector to store 4 numbers
// let my_numbers = vec![10, 20, 30, 40];
// for num in &my_numbers {
//     match num {
//         30 => println!("thirty"),
//         _ => println!("{:?}", num),
//     }
// }
// println!("number of elements = {:?}", my_numbers.len());

// data type: string
//example - string

// print_it("message");
// let message = String::from("another message");
// let owned_string = "owned string".to_owned();
// print_it(&message);
// print_it(&owned_string);


//when i use struct, i need put String
/*
Struct Message {
    name: String
} */
//vector with 3 index
// let _vec = vec![
//     Person {
//         name: "Carlos".to_owned(),
//         age: 10,
//         favorite_color: "Blue".to_owned(),
//     },
//     Person {
//         name: "Jao".to_owned(),
//         age: 80,
//         favorite_color: "Pink".to_owned(),
//     },
//     Person {
//         name: "Ying".to_owned(),
//         age: 2,
//         favorite_color: "Black".to_owned(),
//     },
    
// ];

// for people in _vec {
//     // match people.age {
//     //   10 => 
//     // }
//     if people.age <= 10{
//         print_it(&people.favorite_color);
//         print_it(&people.name);
// }
// fn print_it(data:&str){
//     println!("{}",data);
//         }
//     }

//demo - derive
// let me = Employee{
//     position: Position::Worker, //enum
//     work_hours: 40,
// };
//     println!("{:?}",me);
//     print_employee(me);
//     print_employee(me);
// }
// #[derive(Debug, Clone, Copy)]
// //demo - derive 
// enum Position {
//     Manager,
//     Supervisor,
//     Worker,
// }
// #[derive(Debug,Clone,Copy)] //derive
// struct Employee{
//     position:Position,
//     work_hours: i64,
// }

// fn print_employee(emp:Employee){ //ownership
//     println!("{:?}",emp);
// }

//type annotations - e.g.
// let number: Vec<i32> = vec![1,2,3];
// let letters: Vec<char> = vec!['a','b'];
//let number: Vec<Mouse> = vec![
//    Mouse::LeftClick,
//    Mouse::RightClick,
//]
//enum - revisited
// enum Mouse{
//     LeftClick,
//     RightClick,
//     MiddleClick,
//     Scroll(i32),
//     Move(i32,i32),  //(x,y) position of mouse
// }

// enum PromoDiscount{
//     NewUser,
//     Holiday(String),
// }

// enum Discount{
//     Percent(f64),
//     Flat(i32),
//     Promo(PromoDiscount),
//     Custom(String),
// }

// let n = 3;
// match n{
//     3 => println!("three"),
//     other => println!("{:?}",other),
// };

// let flat = Discount::Flat(2);

// match flat {
//     Discount::Flat(2) => println!("flat é 2"),
//     Discount::Flat(amount) => println!("{}",amount),
//     _ => (),
// }
// let concert = Ticket {
//     event: "concert".to_owned(),
//     price: 50,
// };

// match concert {
//     Ticket {price:50, event} => println!("event @ 50 {:?} ",event),
//     Ticket {price, ..} => println!("event @ 50 {:?} ",price), 
//     //when we had one value and second value is ..
//     // .. it's ignore, when use Ticket {price:50, event}
//     //the value price it's defined && event will be receive your value 
// }
// let tickets = vec![
//         Ticket::Backstage(50.0, "Billy".to_owned()),
//         Ticket::Standard(15.0),
//         Ticket::Vip(30.0, "Amy".to_owned()),
//     ];

//     for ticket in tickets {
//         match ticket {
//             Ticket::Backstage(price, holder) => {
//                 println!("Backstate ticket holder: {:?}, price: {:?}", holder, price);
//             }
//             Ticket::Standard(price) => println!("Standard ticket price: {:?}", price),
//             Ticket::Vip(price, holder) => {
//                 println!("VIP ticket holder: {:?}, price: {:?}", holder, price);
//             }
//         }
//     }

//e.g option
// let mark = Customer {
//     age: Some(22), email: "mark@example.com".to_owned(),
// };

// let notch = Customer {
//     age: None, email: "notch@example.com".to_owned(),
// };

// match notch.age {
//     Some(age) => println!("customer is {:?} years old",age),
//     None => println!("Customer not provided"),
// }
//option - exercise
// let stu_1 = Student {
//     name: ("Cris".to_owned()),
//     locker: Some(123),
// };
// println!("my name is {:?}",stu_1.name);
// match stu_1.locker {
//     Some(ans) => println!("my locker is {:?}",ans),
//     None => println!("don't have locker"),
// }
// use /// for documentation 

//Stanrd library
// let numbers = vec![1,2,3];
// match numbers.is_empty(){
//     true => println!("no numbers"),
//     false => println!("has numbers"),
// }
//SLF - exercise
// let s = "HELLO";
// print!("{}",s.to_lowercase());
// print!("{}",s.to_uppercase());

//result - demo
// let choice: Result<MenuChoice, _> = get_choice("mainmenu");
// println!("choice = {:?}",choice); 
// pick_choice("start");
// let choice = pick_choice("end");
// println!("choice = {:?}",choice);

//result - exercise
// let child = Adult::new(15,"Anita");
// let adult = Adult::new(25,"Bob");

// match child {
//     Ok(child) => println!("{} is {} years",child.name,child.age),
//     Err(e) => println!("{e}",),
// }

// match adult {
//     Ok(a) => println!("{} is {} years",a.name,a.age),
//     Err(e) => println!("{e}",),
// }

//res? - exercise
// Anita is trying to access the Warehouse, which requires access level 500.
// Her keycard has access level 1000, which should be allowed.
// let anita_authorized = authorize("Anita", ProtectedLocation::Warehouse);
// // Brody is trying to access the Office, which requires access level 800.
// // His keycard has access level 500, which should be denied.
// let brody_authorized = authorize("Brody", ProtectedLocation::Office);
// // Catherine is trying to access the Warehouse, which requires access level 500.
// // She doesn't have a keycard, so this should be an error.
// let catherine_authorized = authorize("Catherine", ProtectedLocation::Warehouse);
// println!("{anita_authorized:?}");
// println!("{brody_authorized:?}");
// println!("{catherine_authorized:?}");

 }

//res? - exercise 
//  #[derive(Clone, Copy, Debug)]
//  enum ProtectedLocation {
//      All,
//      Office,
//      Warehouse,
//  }
 
//  impl ProtectedLocation {
//      fn required_access_level(&self) -> u16 {
//          match self {
//              Self::All => 1000,
//              Self::Office => 800,
//              Self::Warehouse => 500,
//          }
//      }
//  }
 
//  #[derive(Debug)]
//  struct Database;
 
//  impl Database {
//      fn connect() -> Result<Self, String> {
//          // In a production application, a database connection error is likely to occur here.
//          Ok(Database)
//      }
 
//      fn find_employee(&self, name: &str) -> Result<Employee, String> {
//          match name {
//              "Anita" => Ok(Employee {
//                  name: "Anita".to_string(),
//              }),
//              "Brody" => Ok(Employee {
//                  name: "Brody".to_string(),
//              }),
//              "Catherine" => Ok(Employee {
//                  name: "Catherine".to_string(),
//              }),
//              _ => Err(String::from("employee not found")),
//          }
//      }
 
//      fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
//          match employee.name.as_str() {
//              "Anita" => Ok(KeyCard { access_level: 1000 }),
//              "Brody" => Ok(KeyCard { access_level: 500 }),
//              other => Err(format!("{other} doesn't have a keycard")),
//          }
//      }
//  }
 
//  #[derive(Clone, Debug)]
//  struct Employee {
//      name: String,
//  }
 
//  #[derive(Debug)]
//  struct KeyCard {
//      access_level: u16,
//  }
 
//  #[derive(Clone, Copy, Debug)]
//  enum AuthorizationStatus {
//      Allow,
//      Deny,
//  }
//  fn authorize(
//     employee_name: &str,
//     location: ProtectedLocation,
// ) -> Result<AuthorizationStatus, String> {
//     let db = Database::connect()?;
//     let employee = db.find_employee(employee_name)?;
//     let keycard = db.get_keycard(&employee)?;

//     if keycard.access_level >= location.required_access_level() {
//         Ok(AuthorizationStatus::Allow)
//     } else {
//         Ok(AuthorizationStatus::Deny)
//     }
// }
// //result - exercise
// #[derive(Debug)]
// struct Adult{
//     age: u8,
//     name: String,
// }

// impl Adult {
//     fn new(age:u8, name: &str) -> Result<Self, &str> {
//         if age >= 21 {
//             Ok(Self 
//                 { 
//                     age, 
//                     name: name.to_string(), 
//             }) 
//         } else {
//             Err("Age must be a least 21")
//         }
//     }
// }
 //result - demo
// #[derive(Debug)]
// //structure
// enum MenuChoice {
//     MainMenu,
//     Start,
//     Quit,
// }
// //print choice
// fn print_choice(choice:&MenuChoice){
//     println!("choice = {:?}",choice);
// }
// //pick choice
// fn pick_choice(input:&str) -> Result<(), String> {
//     let choice:MenuChoice = get_choice(input)?;
//     print_choice(&choice);
//     Ok(())
// }
// //get_choice and match input
// fn get_choice(input:&str) -> Result<MenuChoice, String> {
//  match input {
//     "mainmenu" => Ok(MenuChoice::MainMenu),
//     "start" => Ok(MenuChoice::Start),
//     "quit" => Ok(MenuChoice::Quit),
//     _ => Err("menu not found".to_owned()),  
//  }   
// }
//  //option - exercise
//  struct Student {
//     name: String,
//     locker: Option<i32>
//  }
//  struct Customer{
//     age: Option<i32>,
//     email: String,
//  }

// //definition option
//  enum option<T>{
//      Some(T),
//      None,
//  }

//advanced match - exercise
// enum Ticket {
//     Backstage(f64, String),
//     Standard(f64),
//     Vip(f64, String),
// }

// //advanced match 
// enum Discount {
//     Percent(i32),
//     Flat(i32),
// }

// struct Ticket{
//     event: String,
//     price: i32,
// }
// struct Person {
//     name: String,
//     age: i32,
//     favorite_color: String,
// }
// fn print_it(data:&str){
//     println!("{}",data);
// }
//demo - vector
// struct Test {
//     score: i32
// }
//struct - characteristc
// struct Charact{
//     dimension: i32,
//     weight:f64,
// }
// //enum - box color
// enum BoxColor{
//     Black,
//     Blue,
// }
// impl Charact {
//     fn value_box(color: BoxColor) -> Self{
//     match color {
//         BoxColor::Black => println!("its black"),
//         BoxColor::Blue => println!("its blue"),
//     }    
//         Self {dimension: 20, weight: 25.5}
//     }
//     //print the box
//     fn print_box (&self) -> () {
//         println!("Dimension is {}, Weight is {}",&self.dimension,&self.weight);
//     }
// }
//  //impl - demo
// let hot = Temperature { degress_c: 34.5 };
// hot.show_temp();

// let cold = Temperature::freezing();
// cold.show_temp();
// struct Temperature{
//     degress_c: f64,
// }
// impl Temperature{
// fn freezing () -> Self { //Self its like "myself" ->>Temperature
//     Self {degress_c:35.0}
// }
// fn show_temp(&self){
//     println!("Display temperature: {}",self.degress_c);
//     }
// }

//  struct GroseryItem{
//     quantity:i32,
//     id:i32,
// }
// fn display_quantity(show_items: &GroseryItem){
//         println!("The quantity is {}",show_items.quantity);
// }
// fn display_id(show_items: &GroseryItem){
//     println!("The id number is {}", show_items.id);
// }
//  // FLOW -> _showres = res = _value = my_bool;
// let res = _value;
// let _showres = match res{
    //     true => println!("its big"),
    //     false => println!("its small"),
    // };   
// }
//expression
// let my_num = 3;
// let is_lt_5 = if my_num < 5 {
//     true
// } else {
//     false
// };
// let is_lt_5 = my_num < 5;
// enum Acess{
//     Guest,
//     Admin,
//     Manager,
//     User,
// } 

//function tuples
// fn tuples() -> (i8,i8){
//     (5,7)
// }

//using if...else...if...else for determining to what print
// fn flavors_can(my_can:Flavors){
//     let spr = Store {
//         liquid: 1,
//         price: 2,
//     };
//     let ora = Store {
//         liquid: 2,
//         price: 6,
//     };
//     let gua = Store {
//         liquid: 4,
//         price: 10,
//     };
//     match my_can
//     {
//     Flavors::Sprite => println!("Sprite {}l, {}", spr.liquid,spr.price),
//     Flavors::Orange => println!("Orange {}l, {}",ora.liquid,ora.price),
//     Flavors::Guava => println!("Guava {}l, {}",gua.liquid,gua.price),
//     }
// }
// //enum exer-struct
// enum Flavors { //it's like a library or bookshelf
//     Sprite,
//     Orange,
//     Guava,
// }
// //struct exerc-struct
// struct Store { //it's like a defined, e.g.: Store have products.
//     liquid: i16,
//     price: i16,
// }
// //struct
// struct GroceryItem {
//     stock:i8,
//     price: f32,
// } 

 //exercise about enum
// enum Colors{
//     Black,
//     Blue,
//     Green,
// }

// fn choose_color(my_color:Colors){
//     match my_color{
//         Colors::Black => println!("Black!"),
//         Colors::Blue => println!("Blue"),
//         Colors::Green => println!("Green"),
//     }
// }
//exercise 2 - Conclued
// fn sum(a:i8,b:i8) -> i8{ //display attributes 
//     a + b
// }
// fn res() {
//     println!("{}",sum(5, 5)); //display result -> 5 + 5 = 10;
// }
//function subtraction
// fn sub(a:i32, b:i32) -> i32 {
//     a - b
// }
//let five = sub(8,3);
//método 
// impl Casa {
//     fn area(&self) -> u32 {
//         self.largura * self.comprimento
//     }
// }

// //struct
// struct Casa{ //nome tem q começar com a letra Maiuscula. e.g.: Car, Banana...
//     largura: u32,
//     comprimento: u32,
// }
//enum
/*enum Animal{
    Cachorro,
    Gato,
    Panda,
}
*/

// //enum
// enum Direction {
//     Up,
//     Down,
//     Right,
//     Left,
// }