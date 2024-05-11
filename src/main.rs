fn main(){
/* 
    let mut a: u8 = 250;
    let b: u8 = a+1;
    a -= 2;
    println!("Chislo: {a}{b}");
    let name: String = String::from("Denis");
    println!("{name}");
    let _name1: String = String::from("EMIR");
*/
/* 
    let symbol: char = 'S';
    let text: String = String::from("Some text...");
    let logic: bool = true;
    println!("{text}");
    println!("{symbol}");
    println!("{logic}");
*/

    let age: u16 = 101;
    if age >= 18{
        if age <= 100{
            println!("Можно");
        }else if age == 1488{
            println!("Пасхалко");
        }else{
            println!("ТЫ ХТО??");
        }
    }else{
        print!("Маленький ешо");
    }

    let is_true: bool = false;
    let _num: i8 = if is_true == true{
        1
    }else{
        6
    };
}