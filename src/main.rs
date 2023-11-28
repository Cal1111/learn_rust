//use std::io;

fn main() {
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


}


//function tuples
// fn tuples() -> (i8,i8){
//     (5,7)
// }

//destruct the valeu in two variables

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
//     Left
// }

//tuples 
