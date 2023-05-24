extern crate sdl2;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::env;
use std::time::Duration;

#[derive(Clone, Copy)]
struct Ant{
    cord: Rect,
    view: u8,
    color: Color,
}


fn is_in_list(squere: Vec<Rect>, mut cord: Rect) -> bool{
    for squ in squere{
        if squ.x == cord.x && squ.y == cord.y{
            return true;
            
        }
    }
    return false;
}


fn new_pos_for_red_ant(red_squere: Vec<Rect>, green_squere: Vec<Rect>, blue_squere: Vec<Rect>, ant: Ant, ant_jump: i32, screen_width: u32, screen_height: u32) -> Ant{
    
    let mut ant = Ant { cord:ant.cord, view: (ant.view), color: ant.color };
    if is_in_list(red_squere.clone(), ant.cord){
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        
    }else if is_in_list(green_squere.clone(), ant.cord){
        
        
    }else if is_in_list(blue_squere.clone(), ant.cord){
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        
    }else {
        
        ant.view += 1;
        
        if ant.view == 4{
            ant.view = 0;
        }
    }

    if ant.view == 0{
        ant.cord.x = ant.cord.x + ant_jump;
    }else if ant.view == 1 {
        ant.cord.y = ant.cord.y + ant_jump;
    }else if ant.view == 2{
        ant.cord.x = ant.cord.x - ant_jump;
    }else if ant.view == 3 {
        ant.cord.y = ant.cord.y - ant_jump;
    }

    if ant.cord.x < 0 {
        ant.cord.x = screen_width as i32 -1;
    }
    if ant.cord.y < 0 {
        ant.cord.y = screen_height as i32 -1;
    }

    ant.cord.x = ant.cord.x % screen_width as i32;
    ant.cord.y = ant.cord.y % screen_height as i32;

    ant
}


fn new_pos_for_green_ant(red_squere: Vec<Rect>, green_squere: Vec<Rect>, blue_squere: Vec<Rect>, ant: Ant, ant_jump: i32, screen_width: u32, screen_height: u32) -> Ant{
    
    let mut ant = Ant { cord:ant.cord, view: (ant.view), color: ant.color };
    if is_in_list(green_squere.clone(), ant.cord){
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        
    }else if is_in_list(red_squere.clone(), ant.cord){
        
        
    }else if is_in_list(blue_squere.clone(), ant.cord){
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        
    }else {
        
        ant.view += 1;
        
        if ant.view == 4{
            ant.view = 0;
        }
    }

    if ant.view == 0{
        ant.cord.x = ant.cord.x + ant_jump;
    }else if ant.view == 1 {
        ant.cord.y = ant.cord.y + ant_jump;
    }else if ant.view == 2{
        ant.cord.x = ant.cord.x - ant_jump;
    }else if ant.view == 3 {
        ant.cord.y = ant.cord.y - ant_jump;
    }

    if ant.cord.x < 0 {
        ant.cord.x = screen_width as i32 -1;
    }
    if ant.cord.y < 0 {
        ant.cord.y = screen_height as i32 -1;
    }

    ant.cord.x = ant.cord.x % screen_width as i32;
    ant.cord.y = ant.cord.y % screen_height as i32;

    ant
}


fn new_pos_for_blue_ant(red_squere: Vec<Rect>, green_squere: Vec<Rect>, blue_squere: Vec<Rect>, ant: Ant, ant_jump: i32, screen_width: u32, screen_height: u32) -> Ant{
    
    let mut ant = Ant { cord:ant.cord, view: (ant.view), color: ant.color };
    if is_in_list(blue_squere.clone(), ant.cord){
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        
    }else if is_in_list(red_squere.clone(), ant.cord){
        
        
    }else if is_in_list(green_squere.clone(), ant.cord){
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        
    }else {
        
        ant.view += 1;
        
        if ant.view == 4{
            ant.view = 0;
        }
    }

    if ant.view == 0{
        ant.cord.x = ant.cord.x + ant_jump;
    }else if ant.view == 1 {
        ant.cord.y = ant.cord.y + ant_jump;
    }else if ant.view == 2{
        ant.cord.x = ant.cord.x - ant_jump;
    }else if ant.view == 3 {
        ant.cord.y = ant.cord.y - ant_jump;
    }

    if ant.cord.x < 0 {
        ant.cord.x = screen_width as i32 -1;
    }
    if ant.cord.y < 0 {
        ant.cord.y = screen_height as i32 -1;
    }

    ant.cord.x = ant.cord.x % screen_width as i32;
    ant.cord.y = ant.cord.y % screen_height as i32;

    ant
}


