use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io;


struct User {
    username: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {

    intro();


    let mut anw = String::new();
    io::stdin().read_line(&mut anw)
        .expect("Failed to read line");

    let anw: u32 = anw.trim().parse()
        .expect("Please type a number!");



    if anw == 1{


        let mut ww = "1";
        map(ww);
    }




    let mut filename = String::new();
    filename = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map1".to_string();












    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    let sero : Option<&i32> = v.get(0);
    let third: &i32 = &v[0];

   // println!("{}",v[0]);
   // println!("{:?}",sero);
   // println!("{}",third);

    let mut s = String::new();
    s ="こんにちは".to_string();
    s.push_str("　アニメ");
    //println!("{}",s);



    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        //println!("{}: {}", key, value);
    }



    let  mut maps = HashMap::new();

    maps.insert(String::from("Room1"),filename);

    //println!("  - {:?}",maps.get("Room1"));


}



fn intro(){

    println!("     怖い     怖い   怖い   怖い  怖い  怖い  怖い  怖い   ");
    println!("     怖い     怖い   怖い   怖い  怖い  怖い  怖い  怖い   ");
    println!("     怖い     怖い   怖い   怖い  怖い  怖い  怖い  怖い   ");
    println!("                      Subject 704!                   ");
    println!("     怖い     怖い   怖い   怖い  怖い  怖い  怖い  怖い   ");
    println!("     怖い     怖い   怖い   怖い  怖い  怖い  怖い  怖い   ");
    println!("     怖い     怖い   怖い   怖い  怖い  怖い  怖い  怖い   ");
    println!("     怖い     怖い   怖い   怖い  怖い  怖い  怖い  怖い   ");
    println!("     逃げろ　 逃げろ　逃げろ　逃げろ　逃げろ　逃げろ　逃げろ  ");
    println!("=======================================================");
    println!("     七ゼロ四          1.NEW GAME          七ゼロ四      ");
    println!("     七ゼロ四          2.LOAD GAME         七ゼロ四      ");
    println!("     七ゼロ四          3.QUIT              七ゼロ四      ");
    println!("     七ゼロ四                              七ゼロ四             ");
    println!("     七ゼロ四                              七ゼロ四             ");
    println!("     七ゼロ四                              七ゼロ四             ");


}

fn intro2(){

    println!("");
}

fn map(number: &str){

    let mut filename = String::new();
    filename = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map1".to_string();
    match number{

        "1" => filerino(&filename),
        _ => println!("Invalid "),
    }

}


fn filerino(filename: &String){

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("  {}   ", contents);


}