#[derive(Debug)]
struct Item {
    name: char,
    gold: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn new(name: char, gold: u32, damage: u32, armor: u32) -> Item {
        Item {
            name,
            gold,
            damage,
            armor,
        }
    }
}

#[derive(Debug)]
struct ItemNode {
    total_gold: u32,
    total_damage: u32,
    total_armor: u32,
    items: Vec<char>,
}

impl ItemNode {
    fn new(total_gold: u32, total_damage: u32, total_armor: u32, items: Vec<char>) -> ItemNode {
        ItemNode {
            total_gold,
            total_damage,
            total_armor,
            items,
        }
    }
}

fn part_one() -> u32 {
    // the items
    let weapons: Vec<Item> = vec![
        Item::new('D', 8, 4, 0),
        Item::new('S', 10, 5, 0),
        Item::new('W', 25, 6, 0),
        Item::new('L', 40, 7, 0),
        Item::new('G', 74, 8, 0),
    ];
    let armors: Vec<Item> = vec![
        Item::new('N', 0, 0, 0),
        Item::new('L', 13, 0, 1),
        Item::new('C', 31, 0, 2),
        Item::new('S', 53, 0, 3),
        Item::new('B', 75, 0, 4),
        Item::new('P', 102, 0, 5),
    ];
    let rings: Vec<Item> = vec![
        Item::new('n', 0, 0, 0),
        Item::new('N', 0, 0, 0),
        Item::new('1', 25, 1, 0),
        Item::new('2', 50, 2, 0),
        Item::new('3', 100, 3, 0),
        Item::new('4', 20, 0, 1),
        Item::new('5', 40, 0, 2),
        Item::new('6', 80, 0, 3),
    ];

    let stack_weapons_added: Vec<ItemNode> = weapons
        .iter()
        .map(|w| ItemNode::new(w.gold, w.damage, w.armor, vec![w.name]))
        .collect();

    let mut stack_armor_added: Vec<ItemNode> = vec![];
    // for each weapons create n armor combis
    for w in &stack_weapons_added {
        for a in &armors {
            let new_node: ItemNode = ItemNode::new(
                w.total_gold + a.gold,
                w.total_damage + a.damage,
                w.total_armor + a.armor,
                {
                    let mut t = w.items.clone();
                    t.push(a.name);
                    t
                },
            );
            stack_armor_added.push(new_node);
        }
    }

    // add the two rings
    let mut stack_rings_added: Vec<ItemNode> = vec![];
    for i in &stack_armor_added {
        for r1 in &rings {
            for r2 in rings.iter().filter(|x| r1.name != x.name) {
                let new_node: ItemNode = ItemNode::new(
                    i.total_gold + r1.gold + r2.gold,
                    i.total_damage + r1.damage + r2.damage,
                    i.total_armor + r1.armor + r2.armor,
                    {
                        let mut t = i.items.clone();
                        t.push(r1.name);
                        t.push(r2.name);
                        t
                    },
                );
                stack_rings_added.push(new_node);
            }
        }
    }

    // fn to calc if win or not
    fn fight_boss_with_items(items: &ItemNode) -> bool {
        let mut player_hit_points: i32 = 100;
        let mut boss_hit_points: i32 = 104;
        let boss_damage: i32 = 8;
        let boss_armor: i32 = 1;

        let mut player_damage: i32 = items.total_damage as i32 - boss_armor;
        if player_damage < 1 {
            player_damage = 1;
        }
        let mut boss_damage: i32 = boss_damage - items.total_armor as i32;
        if boss_damage < 1 {
            boss_damage = 1;
        }
        loop {
            boss_hit_points = boss_hit_points - player_damage;
            if boss_hit_points < 1 {
                return true;
            }
            player_hit_points = player_hit_points - boss_damage;
            if player_hit_points < 1 {
                return false;
            }
        }
    }
    stack_rings_added
        .iter()
        .filter(|i| fight_boss_with_items(i))
        .reduce(|accum, i| {
            if accum.total_gold <= i.total_gold {
                accum
            } else {
                i
            }
        })
        .unwrap()
        .total_gold
}