fn new_pos(black_squere: Vec<Rect>, ant: Ant, ant_jump: i32, screen_width: u32, screen_height: u32) -> Ant{
    
    let mut ant = Ant { cord:ant.cord, view: (ant.view), color: ant.color };
    if is_in_list(black_squere.clone(), ant.cord){
        if ant.view == 0{
            ant.view = 3;
        }else {
            ant.view -= 1;
        }
        
    }else {
        
        ant.view += 1;
        
        if ant.view == 4{
            ant.view = 0;
        }
    }

    if ant.view == 0{
        ant.cord.x = ant.cord.x + ant_jump;
    }else if ant.view == 1 {
        ant.cord.y = ant.cord.y + ant_jump;
    }else if ant.view == 2{
        ant.cord.x = ant.cord.x - ant_jump;
    }else if ant.view == 3 {
        ant.cord.y = ant.cord.y - ant_jump;
    }

    if ant.cord.x < 0 {
        ant.cord.x = screen_width as i32 -1;
    }
    if ant.cord.y < 0 {
        ant.cord.y = screen_height as i32 -1;
    }

    ant.cord.x = ant.cord.x % screen_width as i32;
    ant.cord.y = ant.cord.y % screen_height as i32;

    ant
}

pub fn main() {
    let mut rng = rand::thread_rng();
    let mut i: u128 = 0;
    let mut screen_width: u32 = 400;
    let mut screen_height: u32 = 400;
    let mut ant_size: u32 = 1;
    let mut ant_count: usize = 1;
    let args: Vec<String> = env::args().collect();
    let mut ants:Vec<Ant> = Vec::new();
    let mut ant_colors:Vec<Color> = Vec::new();
    let mut ant_colors_active:bool = false;
    ant_colors.push(Color::RGB(255, 0, 0));
    ant_colors.push(Color::RGB(0, 255, 0));
    ant_colors.push(Color::RGB(0, 0, 255));


    for (index, arg) in args.iter().enumerate() {
        if arg == "-w" {
            let newarg;
            if let Some(next_arg) = args.get(index + 1) {
                newarg = next_arg.clone().parse();
                screen_width = match newarg {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Nie udało się przekonwertować na u32");
                        return;
                    }
                };
                
            }  
        }
        if arg == "-h" {
            let newarg;
            if let Some(next_arg) = args.get(index + 1) {
                newarg = next_arg.clone().parse();
                screen_height = match newarg {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Nie udało się przekonwertować na u32");
                        return;
                    }
                };
                
            }  
        }
        if arg == "-s" {
            let newarg;
            if let Some(next_arg) = args.get(index + 1) {
                newarg = next_arg.clone().parse();
                ant_size = match newarg {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Nie udało się przekonwertować na u32");
                        return;
                    }
                };
                
            }  
        }
        if arg == "-a" {
            let newarg;
            if let Some(next_arg) = args.get(index + 1) {
                newarg = next_arg.clone().parse();
                ant_count = match newarg {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Nie udało się przekonwertować na u32");
                        return;
                    }
                };
                
            }  
        }
        if arg == "-c" {
            ant_colors_active = true; 
        }
    }

    let ant_jump: i32 = ant_size as i32;
    let mut black_squere: Vec<Rect> = Vec::new();
    let mut red_squere: Vec<Rect> = Vec::new();
    let mut blue_squere: Vec<Rect> = Vec::new();
    let mut green_squere: Vec<Rect> = Vec::new();


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Ant", screen_width, screen_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    for i in 0..ant_count{
        let mut ant: Ant;
        if ant_colors_active{
            ant = Ant { cord: (Rect::new(rng.gen_range(0..screen_width) as i32, rng.gen_range(0..screen_height) as i32, ant_size, ant_size)), view: (2), color: (ant_colors[i % 3]) };
        }else {
            ant = Ant { cord: (Rect::new(rng.gen_range(0..screen_width) as i32, rng.gen_range(0..screen_height) as i32, ant_size, ant_size)), view: (2), color: (Color::RGB(255, 255, 0)) }; 
        }
        
        ants.push(ant);
    }
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();


    'running: loop {
        println!("{}", i);
        i += 1;
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        for rect in black_squere.clone(){
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.fill_rect(rect);
        }
        for rect in red_squere.clone(){
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.fill_rect(rect);
        }
        for rect in green_squere.clone(){
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.fill_rect(rect);
        }
        for rect in blue_squere.clone(){
            canvas.set_draw_color(Color::RGB(0, 0, 255));
            canvas.fill_rect(rect);
        }
        
        for i in 0..ant_count{
            canvas.set_draw_color(ants[i].color);
            canvas.fill_rect(ants[i].cord);
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.present();
            let old: Rect = ants[i].cord;
            if ants[i].color.r == 255 && ants[i].color.g == 0 && ants[i].color.b == 0{
                ants[i] = new_pos_for_red_ant(red_squere.clone(),green_squere.clone(),blue_squere.clone(), ants[i], ant_jump.clone(), screen_width, screen_width);
            }else if ants[i].color.r == 0 && ants[i].color.g == 255 && ants[i].color.b == 0{
                ants[i] = new_pos_for_green_ant(red_squere.clone(),green_squere.clone(),blue_squere.clone(), ants[i], ant_jump.clone(), screen_width, screen_width);
            }else if ants[i].color.r == 0 && ants[i].color.g == 0 && ants[i].color.b == 255{
                ants[i] = new_pos_for_blue_ant(red_squere.clone(),green_squere.clone(),blue_squere.clone(), ants[i], ant_jump.clone(), screen_width, screen_width);
            }else {
                ants[i] = new_pos(black_squere.clone(), ants[i], ant_jump.clone(), screen_width, screen_width);
            }
            if ants[i].color.r == 255 && ants[i].color.g == 0 && ants[i].color.b == 0{
                if is_in_list(black_squere.clone(), old){
                    black_squere.retain(|&x| x != old);
                }else if is_in_list(blue_squere.clone(), old){
                    blue_squere.retain(|&x| x != old);
                }else if is_in_list(green_squere.clone(), old){
                    green_squere.retain(|&x| x != old)
                }
                if is_in_list(red_squere.clone(), old){
                    red_squere.retain(|&x| x != old);
                }else{
                    red_squere.push(old);
                }
            }else if ants[i].color.r == 0 && ants[i].color.g == 255 && ants[i].color.b == 0{
                if is_in_list(black_squere.clone(), old){
                    black_squere.retain(|&x| x != old);
                }else if is_in_list(red_squere.clone(), old){
                    red_squere.retain(|&x| x != old);
                }else if is_in_list(blue_squere.clone(), old){
                    blue_squere.retain(|&x| x != old)
                }
                if is_in_list(green_squere.clone(), old){
                    green_squere.retain(|&x| x != old);
                }else{
                    green_squere.push(old);
                }
            }else if ants[i].color.r == 0 && ants[i].color.g == 0 && ants[i].color.b == 255{
                if is_in_list(black_squere.clone(), old){
                    black_squere.retain(|&x| x != old);
                }else if is_in_list(red_squere.clone(), old){
                    red_squere.retain(|&x| x != old);
                }else if is_in_list(green_squere.clone(), old){
                    green_squere.retain(|&x| x != old)
                }
                if is_in_list(blue_squere.clone(), old){
                    blue_squere.retain(|&x| x != old);
                }else{
                    blue_squere.push(old);
                }
            }else{
                if is_in_list(black_squere.clone(), old){
                    black_squere.retain(|&x| x != old);
                }else if is_in_list(red_squere.clone(), old){
                    red_squere.retain(|&x| x != old);
                }else if is_in_list(green_squere.clone(), old){
                    blue_squere.retain(|&x| x != old)
                }else if is_in_list(blue_squere.clone(), old){
                    blue_squere.retain(|&x| x != old)
                }else{
                    black_squere.push(old);
                }
            }
            
        }
        ::std::thread::sleep(Duration::new(0, 1u32 / 60));
    }
}