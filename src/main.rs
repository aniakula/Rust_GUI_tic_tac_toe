use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([170.0, 250.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Tic Tac Toe",
        options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    )
}


struct TicTacToeApp{
    board: [[CellState; 3]; 3],
    current_player: Player,
    state_text: String,
    active: bool,
}

impl Default for TicTacToeApp {
    fn default() -> Self {
        TicTacToeApp {
            board: [[CellState::default(); 3]; 3],
            current_player: Player::default(),
            state_text: "Player X's turn".to_string(),
            active: true,
        }
    }
}

impl TicTacToeApp{
    fn clear(&mut self){
        self.board = [[CellState::Empty; 3]; 3];
        self.current_player = Player::X;
        self.state_text = "Player X's turn".to_string();
        self.active = true
    }

    fn is_full(&self) -> bool{
        for row in 0..3{
            for col in 0..3{
                if self.board[row][col] == CellState::Empty {
                    return false;
                }
            }
        }
        return true;
    }

    fn check_winner(&self) -> Option<Player>{
        for row in 0..3{
            if self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2]{
                match self.board[row][0]{
                    CellState::X => return Some(Player::X),
                    CellState::O => return Some(Player::O),
                    _ => (),
                }
            }
        }

        for col in 0..3{
            if self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col]{
                match self.board[0][col]{
                    CellState::X => return Some(Player::X),
                    CellState::O => return Some(Player::O),
                    _ => (),
                }
            }
        }

        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]{
            match self.board[0][0]{
                CellState::X => return Some(Player::X),
                CellState::O => return Some(Player::O),
                _ => (),
            }
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0]{
            match self.board[0][2]{
                CellState::X => return Some(Player::X),
                CellState::O => return Some(Player::O),
                _ => (),
            }
        }

        None
        
    }
    
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    X,
    O,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Empty
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl Default for Player {
    fn default() -> Self {
        Player::X
    }
}



impl eframe::App for TicTacToeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx,|ui|{
        ui.vertical_centered(|ui|{
            ui.heading("Tic Tac Toe");
            let clear_button = ui.add(egui::Button::new("Clear"));
            if clear_button.clicked(){
                self.clear();
            }
        });
        ui.vertical_centered(|ui|{
            egui::Grid::new("tictactoe_grid")
                .spacing([2.0, 2.0])
                .show(ui, |ui| {
                    for row in 0..3 {
                        for col in 0..3 {
                            
                            let cell = &mut self.board[row][col];
                            let button_label = match cell {
                                CellState::Empty => " ",
                                CellState::X => "X",
                                CellState::O => "O",
                            };

                            let button = ui.add_sized(
                                [50.0, 50.0],
                                egui::Button::new(button_label)
                            );

                            if button.clicked() && self.active && *cell == CellState::Empty {
                                *cell = match self.current_player {
                                    Player::X => CellState::X,
                                    Player::O => CellState::O,
                                };
                                self.current_player = match self.current_player {
                                    Player::X => Player::O,
                                    Player::O => Player::X,
                                };
                                self.state_text = format!("Player {}'s turn", match self.current_player{
                                    Player::X => "X",
                                    Player::O => "O",
                                });
                            }
                        }
                        ui.end_row();
                    }
                });
            });
            
        if self.active {
                if self.is_full() && self.check_winner().is_none() {
                    self.state_text = "It's a draw!".to_string();
                } else if self.check_winner().is_some() {
                    let winner = match self.check_winner(){
                        Some(Player::X) => "X".to_string(),
                        Some(Player::O) => "O".to_string(),
                        _ => "X".to_string(),
                    };
                    self.state_text = format!("Player {} wins!", winner);
                    self.active = false;
                } 
        }
        ui.vertical_centered(|ui|{
            ui.heading(&self.state_text);
        });
        
    });
    }
}
