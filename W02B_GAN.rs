use std::io;

struct StudentRecords {
    name: String,
    degree: String,
    sn: u32,
}

fn main() {
    let mut students: Vec<StudentRecords> = Vec::new();

    let mut n = String::new();
    
    io::stdin().read_line(&mut n)
        .expect("Invalid Input!");
    let n: u32 = n.trim().parse().expect("Invalid input");

    // Input student records
    for _i in 0..n {
        
        let mut sn = String::new();
        io::stdin().read_line(&mut sn)
            .expect("Invalid input");
        let sn: u32 = sn.trim().parse().expect("Invalid input");
        
        let mut name = String::new();
        io::stdin().read_line(&mut name)
            .expect("Invalid input");

        let mut degree = String::new();
        io::stdin().read_line(&mut degree)
            .expect("Invalid input");

        let record = StudentRecords { name: name.trim().to_string(), degree: degree.trim().to_string(), sn };
        students.push(record);
    }

    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Invalid Input!");
    let x: u32 = x.trim().parse().expect("Invalid input");

    for _j in 0..x {
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let command: Vec<&str> = input.trim().split_whitespace().collect();

        let sn: u32 = match command[1].parse() {
            Ok(sn) => sn,
            Err(_) => {
                println!("Invalid student number");
                continue;
            }
        };

        if command[0] == "g" {

            let index_g = match students.iter().position(|s| s.sn == sn) {
            
                Some(index_g) => index_g,
                None => {
                
                    println!("Get {}", sn);
                    println!("    Does not exist");
                    continue;
                }
            };

            println!("Get {}", students[index_g].sn);
            println!("    Name: {}", students[index_g].name);
            println!("    Degree: {}", students[index_g].degree);
            continue;
        }

        else if command[0] == "u" {
            
            let index_u = match students.iter().position(|s| s.sn == sn) {
            
                Some(index_u) => index_u,
                None => {
                
                    println!("Update {} d", sn);
                    println!("    Does not exist");
                    continue;
                }
            };

            let degree = command[3..].to_owned();
            let degree = degree.join(" ");
            
            let name = &students[index_u].name;

            let new_record = StudentRecords { name: name.to_string(), degree: degree, sn };
            students[index_u] = new_record;
            println!("Update {} Degree", sn);
            println!("    Success");
        }
    }
}
