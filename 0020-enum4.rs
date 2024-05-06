#[derive(Debug)]

enum Teams {

    Counters(u8),
    YoungJumpers(u8),

}

fn main() {

    let counter_team_people = Teams::Counters(22); // number of team members

    let youngjumpers_team_people = Teams::YoungJumpers(18);

}

