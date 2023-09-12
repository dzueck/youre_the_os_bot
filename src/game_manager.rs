use std::{time::Duration, thread::sleep};

use image::RgbaImage;
use rsautogui::{screen::Rgba, mouse::{self, Button}};
use screenshots::Screen;

#[derive(Debug, Clone)]
pub struct GameState {
    pub cpus: Vec<Option<CpuProcess>>,
    pub idle: Vec<IdleProcess>,
    pub ram: Vec<RamPage>,
    pub disk: Vec<DiskPage>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CpuProcess {
    //pid: u32,
    pub state:ProcessState,
    pos: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct IdleProcess {
    //pid: u32,
    pub state:ProcessState,
    pos: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum ProcessState {
    NeedRam,
    Dying,
    Crying,
    Angry,
    Annoyed,
    Satisfied,
    Happy,
    Finished,
    Waiting,

}

impl ProcessState {
    pub fn should_be_on_cpu(self) -> bool {
        !matches!(self, Self::Finished | Self::Happy | Self::Waiting)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RamPage {
    //pid: u32,
    pub needed: bool,
    pos: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct DiskPage {
    //pid: u32,
    pub needed: bool,
    pos: u32,
}

const CPU_START_X: f32 = 0.07128906;
const CPU_INCREMENT_X: f32 = (0.09277344 - 0.03857422) - 0.0002;
const CPU_Y: f32 = 0.1267361;

const PROCESS_START_X: f32 = CPU_START_X;
const PROCESS_INCREMENT_X: f32 = CPU_INCREMENT_X;
const PROCESS_ROW_WIDTH: u32 = 7;
const PROCESS_START_Y: f32 = 0.2734375;
const PROCESS_INCREMENT_Y: f32 = (0.3107638888888889 - 0.21527778) + 0.0003;
const PROCESS_COLUMN_HEIGHT: u32 = 6;

const RAM_START_X: f32 = 0.4736328;
const RAM_INCREMENT_X: f32 = 0.5053711 - RAM_START_X;
const RAM_ROW_WIDTH: u32 = 16;
const RAM_START_Y: f32 = 0.24131945;
const RAM_INCREMENT_Y: f32 = (0.29166666 - RAM_START_Y) + 0.0005;

const DISK_START_X: f32 = RAM_START_X;
const DISK_INCREMENT_X: f32 = RAM_INCREMENT_X;
const DISK_ROW_WIDTH: u32 = RAM_ROW_WIDTH;
const DISK_START_Y: f32 = 0.3567708333333333;
const DISK_INCREMENT_Y: f32 = RAM_INCREMENT_Y;
const DISK_COLUMN_HEIGHT_SOFT: u32 = 10;

const WAITING_COLOR: Rgba<u8> = Rgba([155, 155, 154, 255]);
const NEED_RAM_COLOR: Rgba<u8> = Rgba([0, 0, 255, 255]);
const FINISHED_COLOR: Rgba<u8> = Rgba([176, 216, 230, 255]);
const HAPPY_COLOR: Rgba<u8> = Rgba([0, 255, 0, 255]);
const SATISFIED_COLOR: Rgba<u8> = Rgba([255, 255, 0, 255]);
const ANNOYED_COLOR: Rgba<u8> = Rgba([255, 165, 0, 255]);
const ANGRY_COLOR: Rgba<u8> = Rgba([255, 0, 0, 255]);
const CRYING_COLOR: Rgba<u8> = Rgba([139, 0, 0, 255]);
const DYING_COLOR: Rgba<u8> = Rgba([80, 0, 0, 255]);

const PAGE_UNUSED_COLOR: Rgba<u8> = Rgba([99, 102, 106, 255]);
const PAGE_NEEDED_1_COLOR: Rgba<u8> = Rgba([255, 255, 255, 255]);
const PAGE_NEEDED_2_COLOR: Rgba<u8> = Rgba([0, 0, 255, 255]);

const PLUS_RADIUS: u32 = 6;
const PLUS_COLOR: Rgba<u8> = Rgba([50, 50, 255, 255]);

const IO_X: f32 = 0.0869140625;
const IO_Y: f32 = 0.036458333333333336;

pub fn save_poses(screen: &Screen, num_cpus: u32, num_ram_rows: u32, path: &str) {
    let mut poses = Vec::new();
    let mut screen = screen.capture().expect("Failed to get screen capture");
    let width = screen.width() as f32;
    let height = screen.height() as f32;
    
    // CPU poses
    poses.extend((0..num_cpus).map(|i| cpu_pos(i, width, height)));

    // Process poses
    poses.extend((0..(PROCESS_COLUMN_HEIGHT * PROCESS_ROW_WIDTH)).map(|i| idle_pos(i, width, height)));

    //ram
    poses.extend((0..(RAM_ROW_WIDTH * num_ram_rows)).map(|i| ram_pos(i, width, height)));

    //disk
    poses.extend((0..(DISK_ROW_WIDTH * DISK_COLUMN_HEIGHT_SOFT)).map(|i| disk_pos(i, num_ram_rows, width, height)));

    poses.push(io_pos(width, height));

    for pos in poses {
        add_plus(&mut screen, pos.0, pos.1);
    }
    screen.save(path).expect("Failed to save image");
}

fn add_plus(image: &mut RgbaImage, x: u32, y: u32) {
    for i in 0..PLUS_RADIUS {
        image.put_pixel(x + i, y, PLUS_COLOR);
        image.put_pixel(x - i, y, PLUS_COLOR);
        image.put_pixel(x, y + i, PLUS_COLOR);
        image.put_pixel(x, y - i, PLUS_COLOR);
    }
}

pub fn get_state(screen: &Screen, num_cpus: u32, num_ram_rows: u32) -> GameState {
    let screen = screen.capture().expect("Failed to get screen capture");
    let width = screen.width() as f32;
    let height = screen.height() as f32;

    let mut cpus = Vec::new();
    for i in 0..num_cpus {
        let pos = cpu_pos(i, width, height);
        let cpu_pixel = screen.get_pixel(pos.0, pos.1);

        let state = match *cpu_pixel {
            WAITING_COLOR => Some(ProcessState::Waiting),
            FINISHED_COLOR => Some(ProcessState::Finished),
            HAPPY_COLOR => Some(ProcessState::Happy),
            SATISFIED_COLOR => Some(ProcessState::Satisfied),
            ANNOYED_COLOR => Some(ProcessState::Annoyed),
            ANGRY_COLOR => Some(ProcessState::Angry),
            CRYING_COLOR => Some(ProcessState::Crying),
            DYING_COLOR => Some(ProcessState::Dying),
            NEED_RAM_COLOR => Some(ProcessState::NeedRam),
            _ => None
        };

        cpus.push(state.map(|state| CpuProcess {state, pos: i}));
    }

    let mut idle = Vec::new();
    for i in 0..(PROCESS_ROW_WIDTH * PROCESS_COLUMN_HEIGHT) {
        let pos = idle_pos(i, width, height);
        let idle_pixel = screen.get_pixel(pos.0, pos.1);

        let state = match *idle_pixel {
            WAITING_COLOR => Some(ProcessState::Waiting),
            FINISHED_COLOR => Some(ProcessState::Finished),
            HAPPY_COLOR => Some(ProcessState::Happy),
            SATISFIED_COLOR => Some(ProcessState::Satisfied),
            ANNOYED_COLOR => Some(ProcessState::Annoyed),
            ANGRY_COLOR => Some(ProcessState::Angry),
            CRYING_COLOR => Some(ProcessState::Crying),
            DYING_COLOR => Some(ProcessState::Dying),
            NEED_RAM_COLOR => Some(ProcessState::NeedRam),
            _ => None
        };

        if let Some(state) = state {
            idle.push(IdleProcess {state, pos: i});
        }
    }

    let mut ram = Vec::new();
    for i in 0..(num_ram_rows * RAM_ROW_WIDTH) {
        let pos = ram_pos(i, width, height);
        let ram_pixel = screen.get_pixel(pos.0, pos.1);
        
        let needed = match *ram_pixel {
            PAGE_UNUSED_COLOR => Some(false),
            PAGE_NEEDED_1_COLOR | PAGE_NEEDED_2_COLOR => Some(true),
            _ => None
        };

        if let Some(needed) = needed {
            ram.push(RamPage {needed, pos: i});
        }
    }

    let mut disk = Vec::new();
    for i in 0..(DISK_COLUMN_HEIGHT_SOFT * DISK_ROW_WIDTH) {
        let pos = disk_pos(i, num_ram_rows, width, height);
        let disk_pixel = screen.get_pixel(pos.0, pos.1);

        let needed = match *disk_pixel {
            PAGE_UNUSED_COLOR => Some(false),
            PAGE_NEEDED_1_COLOR | PAGE_NEEDED_2_COLOR => Some(true),
            _ => None
        };

        if let Some(needed) = needed {
            disk.push(DiskPage {needed, pos: i});
        }
    }

    GameState {cpus, idle, ram, disk}
}

pub fn stop_process(process: &CpuProcess, width: f32, height: f32) {
    click(cpu_pos(process.pos, width, height))
}

pub fn run_process(process: &IdleProcess, width: f32, height: f32) {
    click(idle_pos(process.pos, width, height))
}

pub fn swap_out_page(page: &RamPage, width: f32, height: f32) {
    click(ram_pos(page.pos, width, height))
}

pub fn swap_in_page(page: &DiskPage, num_ram_rows: u32, width: f32, height: f32) {
    click(disk_pos(page.pos,  num_ram_rows, width, height))
}

pub fn clear_io(width: f32, height: f32) {
    click(io_pos(width, height));
}

pub fn avalible_pages_in_ram(num_ram_rows: u32) -> u32 {
    num_ram_rows * RAM_ROW_WIDTH
}

fn cpu_pos(num: u32, width: f32, height: f32) -> (u32, u32) {
    let x = ((CPU_START_X + CPU_INCREMENT_X * num as f32) * width) as u32;
    (x, (CPU_Y * height) as u32)
}

fn idle_pos(num: u32, width: f32, height: f32) -> (u32, u32) {
    let column = num % PROCESS_ROW_WIDTH;
    let row = num / PROCESS_ROW_WIDTH;

    let x = ((PROCESS_START_X + PROCESS_INCREMENT_X * column as f32) * width) as u32;
    let y = ((PROCESS_START_Y + PROCESS_INCREMENT_Y * row as f32) * height) as u32;
    (x, y)
}

fn ram_pos(num: u32, width: f32, height: f32) -> (u32, u32) {
    let column = num % RAM_ROW_WIDTH;
    let row = num / RAM_ROW_WIDTH;

    let x = ((RAM_START_X + RAM_INCREMENT_X * column as f32) * width) as u32;
    let y = ((RAM_START_Y + RAM_INCREMENT_Y * row as f32) * height) as u32;
    (x, y)
}

fn disk_pos(num: u32, num_ram_rows: u32, width: f32, height: f32) -> (u32, u32) {
    let column = num % DISK_ROW_WIDTH;
    let row = num / DISK_ROW_WIDTH;
    let start_y = DISK_START_Y + RAM_INCREMENT_Y * (num_ram_rows - 1) as f32;

    let x = ((DISK_START_X + DISK_INCREMENT_X * column as f32) * width) as u32;
    let y = ((start_y + DISK_INCREMENT_Y * row as f32) * height) as u32;
    (x, y)
}

fn io_pos(width: f32, height: f32) -> (u32, u32) {
    ((width * IO_X) as u32, (height * IO_Y) as u32)
}

fn click(pos: (u32, u32)) {
    mouse::move_to(pos.0 as u16, pos.1 as u16);
    sleep(Duration::from_millis(5));
    mouse::click(Button::Left);
}