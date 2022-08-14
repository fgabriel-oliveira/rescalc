use std::io::stdin;

fn parallel(res_list: Vec<usize>) -> usize {
    let mut req: f64 = 0.0;
    for r in res_list {
        req += 1.0 / r as f64;
    }
    if req != 0.0{
        (1.0 / req) as usize
    }
    else {
        0
    }
}

fn serie(res_list: Vec<usize>) -> usize {
    let mut req: usize = 0;
    for r in res_list {
        req += r;
    }
    req
}

fn main() {

    loop {
        let mut line: String = String::new();
        stdin()
            .read_line(&mut line)
            .expect("alguma coisa deu errado");
        
        let splited_line: Vec<&str> = line
            .split(|c| c == ' ' || c == '\n')
            .collect();
        
        match splited_line[0] {
            "/parallel" => {
                let mut res_list: Vec<usize> = Vec::new();
                if splited_line.len() > 2 {
                    for v in &splited_line[1..splited_line.len() - 1] {
                        res_list.push(v.parse::<usize>().unwrap());
                    }
                    println!("{}", parallel(res_list));

                }
                else {
                    println!("valores inseridos insuficientes");
                }
               
            }
            "/serie" => {
                let mut res_list: Vec<usize> = Vec::new();
                if splited_line.len() > 2 {
                    for v in &splited_line[1..splited_line.len() - 1] {
                        res_list.push(v.parse::<usize>().unwrap());
                    }
                    println!("{}", serie(res_list));

                }
                else {
                    println!("valores inseridos insuficientes");
                }
            }
            "/exit" => break,
            _ => panic!("alguma coisa deu errado"),
        }
    }
    
}