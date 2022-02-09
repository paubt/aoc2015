#[derive(Debug, Clone, Copy)]
struct GameState {
    player_hp: i32,
    mana: i32,
    mana_spend: u32,
    boss_hp: i32,
    boss_damage: u32,
    shield_rest_turns: u32,
    poison_rest_turns: u32,
    recharge_rest_turns: u32,
    win: Option<bool>,
}

impl GameState {
    fn new(
        player_hp: i32,
        mana: i32,
        mana_spend: u32,
        boss_hp: i32,
        boss_damage: u32,
        shield_rest_turns: u32,
        poison_rest_turns: u32,
        recharge_rest_turns: u32,
        win: Option<bool>,
    ) -> GameState {
        GameState {
            player_hp,
            mana,
            mana_spend,
            boss_hp,
            boss_damage,
            shield_rest_turns,
            poison_rest_turns,
            recharge_rest_turns,
            win,
        }
    }
    fn from(&self) -> GameState {
        GameState {
            player_hp: self.player_hp,
            mana: self.mana,
            mana_spend: self.mana_spend,
            boss_hp: self.boss_hp,
            boss_damage: self.boss_damage,
            shield_rest_turns: self.shield_rest_turns,
            poison_rest_turns: self.poison_rest_turns,
            recharge_rest_turns: self.recharge_rest_turns,
            win: self.win,
        }
    }
}

fn part_one() -> u32 {
    let start_game_state: GameState = GameState::new(50, 500, 0, 71, 10, 0, 0, 0, None);
    //let test_game_state: GameState = GameState::new(14, 250, 0, 13, 8, 0, 0, 0, None);
    let mut open_game_states: Vec<(bool, GameState)> = vec![(true, start_game_state)];
    let mut finished_game_state: Vec<GameState> = vec![];

    fn expand_gamestate(in_game_state: GameState) -> Option<Vec<(bool, GameState)>> {
        fn decrement_effects(mut g: GameState) -> (GameState, u32) {
            let temporary_armor: u32;
            {
                if g.shield_rest_turns > 0 {
                    g.shield_rest_turns = g.shield_rest_turns - 1;
                    temporary_armor = 7;
                } else {
                    temporary_armor = 0;
                }
                if g.poison_rest_turns > 0 {
                    g.poison_rest_turns = g.poison_rest_turns - 1;
                    g.boss_hp = g.boss_hp - 3;
                }
                if g.recharge_rest_turns > 0 {
                    g.recharge_rest_turns = g.recharge_rest_turns - 1;
                    g.mana = g.mana + 101;
                }
            }
            (g, temporary_armor)
        }

        let mut new_game_states: Vec<(bool, GameState)> = vec![];
        // update the effects
        let (mut in_game_state, _) = decrement_effects(in_game_state);
        // check if poison killed boss ->  ergo win
        if in_game_state.boss_hp <= 0 {
            in_game_state.win = Some(true);
            return Some(vec![(false, in_game_state)]);
        }
        // player turn
        // go through the spells and add them as possible game state to the return Vec
        {
            // magic missle
            if in_game_state.mana >= 53 {
                let mut gs: GameState = in_game_state.from();
                gs.mana = gs.mana - 53;
                gs.mana_spend = gs.mana_spend + 53;
                gs.boss_hp = gs.boss_hp - 4;
                if gs.boss_hp > 0 {
                    new_game_states.push((true, gs));
                } else {
                    gs.win = Some(true);
                    new_game_states.push((false, gs));
                }
            }
            // drain
            if in_game_state.mana >= 73 {
                let mut gs: GameState = in_game_state.from();
                gs.mana = gs.mana - 73;
                gs.mana_spend = gs.mana_spend + 73;
                gs.boss_hp = gs.boss_hp - 2;
                gs.player_hp = gs.player_hp + 2;
                if gs.boss_hp > 0 {
                    new_game_states.push((true, gs));
                } else {
                    gs.win = Some(true);
                    new_game_states.push((false, gs));
                }
            }
            // shield
            if in_game_state.mana >= 113 && in_game_state.shield_rest_turns == 0 {
                let mut gs: GameState = in_game_state.from();
                gs.mana = gs.mana - 113;
                gs.mana_spend = gs.mana_spend + 113;
                gs.shield_rest_turns = 6;
                new_game_states.push((true, gs));
            }
            // poison
            if in_game_state.mana >= 173 && in_game_state.poison_rest_turns == 0 {
                let mut gs: GameState = in_game_state.from();
                gs.mana = gs.mana - 173;
                gs.mana_spend = gs.mana_spend + 173;
                gs.poison_rest_turns = 6;
                new_game_states.push((true, gs));
            }
            // recharge
            if in_game_state.mana >= 229 && in_game_state.recharge_rest_turns == 0 {
                let mut gs: GameState = in_game_state.from();
                gs.mana = gs.mana - 229;
                gs.mana_spend = gs.mana_spend + 229;
                gs.recharge_rest_turns = 5;
                new_game_states.push((true, gs));
            }
        }

        // boss turn for all that have true for still not decided
        let new_game_states: Vec<(bool, GameState)> = new_game_states
            .iter_mut()
            .map(|(b, g)| {
                if *b == true {
                    // decrement effects
                    let (mut g, t) = decrement_effects(*g);
                    let mut actual_boss_damage = g.boss_damage - t;
                    if actual_boss_damage <= 0 {
                        actual_boss_damage = 1;
                    }
                    g.player_hp = g.player_hp - actual_boss_damage as i32;
                    return if g.player_hp <= 0 {
                        g.win = Some(false);
                        (false, g)
                    } else {
                        (true, g)
                    };
                } else {
                    g.win = Some(false);
                    return (false, *g);
                };
            })
            .collect();
        // if the vec is empty this means that not enugth mana ergo RIP
        if new_game_states.len() == 0 {
            in_game_state.win = Some(false);

            Some(vec![(false, in_game_state)])
        } else {
            Some(new_game_states)
        }
    }

    while !open_game_states.is_empty() {
        // expand each game
        let new_game_states: Vec<(bool, GameState)> = open_game_states
            .iter()
            .map(|(_, g)| expand_gamestate(*g).unwrap())
            .fold(vec![], |mut acc, mut x| {
                acc.append(&mut x);
                acc
            });
        // move all finished ones to finished Vec
        let mut f_g: Vec<GameState> = new_game_states
            .iter()
            .filter(|(b, g)| *b == false)
            .map(|(_, g)| *g)
            .collect();
        finished_game_state.append(&mut f_g);
        // keep all the ones with true (ergo that need further expanding)
        open_game_states = new_game_states
            .iter()
            .filter(|(b, g)| *b == true)
            .map(|(_, g)| (true, *g))
            .collect();
        //print!("{:#?}\n###########\n\n", finished_game_state);
    }
    // go through the finished ones and find the

    let t: GameState = finished_game_state
        .into_iter()
        .filter(|g| g.win.unwrap() == true)
        .min_by_key(|g| g.mana_spend)
        .unwrap();
    print!("{:#?}", t);

    0
}

fn part_two() -> u32 {
    0
}

fn main() {
    let answer_part_one = part_one();

    let answer_part_two = part_two();

    println!(
        "answer part 1: {}\nanswer part 2: {}",
        answer_part_one, answer_part_two
    );
}
