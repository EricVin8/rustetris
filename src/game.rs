use pancurses::{Input, Window};
use std::time::{Instant};
use std::char;
mod block;

pub fn genblockonscreen(vec: Vec<Vec<char>>, win: &Window, boardwidth: usize, boardheight: usize, index: usize) -> block::Blocks {
    let x = win.get_max_x() / 2;
    let y = win.get_max_y() / 2;

    let spawny = y - (boardheight as i32)/2 + 1;
    for i in 0.. vec.len() {
        for j in 0..vec.len() {
            if vec[i][j] == '#' {
                win.mvaddch(spawny + i as i32, j as i32 + x, '#');
            }
        }
    }
    win.refresh();
    block::Blocks{ leftx: x, lefty: spawny, minx: x - (boardwidth as i32/2) + 1, maxx: x + (boardwidth as i32/2), miny: spawny, maxy: y + (boardheight as i32)/2, index: index as i32 + 1, shape: vec,}
    //todo implement initial drawing on screen
    
}
pub fn drawborder(boardwidth: usize, boardheight: usize, win: &Window) {
    let x = win.get_max_x() / 2;
    let y = win.get_max_y() / 2;
    let minx = x - (boardwidth as i32/2) +1;
    let maxx = x + (boardwidth as i32/2);
    let miny = y - (boardheight as i32)/2 + 1;
    let maxy = y + (boardheight as i32)/2;
    for i in 0..boardwidth {
        win.mvaddch(miny - 1, minx + i as i32, '-');
        win.mvaddch(maxy + 1, minx + i as i32, '-');
    }
    for i in 0..boardheight {
        win.mvaddch(miny + i as i32, minx -1, '|');
        win.mvaddch(miny + i as i32, maxx+1, '|');
    }
    win.mvaddstr(2, win.get_max_x() - 12, "Score:");
    let tempscore = 0;
    updatescore(&win, &tempscore);
    win.refresh();
}
pub fn updatescore(window: &Window, score: &i32) {
    let stringscore = score.to_string();
    window.mvaddstr(2, window.get_max_x() - stringscore.len() as i32 - 1, stringscore);
    window.refresh();
}

pub fn clearline(block: block::Blocks, window : & Window, score: &mut i32) -> bool {
    //todo on block end, read screen into array, then run check on array
//if line is completed, then remove, add array of ' ' to top, then update every y value below/above the removed line 
let mut board = [[0u64; 10]; 20];
let mut consecutivecounter = 0;
let mut fulline = false;
let mut changed = false;
for i in block.miny..block.maxy+1 {
    fulline = true;
    for j in block.minx..block.maxx+1 {
        if (window.mvinch(i, j) & 0x7F) == 32 {
            fulline = false;
        }
        board[(i-block.miny) as usize][(j - block.minx) as usize] = window.mvinch(i, j) as u64;
    }
    if fulline {
        changed = true;
            //subtract original score, than add new one
        consecutivecounter += 1;
        for k in (1..(i - block.miny + 1)).rev() {
            board[k as usize] = board[k as usize-1];
        }
    }

}
match consecutivecounter {
    1 => {*score += 40;},
    2 => {*score += 100;},
    3 => {*score += 300;},
    4 => {*score += 1200;},
    _ => (),
}
if changed {
for i in block.miny..block.maxy + 1 {
    for j in block.minx..block.maxx+1 {
        window.mvaddch(i, j, board[(i-block.miny) as usize][(j - block.minx) as usize] as u32);
    }
}
}
window.refresh();
changed
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