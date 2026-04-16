pub struct Players {
    pub first_name: String,
    pub last_name: String,
    pub team: String
}

impl Players {
    pub fn new() -> Self {
        Self { first_name: String::new(), last_name: String::new(), team: String::new() }
    }

    pub fn new_player(&mut self, first_name: String, last_name: String, team: String) {
        self.first_name = first_name;
        self.last_name = last_name;
        self.team = team;
    }
}