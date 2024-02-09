#![allow(
    clippy::uninlined_format_args,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

use colored::Colorize;
use rand::Rng;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::process::Output;

fn run_basic_dice_simulation(
    attacker_dice_count: usize,
    defender_dice_count: usize,
) -> std::io::Result<()> {
    let simulations = 10_000_000;
    let total_games = simulations * defender_dice_count;

    let attacker_wins: usize = (0..simulations)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let mut attacker_dice: Vec<_> = (0..attacker_dice_count)
                .map(|_| rng.gen_range(1..=6))
                .collect();
            let mut defender_dice: Vec<_> = (0..defender_dice_count)
                .map(|_| rng.gen_range(1..=6))
                .collect();

            attacker_dice.sort_unstable_by(|a, b| b.cmp(a));
            defender_dice.sort_unstable_by(|a, b| b.cmp(a));

            attacker_dice
                .iter()
                .zip(defender_dice.iter())
                .filter(|&(a, d)| a > d)
                .count()
        })
        .sum();

    let win_percentage = (attacker_wins as f64 / total_games as f64) * 100.0;
    let win_percentage_str = format!("{:.2}%", win_percentage);

    let colored_win_percentage = match win_percentage as usize {
        0..=49 => win_percentage_str.red().to_string(),
        50..=59 => win_percentage_str.yellow().to_string(),
        60..=100 => win_percentage_str.green().to_string(),
        _ => win_percentage_str.normal().to_string(),
    };

    println!(
        "Attacker with {} dice vs Defender with {} dice: {}",
        attacker_dice_count.to_string().cyan(),
        defender_dice_count.to_string().cyan(),
        colored_win_percentage
    );

    // Open the file in append mode (creates it if it doesn't exist)
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("data/simulation_data.csv")?;

    // Write the data to the file
    writeln!(
        file,
        "{},{},{}",
        attacker_dice_count, defender_dice_count, win_percentage
    )?;

    Ok(())
}

fn run_battle_simulation(
    mut attacker_soldiers: usize,
    mut defender_soldiers: usize,
    simulations: usize,
) -> std::io::Result<()> {
    let initial_attacker_soldiers: usize = attacker_soldiers;
    let initial_defender_soldiers: usize = defender_soldiers;
    let mut attacker_wins = 0;

    for _ in 0..simulations {
        let mut rng = rand::thread_rng();

        while attacker_soldiers > 0 && defender_soldiers > 0 {
            let attacker_dice_count = attacker_soldiers.min(3);
            let defender_dice_count = defender_soldiers.min(2);

            let mut attacker_dice: Vec<_> = (0..attacker_dice_count)
                .map(|_| rng.gen_range(1..=6))
                .collect();
            let mut defender_dice: Vec<_> = (0..defender_dice_count)
                .map(|_| rng.gen_range(1..=6))
                .collect();

            attacker_dice.sort_unstable_by(|a, b| b.cmp(a));
            defender_dice.sort_unstable_by(|a, b| b.cmp(a));

            for (a, d) in attacker_dice.iter().zip(defender_dice.iter()) {
                if a > d {
                    defender_soldiers -= 1;
                } else {
                    attacker_soldiers -= 1;
                }
            }
        }

        if attacker_soldiers > 0 {
            attacker_wins += 1;
        }

        // Reset soldier counts for next simulation
        attacker_soldiers = initial_attacker_soldiers;
        defender_soldiers = initial_defender_soldiers;
    }

    let win_percentage = (f64::from(attacker_wins) / simulations as f64) * 100.0;
    let win_percentage_str = format!("{:.2}%", win_percentage);

    let colored_win_percentage = match win_percentage as usize {
        0..=49 => win_percentage_str.red(),
        50..=59 => win_percentage_str.yellow(),
        60..=100 => win_percentage_str.green(),
        _ => win_percentage_str.normal(),
    };

    println!(
        "Attacker win percentage with {} soldiers vs {}: {}",
        attacker_soldiers.to_string().cyan(),
        defender_soldiers.to_string().cyan(),
        colored_win_percentage
    );

    // Open the file in append mode (creates it if it doesn't exist)
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("data/battle_simulation_data.csv")?;

    // Write the data to the file
    writeln!(
        file,
        "{},{},{}",
        attacker_soldiers, defender_soldiers, win_percentage
    )?;

    Ok(())
}

#[allow(dead_code)]
fn print_out_basic_dice() -> std::io::Result<()> {
    println!(
        "{}",
        "-------------Attacker vs Defender Single Dice Roll Win Percentages-------------".cyan()
    );

    for attacker in (1..=3).rev() {
        for defender in (1..=2).rev() {
            run_basic_dice_simulation(attacker, defender)?;
        }
    }

    println!(
        "{}",
        "-------------------------------------------------------------------------------".cyan()
    );

    Ok(())
}

#[allow(dead_code)]
fn print_out_battle_data() -> std::io::Result<()> {
    println!(
        "{}",
        "--------------Attacker vs Defender Entire Battle Win Percentages---------------".cyan()
    );

    let simulations = 100_000;

    for attacker in (1..=20).rev() {
        for defender in (1..=20).rev() {
            run_battle_simulation(attacker, defender, simulations)?;
        }
    }

    println!(
        "{}",
        "--------------------------------------------------------------------------------".cyan()
    );

    Ok(())
}

fn erase_files() -> std::io::Result<()> {
    std::fs::remove_file("data/simulation_data.csv")?;
    std::fs::remove_file("data/battle_simulation_data.csv")?;
    Ok(())
}

fn execute_python_script() -> io::Result<Output> {
    let python_path = std::env::current_dir()?.join("myenv/scripts/python.exe");
    let script_path = std::env::current_dir()?.join("python/main.py");

    let output = std::process::Command::new(python_path)
        .arg(script_path)
        .output()?;

    Ok(output)
}

fn main() {
    match erase_files() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Failed to erase files: {}", e);
            std::process::exit(1);
        }
    }

    match print_out_basic_dice() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error handling basic dice data: {}", e);
            std::process::exit(1);
        }
    }

    match print_out_battle_data() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error handling battle data: {}", e);
            std::process::exit(1);
        }
    }

    match execute_python_script() {
        Ok(output) => {
            if !output.stdout.is_empty() {
                println!(
                    "Python script output: {}",
                    String::from_utf8_lossy(&output.stdout)
                );
            }
        }
        Err(e) => {
            eprintln!("Error executing python script: {}", e);
            std::process::exit(1);
        }
    }
}
