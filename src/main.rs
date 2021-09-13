extern crate pancurses; 
extern crate rand;
use rand::Rng;
use std::time::{Instant, Duration};

use pancurses::{initscr, endwin, noecho, Input, Window, curs_set};
struct Blocks {
    leftx: i32,
    lefty: i32,
    shape:  Vec<Vec<char>>,
}
impl Blocks {

    fn derotate(&mut self) {
        let mut temparr = vec![vec![' '; self.shape.len()]; self.shape.len()];
        for i in 0..self.shape.len() {
            for j in (0..self.shape.len()).rev() {
                temparr[self.shape.len()-(j+1)][i] = self.shape[i][j];
            }
        }
    }
    fn rotate(&mut self, win: &Window) {
        //problem, properly elim old shape if goign to rotate, also why is it sometimes spawning in new blocks? 
        let mut temparr = vec![vec![' '; self.shape.len()]; self.shape.len()];
        for i in 0..self.shape.len() {
            for j in 0..self.shape.len() {
                if self.shape[i][j] == '#' {
                    win.mvaddch(self.lefty + (i as i32), self.leftx + (j as i32), ' ');
                    //set character at location to ' '!, add oldx and oldy for offset!
                }
                
            }
        }
        for i in 0..self.shape.len() {
         for j in 0..self.shape.len() {
            temparr[self.shape.len()-(j+1)][i] = self.shape[i][j];
         }
        }
        for i in 0..self.shape.len() {
            for j in 0..self.shape.len() {
                self.shape[i][j] = temparr[i][j];
            }
        }
        if !self.check_futurecoords(self.leftx, self.lefty, win) {
            self.derotate();
            self.move_tocoords(self.leftx, self.lefty, win);

        }
        else {
            self.move_tocoords(self.leftx, self.lefty, win);
            win.refresh();
        }
     }
    fn move_tocoords(&mut self, newx: i32, newy: i32, win: &Window) {
        //remove old section from screen, dont forget to window.refresh() after this move!!!
        for i in 0..self.shape.len() {
            for j in 0..self.shape.len() {
                if self.shape[i][j] == '#' {
                    win.mvaddch(self.lefty + (i as i32), self.leftx + (j as i32), ' ');
                    //set character at location to ' '!, add oldx and oldy for offset!
                }
                
            }
        }
        for i in 0..self.shape.len() {
            for j in 0..self.shape.len() {
                if self.shape[i][j] == '#' {
                    win.mvaddch(newy + (i as i32), newx + (j as i32), '#');
                    
                }
                
            }
        }
        self.leftx = newx;
        self.lefty = newy;
        //print new section, then put newx and newy in the place of shape's x and y
    }
    fn check_futurecoords(&mut self, newx: i32, newy: i32, win: &Window) -> bool {
        let mut success = true;

        //todo use left coord as base, then run foor loop, and check at each position where char in base == '#' if there is one exisiting. On the last i where i == shapes.len(), check bottom for hashtag or ground, and if hashtag create new block, and if ground move then create new block, will return false if block is in solid state
        for i in 0..self.shape.len() {
            for j in 0.. self.shape.len() {
                //not sure why a -1 is needed here, but it needs it to work
                if self.shape[i][j] == '#' && newy + i as i32 > win.get_max_y() -1 {
                     success = false;
                }
                if self.shape[i][j] == '#' && win.mvinch(newy + i as i32, newx + j as i32 ) == 35 {
                    //problem found, checks collision of old and new of same shape, add new if checks for solution!
                    //The big checc here, checks if that piece found is a part of the current shape or an already placed block
                    //eprintln!("the values inputted are: j {} and i {} ", j  + (newx - self.leftx) as usize, i + (newy - self.lefty) as usize );
                
                    if !((j as i32 + (newx - self.leftx)) as i32 >= 0 && i as i32 + (newy - self.lefty) >= 0 && j as i32 + (newx - self.leftx) < self.shape.len() as i32 && i as i32 + (newy - self.lefty) < self.shape.len() as i32 && self.shape[(i as i32 + (newy - self.lefty)) as usize][(j as i32 + (newx - self.leftx)) as usize] == '#') {
                    success = false;
                    } 
                }
            }
        }
        
        success
    }
    
    fn move_left(&mut self, win: &Window) {
        if self.check_futurecoords(self.leftx - 1, self.lefty, win) {
            self.move_tocoords(self.leftx - 1, self.lefty, win);
            win.refresh();

        }
    }
    fn move_right(&mut self, win: &Window) {
        if self.check_futurecoords(self.leftx + 1, self.lefty, win) {
            self.move_tocoords(self.leftx + 1, self.lefty, win);
            win.refresh();

        }
    }
    fn move_down(&mut self, win: &Window) -> bool {
        if self.check_futurecoords(self.leftx, self.lefty+1, win) {
            self.move_tocoords(self.leftx, self.lefty+1, win);
            win.refresh();
            true
        }
        else {
        false
        }
    }

}


fn genblockonscreen(vec: Vec<Vec<char>>, win: &Window) -> Blocks {
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
    Blocks{ leftx: x, lefty: y, shape: vec,}
    //todo implement initial drawing on screen
    
}
fn printvector(vec: & Vec<Vec<char>>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() {
        print!("{}", vec[i][j]);
        }
        println!("");
       }
}
fn blockloop(block : &mut Blocks, window : & Window) {
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
    //here time will be checked and move commands/other commands will be issued. 

fn main() { 
    let plusblock = vec![vec!['#', '#', '#'], vec![' ', '#', ' '], vec![' ', ' ', ' ']];
    let square = vec![vec!['#', '#', ' '], vec!['#', '#', ' '], vec![' ', ' ', ' ']];
    let rightl = vec![vec![' ', '#', '#'], vec![' ', '#', ' '], vec![' ', '#', ' ']];
    let leftl = vec![vec!['#', '#', ' '], vec![' ', '#', ' '], vec![' ', '#', ' ']];
    let rightslant = vec![vec![' ', ' ', '#'], vec![' ', '#', '#'], vec![' ', '#', ' ']];
    let leftslant = vec![vec!['#', ' ', ' '], vec!['#', '#', ' '], vec![' ', '#', ' ']];
    let longblock = vec![vec![' ', ' ', ' ', ' '], vec!['#', '#', '#', '#'], vec![' ', ' ', ' ', ' '], vec![' ', ' ', ' ', ' ']];
    let shapes = [plusblock, square, rightl, leftl, rightslant, leftslant, longblock];

   

    let window = initscr();
    window.refresh();
    window.keypad(true);
    window.nodelay(true);
    curs_set(0);
    noecho();
    loop { 
        
       //note: switch to fastrng soon
        let mut block = genblockonscreen(shapes[rand::thread_rng().gen_range(0..shapes.len())].clone(), &window);
        blockloop(&mut block, &window);
    }
    
    endwin();

}

