use std::io::{stdin,stdout,Write};

#[derive(Debug)]
enum FitnessActivity {
    Walk(i64),
    Run(i64),
    Bike(i64),
    Eat(i64),
    Sleep(i64),
    Idle(i64),
}

fn main() {

    let mut testcases = String::new();
        
    stdin().read_line(&mut testcases)
        .expect("Invalid input!");

    let testcases_int: i64 = testcases.trim().parse().unwrap();

    for i in 1..testcases_int + 1 {

        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let mut goals = input.trim().split(' ');
        let exercise_goals: i64 = goals.next().unwrap().parse().unwrap();
        let sleep_goals: i64 = goals.next().unwrap().parse().unwrap();

        input.clear();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut activities: Vec<FitnessActivity> = Vec::new();
        for _j in 1..=n {
            input.clear();
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();

            let mut parts = input.trim().split(' ');
            let activity_str = parts.next().unwrap();
            let duration: i64 = parts.next().unwrap().parse().unwrap();

            let activity = match activity_str {
                "w" => FitnessActivity::Walk(duration),
                "r" => FitnessActivity::Run(duration),
                "b" => FitnessActivity::Bike(duration),
                "e" => FitnessActivity::Eat(duration),
                "s" => FitnessActivity::Sleep(duration),
                "i" => FitnessActivity::Idle(duration),
                _ => panic!("Invalid activity"),
            };

            activities.push(activity);
        }

        let mut exercise_duration = 0;
        let mut rest_duration = 0;
        let mut max_exercise_duration = 0;
        let mut max_rest_duration = 0;
        let mut exercise_duration1 = 0;
        let mut sleep_duration1 = 0;

        for activity in activities {
            match activity {
                FitnessActivity::Walk(t) => {
                    exercise_duration += t;
                    rest_duration = 0;
                    exercise_duration1 += t;
                }
                FitnessActivity::Run(t) => {
                    exercise_duration += t;
                    rest_duration = 0;
                    exercise_duration1 += t;
                }
                FitnessActivity::Bike(t) => {
                    exercise_duration += t;
                    rest_duration = 0;
                    exercise_duration1 += t;
                }
                FitnessActivity::Eat(t) => {
                    exercise_duration = 0;
                    rest_duration += t;
                }
                FitnessActivity::Sleep(t) => {
                    exercise_duration = 0;
                    rest_duration += t;
                    sleep_duration1 += t;
                }
                FitnessActivity::Idle(t) => {
                    exercise_duration = 0;
                    rest_duration += t;
                }
            }

            max_exercise_duration = max_exercise_duration.max(exercise_duration);
            max_rest_duration = max_rest_duration.max(rest_duration);
        }

        println!("Case #{}:", i);
        if exercise_duration1 >= exercise_goals {
            println!("Exercise goal reached")
        } else {
            println!("Exercise goal not reached")
        }
        if sleep_duration1 >= sleep_goals {
            println!("Sleep goal reached")
        } else {
            println!("Sleep goal not reached")
        }
        println!("Max exercise duration: {}m", max_exercise_duration);
        println!("Max rest duration: {}m", max_rest_duration);

    }

}
