use dialoguer::Input;
use std::time::Instant;
use std::fs::File;
use std::io::{BufWriter, Write, Result};

const size: usize = 6;
const size_dec: usize = 5;

fn main() -> Result<()> {
    /*
    let size = Input::<usize>::new()
    .with_prompt("Matrix Size?")
    .interact_text()
    .unwrap();
    */

    let output_file_name: String = Input::new()
    .with_prompt("File to Write Matrices To?")
    .interact_text()
    .unwrap();   

    let output_file = File::create(output_file_name)?;

    let mut outbuf = BufWriter::new(output_file);

    mosaic_gen(& mut outbuf)?;
//  let mut our_mosaic = Mosaic::new(10, output_file).0;
//  our_mosaic.output_buffer.flush();
    Ok(())
}


//Memory safety is hard :(
/*

fn initialize_mosaic<'a>(size:usize, file: File ) -> (Mosaic<'a>, Result<()>) {
    let mut mosaic = Mosaic {
        size_dec: size-1,
        vector_length: size^2 - 1,
        mosaic: vec![0; size^2],
        curr_tile: 0,
        rightward: true,
        digit_index: vec![0; size^2],
        valid_tiles_for: Vec::with_capacity(size^2),
        output_buffer: BufWriter::new(file) 
    };
    (mosaic, Ok(()))
}

*/



