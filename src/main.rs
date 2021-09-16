extern crate pancurses; 
extern crate rand;
use rand::Rng;

use pancurses::{initscr, endwin, noecho, curs_set};
mod game;
//let boardsize = 10;
/*fn printvector(vec: & Vec<Vec<char>>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() {
        print!("{}", vec[i][j]);
        }
        print!("\n");
       }
} */


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
         
        game::blockloop(&mut game::genblockonscreen(shapes[rand::thread_rng().gen_range(0..shapes.len())].clone(), &window), &window);
        //clear line here
    }
    
    endwin();

}

