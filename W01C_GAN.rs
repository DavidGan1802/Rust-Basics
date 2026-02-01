use std::io;

fn main() {
    
    let mut testcases = String::new();
        
    io::stdin().read_line(&mut testcases)
        .expect("Invalid input!");

    let testcases_int: i64 = testcases.trim().parse().unwrap();

    for i in 1..testcases_int + 1 {
        
        let mut str_in = String::new();
        
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");
            
        let split_in: Vec <&str> = str_in.split(' ').collect();
        let p: i64 = split_in[0].trim().parse().expect("p is not a decimal number");
        let c: i64 = split_in[1].trim().parse().expect("c is not a decimal number"); 

        let amnt_pesos = convert_to_pesos(p);
        let amnt_cents = convert_to_pesos(c);

        if p != 0 && c!= 0 {
            println!("Case #{i}: {amnt_pesos}piso at {amnt_cents}sentimo lamang");
        } else if p == 0 && c!= 0 {
            println!("Case #{i}: {amnt_cents}sentimo lamang");
        } else if p != 0 && c== 0 {
            println!("Case #{i}: {amnt_pesos}piso lamang");
        }
    }
}

fn convert_to_pesos(num: i64) -> String {

    let one_to_nine = ["","isa", "dalawa", "tatlo", "apat", "lima", "anim", "pito", "walo", "siyam"];
    let ten_to_nineteen = ["sampu", "labingisa", "labingdalawa", "labingtatlo", "labingapat", "labinglima", "labinganim", "labingpito", "labingwalo", "labingsiyam"];
    let twenty_to_ninety = ["", "", "dalawangpu", "tatlongpu", "apatnapu", "limangpu", "animnapu", "pitongpu", "walongpu", "siyamnapu"];

    let mut words = String::new();
    let mut num = num.abs();

    while num > 0 {
        let mut hundreds = num % 1000;

        if hundreds > 0 {
            let mut hundreds_words = String::new();
            
            if hundreds >= 100 {
                if (hundreds/100) == 4 || (hundreds/100) == 6 || (hundreds/100) == 9 {
                    if (hundreds % 100) == 0 {
                        hundreds_words.push_str(one_to_nine[(hundreds / 100) as usize]);
                        hundreds_words.push_str(" na daang ");
                    } else {
                        hundreds_words.push_str(one_to_nine[(hundreds / 100) as usize]);
                        hundreds_words.push_str(" na daan at ");
                    }
                } else {
                    if (hundreds % 100) == 0 {
                        hundreds_words.push_str(one_to_nine[(hundreds / 100) as usize]);
                        hundreds_words.push_str("ng daang ");
                    } else {
                        hundreds_words.push_str(one_to_nine[(hundreds / 100) as usize]);
                        hundreds_words.push_str("ng daan at ");
                    }
                }
                hundreds %= 100;
            }

            if hundreds >= 20 {
                hundreds_words.push_str(twenty_to_ninety[(hundreds / 10) as usize]);
                if (hundreds % 10) == 0 {
                    hundreds_words.push_str("ng ");
                } else {
                    hundreds_words.push_str("'t ");
                }
                hundreds %= 10;
            }
           
            if hundreds > 0 {
                if hundreds < 10 {
                    hundreds_words.push_str(one_to_nine[(hundreds) as usize]);
                    if hundreds == 4 || hundreds == 6 || hundreds == 9 {
                        hundreds_words.push_str(" na ");
                    } else {
                        hundreds_words.push_str("ng ");
                    }
                } else {
                    hundreds_words.push_str(ten_to_nineteen[(hundreds - 10) as usize]);
                    if (hundreds-10) == 4 || (hundreds-10) == 6 || (hundreds-10) == 9 {
                        hundreds_words.push_str(" na ");
                    } else {
                        hundreds_words.push_str("ng ");
                    }
                }
            }

            if words.len() > 0 {
                words.insert_str(0, "libo't ");
            }

            words.insert_str(0, &hundreds_words);
        }
        num /= 1000;
    }
    return words
}