mod game_manager;


use std::time::Duration;
use std::{env::args, thread::sleep};
use game_manager::{stop_process, run_process, clear_io};
use screenshots::Screen;

use rsautogui::{screen, mouse};

use crate::game_manager::{save_poses, swap_out_page, swap_in_page, avalible_pages_in_ram};

const RUN_COMMAND: &str = "run";
const VERIFY_COMMAND: &str = "verify";
const PRINT_CURSOR_COMMAND: &str = "print";

fn main() {
    let size = screen::size();

    let width = size.0 as f32;
    let height = size.1 as f32;

    let args = Vec::from_iter(args());

    let command = args.get(1).expect("No command given");

    if command == PRINT_CURSOR_COMMAND {
        print_cursor_percentage(size);
    }

    let num_cpus = args.get(2).expect("No CPU number passed in").parse::<u32>().expect("CPU count was not a number");
    let num_ram_rows = args.get(3).expect("No ram row number passed in").parse::<u32>().expect("ram row count was not a number");
    let screen: Screen = *Screen::all().unwrap().iter().find(|x| x.display_info.is_primary).unwrap();

    match command.as_str() {
        VERIFY_COMMAND => {
            let path = args.get(4).expect("No path given for verification image");
            save_poses(&screen, num_cpus, num_ram_rows, path);
        }
        RUN_COMMAND => {
            run_bot(&screen, num_cpus, num_ram_rows, width, height)
        }
        _ => {
            panic!("Command not recognized");
        }
    }
}

fn run_bot(screen: &Screen, num_cpus: u32, num_ram_rows: u32, width: f32, height: f32) {
    loop {
        let mut state = game_manager::get_state(screen, num_cpus, num_ram_rows);

        clear_io(width, height);

        for i in 0..state.cpus.len() {
            if let Some(process) = state.cpus[i] {
                if !process.state.should_be_on_cpu() {
                    stop_process(&process, width, height);
                    state.cpus[i] = None;
                }
            }
        }
        
        let num_open_cpus = state.cpus.iter().filter(|x| x.is_none()).count();
        state.idle.sort();
        for i in 0..num_open_cpus {
            if i < state.idle.len() {
                run_process(&state.idle[i], width, height);
            }
        }

        let mut i = 0;
        let mut avalible_ram_pages = avalible_pages_in_ram(num_ram_rows) - state.ram.len() as u32;
        'disk_loop: for needed_from_disk in state.disk.iter().filter(|x| x.needed) {
            if avalible_ram_pages > 0 {
                swap_in_page(needed_from_disk, num_ram_rows, width, height);
                avalible_ram_pages -= 1;
            }
            loop {
                if i >= state.ram.len() {
                    break 'disk_loop;
                }
                if !state.ram[i].needed {
                    swap_out_page(&state.ram[i], width, height);
                    i += 1;
                    break;
                }
                i += 1;
            }
            swap_in_page(needed_from_disk, num_ram_rows, width, height);
        }
    }    
}

fn print_cursor_percentage(size: (u16, u16)) {
    loop {
        sleep(Duration::from_millis(500));
        let pos = mouse::position();
        println!("{}, {}, {}, {}", pos.0, pos.1, pos.0 as f64 / size.0 as f64, pos.1 as f64 / size.1 as f64);
    }
}