extern crate pancurses;
extern crate rand;
use rand::Rng;

use pancurses::{
    curs_set, endwin, init_color, init_pair, initscr, noecho, start_color, COLOR_BLACK, COLOR_BLUE,
    COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_PAIR, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
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
    let plusblock = vec![
        vec!['#', '#', '#'],
        vec![' ', '#', ' '],
        vec![' ', ' ', ' '],
    ];
    let square = vec![
        vec!['#', '#', ' '],
        vec!['#', '#', ' '],
        vec![' ', ' ', ' '],
    ];
    let rightl = vec![
        vec![' ', '#', '#'],
        vec![' ', '#', ' '],
        vec![' ', '#', ' '],
    ];
    let leftl = vec![
        vec!['#', '#', ' '],
        vec![' ', '#', ' '],
        vec![' ', '#', ' '],
    ];
    let rightslant = vec![
        vec![' ', ' ', '#'],
        vec![' ', '#', '#'],
        vec![' ', '#', ' '],
    ];
    let leftslant = vec![
        vec!['#', ' ', ' '],
        vec!['#', '#', ' '],
        vec![' ', '#', ' '],
    ];
    //let amogus = vec![vec!['#', '#', '#', ' '], vec!['#', ' ', '#', '#'], vec!['#', '#', '#', '#'], vec!['#', ' ', '#', ' ']];
    let longblock = vec![
        vec![' ', ' ', ' ', ' '],
        vec!['#', '#', '#', '#'],
        vec![' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' '],
    ];
    let shapes = [
        plusblock, square, rightl, leftl, rightslant, leftslant, longblock,
    ];

    let mut score = 0;
    let window = initscr();
    window.refresh();
    window.keypad(true);
    window.nodelay(true);
    curs_set(0);
    noecho();
    game::drawborder(BOARDWIDTH.clone(), BOARDHEIGHT.clone(), &window);
    start_color();
    init_color(8, 1000, 647, 0);
    init_color(COLOR_YELLOW, 1000, 1000, 0);
    init_color(COLOR_BLUE, 0, 0, 1000);
    init_color(COLOR_MAGENTA, 933, 510, 933);
    init_color(COLOR_RED, 1000, 0, 0);
    init_color(COLOR_GREEN, 0, 1000, 0);
    init_color(COLOR_CYAN, 0, 1000, 1000);
    init_pair(1, COLOR_MAGENTA, COLOR_MAGENTA);
    init_pair(2, COLOR_YELLOW, COLOR_YELLOW);
    init_pair(3, COLOR_BLUE, COLOR_BLUE);
    init_pair(4, 8, 8);
    init_pair(5, COLOR_RED, COLOR_RED);
    init_pair(6, COLOR_GREEN, COLOR_GREEN);
    init_pair(7, COLOR_CYAN, COLOR_CYAN);
    init_pair(8, COLOR_BLACK, COLOR_BLACK);
    loop {
        //note, colors will break if new block is added
        //note: switch to fastrng soon
        let randomblock = rand::thread_rng().gen_range(0..shapes.len());
        window.attron(COLOR_PAIR(randomblock as u32 + 1));
        let mut block = game::genblockonscreen(
            shapes[randomblock].clone(),
            &window,
            BOARDWIDTH.clone(),
            BOARDHEIGHT.clone(),
            randomblock,
        );
        game::blockloop(&mut block, &window);
        window.attroff(COLOR_PAIR(randomblock as u32 + 1));
        if game::clearline(block, &window, &mut score) {
            game::updatescore(&window, &score);
        }
        //clear line here
    }

    endwin();
}