fn mosaic_gen(output_buffer: &mut BufWriter<File> ) -> Result<()> {
    let vector_length = size*size - 1;
    let mut mosaic: Vec<usize> = vec![0; vector_length + 1];
    let mut curr_tile: usize = 0;
    let mut rightward = true;
    let mut digit_index: Vec<usize> = vec![0; vector_length + 1];
    let mut valid_tiles_for = Vec::with_capacity(vector_length + 1);
    unsafe {
            valid_tiles_for.set_len(vector_length + 1);
    }
    //Lookup table
    let connection_table: Vec<Vec<Vec<Vec<usize>>>> = vec![
        vec![vec![vec![0,2]]], //Top left corner -- conection_table[0][top][left]
        vec![ //Top row
            vec![vec![0,2], vec![0,2], vec![1,5], vec![1,5], vec![0,2], vec![1,5], vec![0,2], vec![1,5], vec![1,5], vec![1,5]]
        ],
        vec![ //Top right corner
            vec![vec![0], vec![0], vec![1], vec![1], vec![0], vec![1], vec![0], vec![1], vec![1], vec![1]]
        ],
        vec![ //Left column
            vec![vec![0,2]],
            vec![vec![3,6]],
            vec![vec![3,6]],
            vec![vec![0,2]],
            vec![vec![0,2]],
            vec![vec![0,2]],
            vec![vec![3,6]],
            vec![vec![3,6]],
            vec![vec![3,6]],
            vec![vec![3,6]],
        ],
        vec![ //Middle of mosaic
            vec![vec![0,2], vec![0,2], vec![1,5], vec![1,5], vec![0,2], vec![1,5], vec![0,2], vec![1,5], vec![1,5], vec![1,5]],
            vec![vec![3,6], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![4,7,8,9]], 
            vec![vec![3,6], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![4,7,8,9]], 
            vec![vec![0,2], vec![0,2], vec![1,5], vec![1,5], vec![0,2], vec![1,5], vec![0,2], vec![1,5], vec![1,5], vec![1,5]],
            vec![vec![0,2], vec![0,2], vec![1,5], vec![1,5], vec![0,2], vec![1,5], vec![0,2], vec![1,5], vec![1,5], vec![1,5]],
            vec![vec![0,2], vec![0,2], vec![1,5], vec![1,5], vec![0,2], vec![1,5], vec![0,2], vec![1,5], vec![1,5], vec![1,5]], 
            vec![vec![3,6], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![4,7,8,9]], 
            vec![vec![3,6], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![4,7,8,9]], 
            vec![vec![3,6], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![4,7,8,9]], 
            vec![vec![3,6], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![3,6], vec![4,7,8,9], vec![4,7,8,9], vec![4,7,8,9]]
        ],
        vec![ //Right Column
            vec![vec![0], vec![0], vec![1], vec![1], vec![0], vec![1], vec![0], vec![1], vec![1], vec![1]],
            vec![vec![6], vec![6], vec![4], vec![4], vec![6], vec![4], vec![6], vec![4], vec![4], vec![4]], 
            vec![vec![6], vec![6], vec![4], vec![4], vec![6], vec![4], vec![6], vec![4], vec![4], vec![4]], 
            vec![vec![0], vec![0], vec![1], vec![1], vec![0], vec![1], vec![0], vec![1], vec![1], vec![1]],
            vec![vec![0], vec![0], vec![1], vec![1], vec![0], vec![1], vec![0], vec![1], vec![1], vec![1]],
            vec![vec![0], vec![0], vec![1], vec![1], vec![0], vec![1], vec![0], vec![1], vec![1], vec![1]],
            vec![vec![6], vec![6], vec![4], vec![4], vec![6], vec![4], vec![6], vec![4], vec![4], vec![4]], 
            vec![vec![6], vec![6], vec![4], vec![4], vec![6], vec![4], vec![6], vec![4], vec![4], vec![4]], 
            vec![vec![6], vec![6], vec![4], vec![4], vec![6], vec![4], vec![6], vec![4], vec![4], vec![4]], 
            vec![vec![6], vec![6], vec![4], vec![4], vec![6], vec![4], vec![6], vec![4], vec![4], vec![4]], 
        ],

        vec![ //Bottom left corner
            vec![vec![0]],
            vec![vec![3]],
            vec![vec![3]],
            vec![vec![0]],
            vec![vec![0]],
            vec![vec![0]],
            vec![vec![3]],
            vec![vec![3]],
            vec![vec![3]],
            vec![vec![3]],
        ],
        
        vec![ //Bottom row
            vec![vec![0], vec![0], vec![5], vec![5], vec![0], vec![5], vec![0], vec![5], vec![5], vec![5]],
            vec![vec![3], vec![3], vec![4], vec![4], vec![3], vec![4], vec![3], vec![4], vec![4], vec![4]],
            vec![vec![3], vec![3], vec![4], vec![4], vec![3], vec![4], vec![3], vec![4], vec![4], vec![4]],
            vec![vec![0], vec![0], vec![5], vec![5], vec![0], vec![5], vec![0], vec![5], vec![5], vec![5]],
            vec![vec![0], vec![0], vec![5], vec![5], vec![0], vec![5], vec![0], vec![5], vec![5], vec![5]],
            vec![vec![0], vec![0], vec![5], vec![5], vec![0], vec![5], vec![0], vec![5], vec![5], vec![5]],
            vec![vec![3], vec![3], vec![4], vec![4], vec![3], vec![4], vec![3], vec![4], vec![4], vec![4]],
            vec![vec![3], vec![3], vec![4], vec![4], vec![3], vec![4], vec![3], vec![4], vec![4], vec![4]],
            vec![vec![3], vec![3], vec![4], vec![4], vec![3], vec![4], vec![3], vec![4], vec![4], vec![4]],
            vec![vec![3], vec![3], vec![4], vec![4], vec![3], vec![4], vec![3], vec![4], vec![4], vec![4]],
        ],

        vec![ //Bottom right corner
            vec![vec![0], vec![0], vec![], vec![], vec![0], vec![], vec![0], vec![], vec![], vec![]],
            vec![vec![], vec![], vec![4], vec![4], vec![], vec![4], vec![], vec![4], vec![4], vec![4]],
            vec![vec![], vec![], vec![4], vec![4], vec![], vec![4], vec![], vec![4], vec![4], vec![4]],
            vec![vec![0], vec![0], vec![], vec![], vec![0], vec![], vec![0], vec![], vec![], vec![]],
            vec![vec![0], vec![0], vec![], vec![], vec![0], vec![], vec![0], vec![], vec![], vec![]],
            vec![vec![0], vec![0], vec![], vec![], vec![0], vec![], vec![0], vec![], vec![], vec![]],
            vec![vec![], vec![], vec![4], vec![4], vec![], vec![4], vec![], vec![4], vec![4], vec![4]],
            vec![vec![], vec![], vec![4], vec![4], vec![], vec![4], vec![], vec![4], vec![4], vec![4]],
            vec![vec![], vec![], vec![4], vec![4], vec![], vec![4], vec![], vec![4], vec![4], vec![4]],
            vec![vec![], vec![], vec![4], vec![4], vec![], vec![4], vec![], vec![4], vec![4], vec![4]],
        ],
    ];

    loop {
        //print!("tile: {}", curr_tile);
        if rightward {
            match curr_tile / size {
                    0=> {           //Top row
                        match curr_tile % size {
                            0=>valid_tiles_for[curr_tile] = &connection_table[0][0][0], 
                            size_dec=>valid_tiles_for[curr_tile] = &connection_table[2][0][mosaic[curr_tile-1]],
                            _=>valid_tiles_for[curr_tile] = &connection_table[1][0][mosaic[curr_tile-1]]
                        }
                    },
                    size_dec => {  //Bottom row
                        match curr_tile % size {
                            0=>valid_tiles_for[curr_tile] = &connection_table[6][mosaic[curr_tile-size]][0], 
                            size_dec=>valid_tiles_for[curr_tile] = &connection_table[8][mosaic[curr_tile-size]][mosaic[curr_tile-1]],
                            _=>{
                                valid_tiles_for[curr_tile] = &connection_table[7][mosaic[curr_tile-size]][mosaic[curr_tile-1]];
                            }
                        }
                    },
                    _=> {
                        match curr_tile % size {
                            0=>valid_tiles_for[curr_tile] = &connection_table[3][mosaic[curr_tile-size]][0],
                            size_dec=>valid_tiles_for[curr_tile] = &connection_table[5][mosaic[curr_tile-size]][mosaic[curr_tile-1]],
                            _=>valid_tiles_for[curr_tile] = &connection_table[4][mosaic[curr_tile-size]][mosaic[curr_tile-1]]
                        }
                    }
                }
           if valid_tiles_for[curr_tile].len() == 0 {
                rightward = false;
                curr_tile -= 1;
                continue;
            }
            digit_index[curr_tile] = 1;
            mosaic[curr_tile] = valid_tiles_for[curr_tile][0];
            if curr_tile == vector_length {
                rightward = false;
                continue;
            }
            curr_tile += 1;
            continue;
        }

        if curr_tile == vector_length {
            writeln!(output_buffer, "{:?}", mosaic)?;
        }

        //Carrying tiles left
        if digit_index[curr_tile] == valid_tiles_for[curr_tile].len() {
            if curr_tile == 0 {
                break;
            }
            curr_tile -= 1;
            continue;
        }

        //Move to next tile in list for current tile
        mosaic[curr_tile] = valid_tiles_for[curr_tile][digit_index[curr_tile]];
        digit_index[curr_tile] += 1;
        if curr_tile < vector_length {
            curr_tile += 1;
            rightward = true;
        }

    }
    
    Ok(())
}
/* 
fn get_valid_tiles(mosaic: &Vec<usize>, curr_tile: usize, size: usize, ize_dec: usize) -> &Vec<usize> {
    match curr_tile / size {
        0=> {           //Top row
            match curr_tile % size {
                0=>connection_table[0][0], 
                size_dec=>right_connection_table[0][mosaic[curr_tile-1]],
                _=>connection_table[0][mosaic[curr_tile-1]]
            }
        }
        size_dec => {  //Bottom row
            match curr_tile % size {
                0=>connection_table[mosaic[curr_tile-size]][0], 
                size_dec=>right_connection_table[mosaic[curr_tile-size]][mosaic[curr_tile-1]],
                _=>connection_table[0][mosaic[curr_tile-1]]
            }
        },
        _=> {
            match curr_tile % size {
                0=>bottom_connection_table[curr_tile-size][0],
                size_dec=>final_connection_table[0][matrix[curr_tile-1]],
                _=>bottom_connection_table[mosaic[curr_tile-size]][mosaic[curr_tile-1]]
            }
        }
    }
}
*/

