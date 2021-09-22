extern crate pancurses; 
extern crate rand;
use rand::Rng;

use pancurses::{initscr, endwin, noecho, curs_set};
mod game;
static BOARDWIDTH: usize = 10;
static BOARDHEIGHT: usize = 20;
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
    //let amogus = vec![vec!['#', '#', '#', ' '], vec!['#', ' ', '#', '#'], vec!['#', '#', '#', '#'], vec!['#', ' ', '#', ' ']];
    let longblock = vec![vec![' ', ' ', ' ', ' '], vec!['#', '#', '#', '#'], vec![' ', ' ', ' ', ' '], vec![' ', ' ', ' ', ' ']];
    let shapes = [plusblock, square, rightl, leftl, rightslant, leftslant, longblock];

   

    let window = initscr();
    window.refresh();
    window.keypad(true);
    window.nodelay(true);
    curs_set(0);
    noecho();
    game::drawborder(BOARDWIDTH.clone(), BOARDHEIGHT.clone(), &window);
    loop { 
        
       //note: switch to fastrng soon
        let mut block = game::genblockonscreen(shapes[rand::thread_rng().gen_range(0..shapes.len())].clone(), &window, BOARDWIDTH.clone(), BOARDHEIGHT.clone()); 
        game::blockloop(&mut block, &window);
        game::clearline(block, &window);
        //clear line here
    }
    
    endwin();

}