fn part_two() -> u32 {
    // the items
    let weapons: Vec<Item> = vec![
        Item::new('D', 8, 4, 0),
        Item::new('S', 10, 5, 0),
        Item::new('W', 25, 6, 0),
        Item::new('L', 40, 7, 0),
        Item::new('G', 74, 8, 0),
    ];

    let armors: Vec<Item> = vec![
        Item::new('N', 0, 0, 0),
        Item::new('L', 13, 0, 1),
        Item::new('C', 31, 0, 2),
        Item::new('S', 53, 0, 3),
        Item::new('B', 75, 0, 4),
        Item::new('P', 102, 0, 5),
    ];
    let rings: Vec<Item> = vec![
        Item::new('n', 0, 0, 0),
        Item::new('N', 0, 0, 0),
        Item::new('1', 25, 1, 0),
        Item::new('2', 50, 2, 0),
        Item::new('3', 100, 3, 0),
        Item::new('4', 20, 0, 1),
        Item::new('5', 40, 0, 2),
        Item::new('6', 80, 0, 3),
    ];
    let stack_weapons_added: Vec<ItemNode> = weapons
        .iter()
        .map(|w| ItemNode::new(w.gold, w.damage, w.armor, vec![w.name]))
        .collect();

    let mut stack_armor_added: Vec<ItemNode> = vec![];
    // for each weapons create n armor combis
    for w in &stack_weapons_added {
        for a in &armors {
            let new_node: ItemNode = ItemNode::new(
                w.total_gold + a.gold,
                w.total_damage + a.damage,
                w.total_armor + a.armor,
                {
                    let mut t = w.items.clone();
                    t.push(a.name);
                    t
                },
            );
            stack_armor_added.push(new_node);
        }
    }
    // add the two rings
    let mut stack_rings_added: Vec<ItemNode> = vec![];
    for i in &stack_armor_added {
        for r1 in &rings {
            for r2 in rings.iter().filter(|x| r1.name != x.name) {
                let new_node: ItemNode = ItemNode::new(
                    i.total_gold + r1.gold + r2.gold,
                    i.total_damage + r1.damage + r2.damage,
                    i.total_armor + r1.armor + r2.armor,
                    {
                        let mut t = i.items.clone();
                        t.push(r1.name);
                        t.push(r2.name);
                        t
                    },
                );
                stack_rings_added.push(new_node);
            }
        }
    }

    // fn to calc if win or not
    fn fight_boss_with_items(items: &ItemNode) -> bool {
        let mut player_hit_points: i32 = 100;
        let mut boss_hit_points: i32 = 104;
        let boss_damage: i32 = 8;
        let boss_armor: i32 = 1;

        let mut player_damage: i32 = items.total_damage as i32 - boss_armor;
        if player_damage < 1 {
            player_damage = 1;
        }
        let mut boss_damage: i32 = boss_damage - items.total_armor as i32;
        if boss_damage < 1 {
            boss_damage = 1;
        }
        loop {
            boss_hit_points = boss_hit_points - player_damage;
            if boss_hit_points < 1 {
                return true;
            }
            player_hit_points = player_hit_points - boss_damage;
            if player_hit_points < 1 {
                return false;
            }
        }
    }

    stack_rings_added
        .iter()
        .filter(|i| !fight_boss_with_items(i))
        .reduce(|accum, i| {
            if accum.total_gold >= i.total_gold {
                accum
            } else {
                i
            }
        })
        .unwrap()
        .total_gold
}

fn main() {
    let answer_part_one = part_one();

    let answer_part_two = part_two();

    println!(
        "answer part 1: {}\nanswer part 2: {}",
        answer_part_one, answer_part_two
    );
}