/*
match curr_tile / size {
                0=> {           //Top row
                    match curr_tile % size {
                        0=> valid_tiles_for[curr_tile] = connection_table[0,2],
                        size_dec=>match mosaic[curr_tile-1] {
                            0|1=>valid_tiles_for[curr_tile] = connection_table[0,2],
                            _=>valid_tiles_for[curr_tile] = connection_table[1,5] 
                        },
                        _=>match mosaic[curr_tile-1] {
                            0|1=>valid_tiles_for[curr_tile] = [0],
                            _=>valid_tiles_for[curr_tile] = connection_table[1]
                        }
                    }
                }
                size_dec => {  //Bottom row
                    match curr_tile % size {
                        0=> match mosaic[curr_tile - size] {
                                2|6=> valid_tiles_for[curr_tile] = connection_table[3],
                                _=> valid_tiles_for[curr_tile] = 
                            }
                        size_dec=>match mosaic[curr_tile-1] {
                            0|1=>match mosaic[curr_tile-size] {
                                valid_tiles_for[curr_tile] = connection_table[1,5] 
                            }

                            _=>
                        },
                        _=>match mosaic[curr_tile-1] {
                            0|1=>valid_tiles_for[curr_tile] = [0],
                            _=>valid_tiles_for[curr_tile] = connection_table[1]
                        }
                    }
                },
                _=> {
                    match curr_tile % size {
                        0=> valid_tiles_for[curr_tile] = connection_table[0,2],
                        size_dec=>match mosaic[curr_tile-1] {
                            0|1=>valid_tiles_for[curr_tile] = connection_table[0,2],
                            _=>valid_tiles_for[curr_tile] = connection_table[1,5] 
                        },
                        _=>match mosaic[curr_tile-1] {
                            0|1=>valid_tiles_for[curr_tile] = [0],
                            _=>valid_tiles_for[curr_tile] = connection_table[1]
                        }
                    }
                }
            }
            */

