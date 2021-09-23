use pancurses::{Window};

pub struct Blocks {
    pub leftx: i32,
    pub lefty: i32,
    pub minx: i32,
    pub maxx: i32,
    pub miny: i32,
    pub maxy: i32,
    pub shape:  Vec<Vec<char>>,
}
impl Blocks {

    pub fn rotate(&mut self, win: &Window) {
        //problem, properly elim old shape if goign to rotate, also why is it sometimes spawning in new blocks? 
        //maybe i should have it rotate the way tetris blocks normally rotate...
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
            temparr[j][self.shape.len()-(i+1)] = self.shape[i][j];
         }
        }
        let mut swapsuccess = true;
        for i in 0..self.shape.len() {
            for j in 0..self.shape.len() {
                if temparr[i][j] == '#' && (win.mvinch(self.lefty + i as i32, self.leftx + j as i32 ) & 0x7F) == 35 || self.lefty + (i as i32) > self.maxy || self.leftx + (j as i32) > self.maxx || self.leftx + (j as i32) < self.minx {
                    swapsuccess = false;
                }
            }
        }
        if swapsuccess {
        for i in 0..self.shape.len() {
            for j in 0..self.shape.len() {
                self.shape[i][j] = temparr[i][j];
            }
        }
    }
        self.move_tocoords(self.leftx, self.lefty, win);
    
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
                if self.shape[i][j] == '#' && (newy + i as i32 > self.maxy || newx + j as i32 > self.maxx || (newx + j as i32) < self.minx){
                     success = false;
                }
                if self.shape[i][j] == '#' && ((win.mvinch(newy + i as i32, newx + j as i32 )) & 0x7F) == 35 {
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
    
    pub fn move_left(&mut self, win: &Window) {
        if self.check_futurecoords(self.leftx - 1, self.lefty, win) {
            self.move_tocoords(self.leftx - 1, self.lefty, win);
            win.refresh();

        }
    }
    pub fn move_right(&mut self, win: &Window) {
        if self.check_futurecoords(self.leftx + 1, self.lefty, win) {
            self.move_tocoords(self.leftx + 1, self.lefty, win);
            win.refresh();

        }
    }
    pub fn move_down(&mut self, win: &Window) -> bool {
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
