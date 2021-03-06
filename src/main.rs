// Main file for sudoku solver
// serves as an entry point for the MVC framework
// (or at least my sloppy version of it)

// mod rusty_sudoku_model;

// for optimization only, not used in initial solution
extern crate bitflags;

// DEBUG ONLY
// function to print sudoku grids
// takes in immutable slice of the full on vector
fn pretty_print_grid(grid_numbers : &[u8]) {

   // constants, vector slicing, and setting up range bounds
   let sudoku_size : usize = 9;
   let mut curr_idx : usize = 1;
   let mut low_idx : usize;

   while curr_idx <= grid_numbers.len() {
        // printing the current line
        if curr_idx % sudoku_size == 0 {
            low_idx = curr_idx - sudoku_size;
            println!("{:?}", &grid_numbers[low_idx..curr_idx]);
            curr_idx = curr_idx + 1;
        }
        else {
            curr_idx = curr_idx + 1;
        }
   }
}

#[derive(Clone)]
struct GridUnit {
    solutions : Vec<u8>,
    is_final : bool
}

fn main() {

    // websudoku.com/images/example-steps.html
    // CURRENT ISSUE: trims options correctly, but doesn't affirm single remaining
    // options for solutions based on solution space. See step one on guided walkthrough
    // for exmaple of needed "affirmative" logic.

    let mut test_input : Vec<Vec<u8>> =
        vec![vec![0,0,0,0,0,0,6,8,0],
             vec![0,0,0,0,7,3,0,0,9],
             vec![3,0,9,0,0,0,0,4,5],
             vec![4,9,0,0,0,0,0,0,0],
             vec![8,0,3,0,5,0,9,0,2],
             vec![0,0,0,0,0,0,0,3,6],
             vec![9,6,0,0,0,0,3,0,8],
             vec![7,0,0,6,8,0,0,0,0],
             vec![0,2,8,0,0,0,0,0,0]];

    // =================
    // INITALIZATION
    // =================

    // initializing an empty 9x9 collection of grid units
    let mut solution_space = Vec::new();
    for x in 0..9 {
        let mut solution_space_sub = Vec::new();
        for y in 0..9 {
            let mut single_grid_obj = GridUnit{solutions : vec![], is_final : false };
            solution_space_sub.push(single_grid_obj);
        }
        solution_space.push(solution_space_sub);
    }

    // filling the 9x9 collection with information
    for x in 0..9 {
        for y in 0..9 {
            if test_input[x][y] != 0 {
                solution_space[x][y].solutions.push(test_input[x][y]);
                solution_space[x][y].is_final = true;
            } else {
                let mut filler : Vec<u8> = (1..=9).collect();
                solution_space[x][y].solutions.extend(filler);
            }
        }
    }

    // debug grid print loop
    // for x in 0..9 {
    //     for y in 0..9 {
    //         println!("{0:?} {1:?}", solution_space[x][y].is_final,
    //                                 solution_space[x][y].solutions);
    //     }
    // }

    // =================
    // SOLUTION PROCESS
    // =================

    // variable for the solution state of the puzzle,
    // not just the individual grid items
    let mut iter_num : u64 = 0;
    let mut solved : bool = false;

    // process for looping over and solving the puzzle
    while !solved {

        println!("iteration # : {:?}",iter_num);

        // POSIBILITY REDUCTION
        // for all the 'solved' grid items...
        for x in 0..9 {
            for y in 0..9 {
                if solution_space[x][y].is_final == true {

                    // get current finalized value
                    let mut curr_num : u8 = solution_space[x][y].solutions[0];

                    // remove from row
                    for row in 0..9 {
                        if row != x {
                            solution_space[row][y].solutions.retain(|&x| x != curr_num);
                        }
                    }

                    // remove from columns
                    for col in 0..9 {
                        if col != y {
                            solution_space[x][col].solutions.retain(|&x| x != curr_num);
                        }
                    }

                    // remove from box
                    // there's a more efficent way to do this, I'm just being lazy I guess
                    let mut box_row : u8 = (x as u8) / 3;
                    let mut box_col : u8 = (y as u8) / 3;
                    let mut box_num = (box_row * 3) + box_col;

                    for row in 0..9 {
                        for col in 0..9 {
                            let mut inner_box_row : u8 = (row as u8) / 3;
                            let mut inner_box_col : u8 = (col as u8) / 3;
                            let mut inner_box_num : u8 = (inner_box_row * 3) + inner_box_col;

                            if (row != x) && (col != y) {
                                if box_num == inner_box_num {
                                    solution_space[row][col].solutions.retain(|&x| x != curr_num);
                                }
                            }
                        }
                    }

                }
            }
        }

        // POSSIBILITY AFIRMATION
        // add "affirm row" here (i.e., if this row still needs #3 and it can only
        // possibly be written to one place, write that value to the grid unit and
        // reduce the possibilites vector)
        for row in 0..9 {
            for cell in 0..9 {

            }
        }


        // mark newly final solutions
        for x in 0..9 {
            for y in 0..9 {
                if solution_space[x][y].solutions.len() == 1 {
                    solution_space[x][y].is_final = true;
                }
            }
        }

        // have we solved the puzzle?
        // if any of the entries are longer than 1 element, we haven't, so keep solving.
        // otherwise the process is complete.
        solved = true;
        for x in 0..9 {
            for y in 0..9 {
                if solution_space[x][y].is_final == false {
                    solved = false;
                    break;
                }
            }
        }

        iter_num = iter_num + 1;

        // debug
        if iter_num > 10 {
            for x in 0..9 {
                println!("\nRow # {:?}", x);
                for y in 0..9 {
                    println!("{0:?} {1:?}", solution_space[x][y].is_final,
                                            solution_space[x][y].solutions);
                }
            }
            break;
        }

    }

    println!("The puzzle is solved! Final answer:");

    for row in 0..9 {
        let mut row_print = Vec::new();
        for col in 0..9 {
            row_print.push(solution_space[row][col].solutions[0]);
        }
        println!("{:?}", row_print);
    }
}
