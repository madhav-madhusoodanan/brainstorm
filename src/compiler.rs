use std::io::prelude::*;
use std::io;
use std::net::{TcpListener, TcpStream};

type Memtype = usize;
type ContentType = u8;
const MEMSIZE: usize = 30_000;
const ULIMIT:Memtype = 29_999;
const LLIMIT:Memtype = 0;
const UCONTENT:ContentType = 255;
const LCONTENT:ContentType = 0;

#[derive(Debug, PartialEq)]
pub enum CompileError {
    ReadError,
    OppositesNotAttracted(usize)    // thats when the loop brackets are not properly matched
    // ItsTooBig(usize),
    // ItsTooSmall(usize),
}

/* thats just an easter egg */
fn rick_roll_check(code: &String) {
    if code.contains("rick") {
        println!("Never gonna give you up, never gonna let you down <3")
    }
}


pub fn evaluate(code: &String) -> Result<Vec<ContentType>, CompileError> {
    let bytes: Vec<ContentType> = code.bytes().collect();
    let mut memory = [0u8; MEMSIZE];
    let mut address:usize = 0;
    let mut code_index = 0usize;
    let mut output: Vec<ContentType> = Vec::new();
    let mut loop_points:Vec<(usize, usize)> = Vec::new();

    let mut listener: Option<TcpListener> = None;
    let mut stream: Option<TcpStream> = None;

    /* checking for loop points, and verifying if the loop brackets are properly matched */
    {
        let mut loc:Vec<usize> = Vec::new();
        for (code_index, code) in bytes.iter().enumerate() {
            match code {
                b'[' => loc.push(code_index),
                    
                b']' => {
                        match loc.pop() {
                            Some(loc_start) => loop_points.push((loc_start, code_index)),
                            None => return Err(CompileError::OppositesNotAttracted(code_index))
                        };
                    }
                _ => continue
            }
        }
    }

    rick_roll_check(code);

    loop {
        
        match bytes.get(code_index) {
            Some(&char) => {
                match char {
                    b'+' => { 
                        // increment value at a memory location 
                        if memory[address] == UCONTENT {
                            memory[address] = LCONTENT;
                            code_index += 1;
                            // return Err(CompileError::ItsTooBig(code_index))
                        } else {
                            memory[address] += 1;
                            code_index += 1;
                        }
                    }
                    b'-' => { 
                        // decrement value at the memory location
                        if memory[address] == LCONTENT {
                            memory[address] = UCONTENT;
                            code_index += 1;
                            // return Err(CompileError::ItsTooSmall(address))
                        } else {
                            memory[address] -= 1;
                            code_index += 1;
                        }
                    }
                    b'>' => { 
                        // move right
                        if address + 1 == MEMSIZE {
                            address = 0;
                        } else {
                            address += 1
                        }
                        code_index += 1;
                    }
                    b'<' => {
                        // move left
                        if address == 0 {
                            address = MEMSIZE - 1;
                        } else {
                            address -= 1;
                        }
                        code_index += 1;
                    }
                    b'[' => {
                        // if this is the outermost loop
                        let index = loop_points.iter().position(|&(start, _)| start == code_index).unwrap();
                        if index == 0 {
                            // start the server
                            if listener.is_some() {
                                let t_stream = listener.expect("Active port was unbinded");
                                listener = t_stream.try_clone().ok();
                                let (tt_stream, _) = t_stream.accept().unwrap();
                                stream = Some(tt_stream)
                            }
                        }
                        // start the loop
                        if memory[address] > LCONTENT {
                            code_index += 1;
                        } else {
                            code_index = match loop_points.iter().find(|&&(start, _)| start == code_index){
                                Some(&(_, end)) => end,
                                None => panic!("")
                            };
                        }
                    }
                    b']' => {
                        // let index = loop_points.iter().position(|&(_, end)| end == code_index).unwrap();
                        // end of loop, go to its corresponding opening bracket
                        if memory[address] == LCONTENT {
                            code_index += 1;
                        } else {
                            code_index = match loop_points.iter().find(|&&(_, end)| end == code_index){
                                Some(&(start, _)) => start,
                                None => panic!("")
                            };
                        }
                    }
                    b',' => {
                        // getchar
                        if memory[ULIMIT] == 2 {
                            let mut buffer = [0u8];
                            match io::stdin().read(&mut buffer) {
                                Ok(_) => (),
                                Err(_) => return Err(CompileError::ReadError)
                            };
                            memory[address] = buffer[0];
                            code_index += 1;

                        } else if memory[ULIMIT] == 1 {
                            let mut buffer = [0u8; 1024];
                            let mut t_stream = stream.expect("No active connection");
                            stream = t_stream.try_clone().ok();
                            t_stream.read(&mut buffer).unwrap();

                            for index in 0..buffer.len() {
                                memory[index + LLIMIT] = buffer[index]
                            }

                        } else if memory[ULIMIT] == 0 {
                            listener = None;
                        }
                    }
                    b'.' => {
                        if memory[ULIMIT] == 2 {
                            // putchar
                            output.push(memory[address]);
                            code_index += 1;

                        } else if memory[ULIMIT] == 0 {
                            let mut sum: usize = 0;
                            for val in memory {
                                sum += usize::from(val);
                            }
                            listener = TcpListener::bind(format!("127.0.0.1:{}", sum)).ok();

                        } else if memory[ULIMIT] == 1 {
                            let mut t_stream = stream.expect("No active connection");
                            stream = t_stream.try_clone().ok();

                            t_stream.write(memory.as_slice()).unwrap();
                            t_stream.flush().unwrap();
                        }
                    }

                    _ => code_index += 1
                }
            }
            None => {break;}
        }
    }
    Ok(output)
}