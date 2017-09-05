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
    let mut file_map_4 = String::new();

    file_map_1_intro = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_1_intro_704".to_string();
    file_map_2 = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_2_704".to_string();
    file_map_4 = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_4".to_string();//map_4

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


    let  mut maps=  HashMap::new();

    maps.insert(String::from("Room1"),file_map_1_intro);
    maps.insert(String::from("Map 2"),file_map_2);
    maps.insert(String::from("Map 4"),file_map_4);


    filerino(&maps.get("Map 2").unwrap());//start act 2
    let mut anw = String::new();
    io::stdin().read_line(&mut anw)
        .expect("Failed to read line");

    let anw: u32 = anw.trim().parse()
        .expect("Please type a number!");

    play_act_2(player);


    match player.player_p{

        0 => return (),//edn game haha you suck

        2 =>{
            //door
        },

        4 =>{

            questions_3(); // ask some stupid questions

        }


    }



}





fn questions_3(){
// loop entre map 3 wtp way e wnt

    let mut file_map_3_wtp = String::new();
    let mut file_map_3_way = String::new();
    let mut file_map_3_wnt = String::new();
    let mut file_map_3_questions = String::new();

    file_map_3_wtp = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_3_wtp".to_string(); // map 3 wtp
    file_map_3_way = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_3_way".to_string(); // map 3 way
    file_map_3_wnt = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_3_wnt".to_string(); // map 3 wnt
    file_map_3_questions = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_3_questions".to_string(); // map 3 questions

    let  mut maps=  HashMap::new();

     maps.insert(String::from("WTP"),file_map_3_wtp);
     maps.insert(String::from("WAY"),file_map_3_way);
     maps.insert(String::from("WNT"),file_map_3_wnt);
     maps.insert(String::from("Questions"),file_map_3_questions);


    loop{

        let mut anw = String::new();
        io::stdin().read_line(&mut anw)
            .expect("Failed to read line");

        let anw: u32 = anw.trim().parse()
            .expect("Please type a number!");

        if anw == 1{
            filerino(&maps.get("WTP").unwrap());
            continue;
        }
        if anw == 2 {
            filerino(&maps.get("WAY").unwrap());
            continue;
        }
        if anw == 3 {
            filerino(&maps.get("WNT").unwrap());
            continue;

        }

        if anw == 4 {
            break;
        }

        else{
            filerino(&maps.get("Questions").unwrap());
            continue;

        }

    }


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
    let mut file_map_3_questions = String::new();


    file_map_3 = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_3_704".to_string();// map_3
    file_map_3_dead = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_3_playagain".to_string();// map_3_dead
    file_map_3_questions = "C:\\Users\\Thiago\\IdeaProjects\\subject704\\src\\map_3_questions".to_string();// map_3_questions

    maps.insert(String::from("Map 3"),file_map_3);
    maps.insert(String::from("Dead"),file_map_3_dead);
    maps.insert(String::from("Map 3 Questions"),file_map_3_questions);

    let mut anw = String::new();
    io::stdin().read_line(&mut anw)
        .expect("Failed to read line");

    let anw: u32 = anw.trim().parse()
        .expect("Please type a number!");

    filerino(&maps.get("Map 3").unwrap());// open map 3


    match anw{
        1 =>{
            filerino(&maps.get("Dead").unwrap());
            playa.player_p = 0;//dead

        },

        2 =>{

            playa.player_p = 2;//door
        },
        3 =>{
            filerino(&maps.get("Map 3 Questions").unwrap());
            playa.player_p = 4;//questions
        },

        _=> return(),


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