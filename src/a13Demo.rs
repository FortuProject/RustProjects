struct Test {
    score: i32,
}

pub fn main() {

    let my_scores = vec![
        Test{score: 90},
        Test{score: 88},
        Test{score: 77},
        Test{score: 93},
    ];

    for Test in my_scores {
        println!("score = {:?}", Test.score);
    }
}