mod frames;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use frames::{FRAMES_BATH, FRAMES_DEAD, FRAMES_EAT, FRAMES_IDLE, FRAMES_PLAY, FRAMES_SLEEP};
use rand::Rng;
use std::{thread, time::Duration};

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_header() {
    println!("¡Bienvenido a Tamagotchi! 🐱");
    println!();
    println!("Presiona las siguientes teclas para interactuar:");
    println!("  - 'c' para darle de comer");
    println!("  - 'j' para jugar con él");
    println!("  - 'd' para dormir");
    println!("  - 'b' para bañarlo");
    println!("  - 'q' o 'Esc' para salir");
    println!("----------------------------------")
}

struct Tamagotchi {
    hunger: u8,
    boredom: u8,
    sleepiness: u8,
    hygiene: u8,
    state: State,
}

impl Tamagotchi {
    fn new() -> Self {
        Tamagotchi {
            hunger: 0,
            boredom: 0,
            sleepiness: 0,
            hygiene: 0,
            state: State::Idle,
        }
    }

    fn update_stats(&mut self) {
        if self.state == State::Dead {
            return;
        }

        if self.state != State::Eat {
            self.hunger = self.hunger.saturating_add(2).min(100);
        }
        if self.state != State::Play {
            self.boredom = self
                .boredom
                .saturating_add(rand::thread_rng().gen_range(0..3))
                .min(100);
        }
        if self.state != State::Sleep {
            self.sleepiness = self.sleepiness.saturating_add(1).min(100);
        }
        if self.state != State::Bath {
            self.hygiene = self.hygiene.saturating_add(1).min(100);
        }
    }

    fn check_stats(&mut self) {
        if self.hunger >= 100 {
            self.state = State::Dead;
            println!("¡Tu Tamagotchi ha muerto de hambre!");
        }

        if self.boredom >= 100 {
            self.state = State::Dead;
            println!("¡Tu Tamagotchi ha muerto de aburrimiento!");
        }

        if self.sleepiness >= 100 {
            self.state = State::Dead;
            println!("¡Tu Tamagotchi ha muerto de sueño!");
        }

        if self.hygiene >= 100 {
            self.state = State::Dead;
            println!("¡Tu Tamagotchi ha muerto por suciedad!");
        }
    }

    fn print_stats(&mut self) {
        println!(
            "Hambre:        [{}] {}%",
            self.create_progress_bar(self.hunger),
            self.hunger
        );
        println!(
            "Aburrimiento:  [{}] {}%",
            self.create_progress_bar(self.boredom),
            self.boredom
        );
        println!(
            "Sueño:         [{}] {}%",
            self.create_progress_bar(self.sleepiness),
            self.sleepiness
        );
        println!(
            "Higiene:       [{}] {}%",
            self.create_progress_bar(self.hygiene),
            self.hygiene
        );
        println!("----------------------------------");
        self.check_stats();
    }

    fn create_progress_bar(&self, value: u8) -> String {
        let filled_blocks = (value as f32 / 10.0).ceil() as usize;
        let empty_blocks = 10 - filled_blocks;
        let filled_bar = "█".repeat(filled_blocks);
        let empty_bar = "░".repeat(empty_blocks);
        format!("{}{}", filled_bar, empty_bar)
    }

    fn eat(&mut self) {
        self.hunger = self.hunger.saturating_sub(20);
    }

    fn play(&mut self) {
        self.boredom = self.boredom.saturating_sub(20);
    }

    fn sleep(&mut self) {
        self.sleepiness = self.sleepiness.saturating_sub(20);
    }

    fn bath(&mut self) {
        self.hygiene = self.hygiene.saturating_sub(20);
    }
}

#[derive(PartialEq)]
enum State {
    Idle,
    Eat,
    Play,
    Sleep,
    Bath,
    Dead,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut tamagotchi = Tamagotchi::new();

    enable_raw_mode()?;

    loop {
        tamagotchi.update_stats();
        match tamagotchi.state {
            State::Idle => {
                for frame in &FRAMES_IDLE {
                    clear_terminal();
                    print_header();
                    tamagotchi.print_stats();
                    println!("{}", frame);
                    thread::sleep(Duration::from_millis(200));
                }
            }
            State::Sleep => {
                tamagotchi.sleep();
                for frame in &FRAMES_SLEEP {
                    clear_terminal();
                    print_header();
                    tamagotchi.print_stats();
                    println!("A dormir!");
                    println!("{}", frame);
                    thread::sleep(Duration::from_millis(200));
                }
                tamagotchi.state = State::Idle;
            }
            State::Eat => {
                tamagotchi.eat();
                for frame in &FRAMES_EAT {
                    clear_terminal();
                    print_header();
                    tamagotchi.print_stats();
                    println!("A comer!");
                    println!("{}", frame);
                    thread::sleep(Duration::from_millis(200));
                }
                tamagotchi.state = State::Idle;
            }
            State::Play => {
                tamagotchi.play();
                for frame in &FRAMES_PLAY {
                    clear_terminal();
                    print_header();
                    tamagotchi.print_stats();
                    println!("A jugar!");
                    println!("{}", frame);
                    thread::sleep(Duration::from_millis(200));
                }
                tamagotchi.state = State::Idle;
            }
            State::Bath => {
                tamagotchi.bath();
                for frame in &FRAMES_BATH {
                    clear_terminal();
                    print_header();
                    tamagotchi.print_stats();
                    println!("A bañarse!");
                    println!("{}", frame);
                    thread::sleep(Duration::from_millis(200));
                }
                tamagotchi.state = State::Idle;
            }
            State::Dead => {
                for frame in &FRAMES_DEAD {
                    clear_terminal();
                    print_header();
                    tamagotchi.print_stats();
                    println!("{}", frame);
                    thread::sleep(Duration::from_millis(200));
                }
            }
        }

        if event::poll(std::time::Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('c') => {
                        tamagotchi.state = State::Eat;
                    }
                    KeyCode::Char('j') => {
                        tamagotchi.state = State::Play;
                    }
                    KeyCode::Char('d') => {
                        tamagotchi.state = State::Sleep;
                    }
                    KeyCode::Char('b') => {
                        tamagotchi.state = State::Bath;
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
