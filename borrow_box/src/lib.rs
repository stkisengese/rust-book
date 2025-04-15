#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games
        })
    }
    pub fn read_winner(&self) -> (String, u16) {
        let (p1_name, p1_score) = &self.p1;
        let (p2_name, p2_score) = &self.p2;
    
        if p1_score > p2_score {
            (p1_name.clone(), *p1_score)
        } else if p2_score > p1_score {
            (p2_name.clone(), *p2_score)
        } else {
            ("Same score! tied".to_string(), *p1_score)
        }
        

    }
    pub fn update_score(&mut self, user_name: String) {
        let max_score = (self.nb_games / 2) + 1;
        if self.p1.1 >= max_score || self.p2.1 >= max_score {
            return;
        }
        if user_name == self.p1.0 {
            self.p1.1 += 1;
        } else if user_name == self.p2.0 {
            self.p2.1 += 1;
        }
    }
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
