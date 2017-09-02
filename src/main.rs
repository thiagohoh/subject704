use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};
#[allow(dead_code)]
struct User {

    player_p: u32

}


fn main() {


    let mut player_points = 3;

    let mut player = User{

         player_p: player_points
};



    let mut file_map_1_intro = String::new();
    let mut file_map_2 = String::new();
    file_map_1_intro = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_1_intro_704".to_string();
    file_map_2 = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_2_704".to_string();

    intro();

    let mut anw = String::new();
    io::stdin().read_line(&mut anw)
        .expect("Failed to read line");

    let anw: u32 = anw.trim().parse()
        .expect("Please type a number!");

    //filerino(&maps.get("Room1").unwrap());

    if anw == 1{//start game Map A

        intro2();// intro
       // sleep(Duration::new(12, 0));
        let mut ww = "1";
        map(ww);//map1


        play_intro();// Map B C D
        //sleep(Duration::new(10, 0));


        filerino(&file_map_1_intro);//704 intro

        let mut anw = String::new();
        io::stdin().read_line(&mut anw)
        .expect("Failed to read line");

        let anw: u32 = anw.trim().parse()
        .expect("Please type a number!");




    }else{
        return ();
    }


    filerino(&file_map_2);//start act 2



    let  mut maps=  HashMap::new();

    maps.insert(String::from("Room1"),file_map_1_intro);




}




fn play_intro(){


    let mut filename = String::new();
    filename = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map1".to_string();

    let mut filer =String::new();
    filer = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\mapb".to_string();

    let mut filer_c = String::new();
    filer_c ="C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\mapc".to_string();

    let mut filer_d = String::new();
    filer_d ="C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\mapd".to_string();


    let  mut maps = HashMap::new();
    maps.insert(String::from("Room1"),filename);
    maps.insert(String::from("RoomB"),filer);
    maps.insert(String::from("RoomC"),filer_c);
    maps.insert(String::from("RoomD"),filer_d);



    let mut anw = String::new();
    io::stdin().read_line(&mut anw)
        .expect("Failed to read line");

    let anw: u32 = anw.trim().parse()
        .expect("Please type a number!");

    match anw{//Map B
        1 => filerino(&maps.get("RoomB").unwrap()),
        2 => filerino(&maps.get("RoomC").unwrap()),
        3 => filerino(&maps.get("RoomD").unwrap()),

        _ =>  println!("Invalid "),
    }

    if anw == 1{

        let mut anw = String::new();
        io::stdin().read_line(&mut anw)
            .expect("Failed to read line");

        let anw: u32 = anw.trim().parse()
            .expect("Please type a number!");


        match anw{//Map B

            1 => filerino(&maps.get("RoomC").unwrap()),
            2 => filerino(&maps.get("RoomD").unwrap()),

            _ =>  println!("Invalid "),
        }

    }else{
        return();
    }


}




fn play_act_2(playa :User){

    let  mut maps=  HashMap::new();
    let mut file_map_3 = String::new();
    let mut file_map_3_dead = String::new();

    file_map_3 = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_3_704".to_string();
    file_map_3_dead = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_3_playagain".to_string();

    maps.insert(String::from("Dead"),file_map_3);
    maps.insert(String::from("Map 3"),file_map_3_dead);

    let mut anw = String::new();
    io::stdin().read_line(&mut anw)
        .expect("Failed to read line");

    let anw: u32 = anw.trim().parse()
        .expect("Please type a number!");



    match anw{
        1 => {
            playa.player_p = 1;
            filerino(&file_map_3_dead);
        },

        2 =>{
            playa.player_p = 2;
            filerino(&file_map_3);
        }
        3 =>{
            playa.player_p = 3;
            filerino();
        }
        _=> return()
    }


}


fn map_3_questions(){



    match answ{



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
    println!("║Several minutes pass by, a rather strange light illuminates in front of you.         ║");
    println!("║It is a little hard to see, the light is too bright                                  ║");
    println!("║but you can get a glimpse of what is upon the table.                                 ║");
    println!("║...                                                                                  ║");
    println!("║Voice - '' Subject 703 please stand up.''                                            ║");
    println!("║Voice - '' ... ''                                                                    ║");
    println!("║Voice - '' Subject 703 please stand up.''                                            ║");
    println!("║Voice - '' In order to learn and fully understand human nature... ''                 ║");
    println!("║Voice - '' We need to test what you as a species call 'luck'. ''                     ║");
    println!("║Voice - '' Please grab the gun and play the game called Russian Roulette. ''         ║");
    println!("║Voice - '' Please don't waste our time, you know how to play it. ''                  ║");
    println!("╚=====================================================================================╝");

}


fn test(p_p: u32)->u32{

    println!("{}",p_p);
    p_p +1
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