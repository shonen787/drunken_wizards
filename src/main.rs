#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use std::{collections::HashMap, fmt::format};
use rand::{thread_rng, Rng};

struct MyApp {
    name: String,
    age: u32,
    shown: bool,
    rules: HashMap<u32, String>,
    guard: bool,
}

impl MyApp{
    fn add_rules(MyApp: &mut  MyApp, key: u32, value: String) {
        MyApp.rules.insert(key, value);
      }
}

impl Default for MyApp {

    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            shown: false, 
            rules: HashMap::new(),
            guard: true,
        }

  } 
}



impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.guard {
                MyApp::add_rules(self, 0, String::from("Sol Ring turn 1"));
                MyApp::add_rules(self, 1, String::from("Plays 3 or more lands in 1 turn (fetching counts as 2)"));
                MyApp::add_rules(self, 2, String::from("Plays Thassa’s Oracle (chugs 2 beers)"));
                MyApp::add_rules(self, 3, String::from("Tutors for a card turn 1"));
                MyApp::add_rules(self, 4, String::from("If there are 3 or more counter spells on a stack, each counter owner must take a shot"));
                MyApp::add_rules(self, 5, String::from("Someone plays a counter"));
                MyApp::add_rules(self, 6, String::from("(everyone including bido) Bido says something about white power when playing Heliod"));
                MyApp::add_rules(self, 7, String::from("Drink for every draw phase this turn"));
                MyApp::add_rules(self, 8, String::from("When a player taxes, that player takes a swig of beer (causes something to come in tapped)"));
                MyApp::add_rules(self, 9, String::from("Plays a board wipe"));
                MyApp::add_rules(self, 10, String::from("Tax Fraud: Not paying a tax"));
                MyApp::add_rules(self, 11, String::from("Casting a spell without paying mana"));
                MyApp::add_rules(self, 12, String::from("If you draw a card(s) outside your draw step (once if you draw five at once but per card if you draw 5 separate cards)"));
                MyApp::add_rules(self, 13, String::from("Casting a spell from anywhere other than your hand"));
                MyApp::add_rules(self, 14, String::from("Taking an extra turn (shot)"));
                MyApp::add_rules(self, 15, String::from("Monarchs Rule: First blood becomes the Monarch. When monarch triggers, add a drink to the kings cup. If you end the game as the monarch, drink the cup. Add cups as necessary."));
                MyApp::add_rules(self, 16, String::from("Wipe out!: Take a sip for every creature destroyed on a wipe"));
                MyApp::add_rules(self, 17, String::from("Whenever a player says \"it resolves\"/”i’ll allow it” , they take a shot."));
                MyApp::add_rules(self, 18, String::from("Down with the ship: If your commander dies/tucked/exiled/etc. Sip a beer.."));
                MyApp::add_rules(self, 19, String::from("You can take a double shot instead of each card you would mulligan"));
                MyApp::add_rules(self, 20, String::from("If a turn lasts more than 3 minutes each player takes a shot. Turn player takes 2. .It was nice knowing you Bido"));
                MyApp::add_rules(self, 21, String::from("Drinks on Me!: Add your own rule."));
                MyApp::add_rules(self, 22, String::from("Burn baby burn!! Take a shot for the mana burn"));
                MyApp::add_rules(self, 23, String::from("Ante up: Players can place bets on drinks split among the losers."));
                MyApp::add_rules(self, 24, String::from("Shuffle up: If you hand shuffle, take a shot. (The Chao killer)"));
                MyApp::add_rules(self, 25, String::from("Phyrexian Mana: take a shot."));

            }else {self.guard = false;}
            let mut rng = thread_rng();
            let mut list: Vec<u32> = Vec::new();
            let mut n: u32 = rng.gen_range(0..self.rules.keys().len() as u32);
            let  mut looper: bool = true;

            
            ui.heading("Drunken Wizards");
            ui.horizontal(|ui|{
                ui.label("Click the button to get the rules.");
            });
            if ui.button("Let's get the rules!!").clicked() {
                if self.shown{self.shown = false}else{self.shown = true;}
                looper = true;
            }

            if self.shown && looper {
                ui.group(|ui|{
                    let mut size = self.rules.keys().len() as i32;
                    let mut i = 0;
                    while i < 5{
                        if  list.contains(&n){
                            n = rng.gen_range(0..self.rules.keys().len() as u32);
                        } else {
                            list.push(n);
                            ui.label(format!("Rule {}:\t{}", i, self.rules[&n]));
                            ui.label("------");
                            i+=1;
                        }

                    }
                    
                });
                looper = false;
            }


        });
    }
}




fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Drunk Wizards",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
