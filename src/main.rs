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

    //filerino(&maps.get("Room1").unwrap());

    if anw == 1{//start game Map A

        intro2();// intro
        let mut ww = "1";
        map(ww);

        play();


    }else{
        return ();
    }









    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        //println!("{}: {}", key, value);
    }


}




fn play(){


    let mut filename = String::new();
    filename = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map1".to_string();

    let mut filer =String::new();
    filer = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\mapb".to_string();

    let mut filer_c = String::new();
    filer_c ="C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\mapc".to_string();


    let  mut maps = HashMap::new();
    maps.insert(String::from("Room1"),filename);
    maps.insert(String::from("RoomB"),filer);



    let mut anw = String::new();
    io::stdin().read_line(&mut anw)
        .expect("Failed to read line");

    let anw: u32 = anw.trim().parse()
        .expect("Please type a number!");

    match anw{//Map B
        1 => filerino(&maps.get("RoomB").unwrap()),
        2 => filerino(&maps.get("RoomC").unwrap()),

        _ =>  println!("Invalid "),


    }



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

    println!("\t    =======================================");
    println!("\t    =======================================");
    println!("\t    =================ACT 1=================");
    println!("\t    =======================================");
    println!("\t    =======================================");

    println!("╔=====================================================================================╗");
    println!("║In a dark cold room no lights or sounds completely alone...                          ║");
    println!("║...                                                                                  ║");
    println!("║Several minutes pass by, a rather strange light iluminates in front of you.          ║");
    println!("║It is a little hard to see, the light is too bright                                  ║");
    println!("║but you can get a glimpse of what is upon the table.                                 ║");
    println!("║...                                                                                  ║");
    println!("║Voice - '' Subject 703 please stand up.''                                            ║");
    println!("║Voice - '' ... ''                                                                    ║");
    println!("║Voice - '' Subject 703 please stand up.''                                            ║");
    println!("║Voice - '' In order to learn and fully understand human nature... ''                 ║");
    println!("║Voice - '' We need to test what you as a species call 'luck'. ''                     ║");
    println!("║Voice - '' Please grab the gun and play the game called Russian Roulett. ''          ║");
    println!("║Voice - '' Please don't waste our time, you know how to play it. ''                  ║");
    println!("╚=====================================================================================╝");

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