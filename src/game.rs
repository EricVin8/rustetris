use pancurses::{Input, Window};
use std::time::{Instant};
mod block;


pub fn genblockonscreen(vec: Vec<Vec<char>>, win: &Window) -> block::Blocks {
    let x = 10;
    let y = 10;
    
    for i in 0.. vec.len() {
        for j in 0..vec.len() {
            if vec[i][j] == '#' {
                win.mvaddch(y + i as i32, j as i32 + x, '#');
            }
        }
    }
    win.refresh();
    block::Blocks{ leftx: x, lefty: y, shape: vec,}
    //todo implement initial drawing on screen
    
}

pub fn blockloop(block : &mut block::Blocks, window : & Window) {
    //is this needed or should this be in main????
    //block should be generated here
    let mut onblock = true;
    let mut starttime = Instant::now();

    while onblock {
        let newtime = Instant::now();
        let j = newtime.duration_since(starttime);
        if j.as_millis() > 400 {
            onblock = block.move_down(window);
            starttime = Instant::now();
        }
        //in here, we will set onblock equal to the checkcollisionand move function, and check keyboard inputs
        match window.getch() {
            None => (),
            //Some(Input::Character('\x1B')) => break,
            
            Some(Input::KeyLeft) => {
                block.move_left(window);
            }
            Some(Input::KeyRight) => {
                block.move_right(window);
            }
            Some(Input::KeyDown) => {
                onblock = block.move_down(window);
                starttime = Instant::now();
            }
            Some(Input::KeyUp) => {
                block.rotate(window);
            }
            
            
            _ => (),
            
            
        }
        
    }
    }