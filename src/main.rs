use std::io::*;
fn main() {
    lotka_volterra();
}
fn get_f32(stdin: &mut Stdin, line : &mut String) -> f32 {
    stdin.lock().read_line(line).unwrap();
    let x : f32 = line.trim().parse().unwrap();
    line.clear();
    x
}
fn get_u32(stdin: &mut Stdin, line : &mut String) -> u32 {
    stdin.lock().read_line(line).unwrap();
    let x : u32 = line.trim().parse().unwrap();
    line.clear();
    x
}

fn get_string(stdin: &mut Stdin) -> String {
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line.truncate(line.len() - 2);
    line
} 

fn lotka_volterra() {
    println!("This is a Lotka-Volterra Simulator");
    let mut stdin = std::io::stdin();
    println!("Enter the name of the predator species");
    let predator_name = get_string(&mut stdin);
    println!("Enter the name of the prey species");
    let prey_name = get_string(&mut stdin);
    println!("Enter the parameters alpha, beta, gamma and delta each on seperate lines");
    let (params, x_0, y_0, h, no_iters) = {
        let mut array : [f32 ; 4] = [0.0; 4];
        let mut line = String::new();
        for i in 0..4 {
            array[i] = get_f32(&mut stdin, &mut line);
        }
        println!("Enter the starting population of {}", &prey_name);
        let x = get_f32(&mut stdin, &mut line);
        println!("Enter the starting population of {}", &predator_name);
        let y = get_f32(&mut stdin, &mut line);
        println!("Enter the timestep");
        let h =  get_f32(&mut stdin, &mut line);
        println!("Enter the number of iterations");
        /*let no_iters : usize = {
            let dur = get_f32(&mut stdin, &mut line);
            ((dur / h) as i32).try_into().unwrap()
        }; */
        let no_iters = get_u32(&mut stdin, &mut line);
        (array, x, y, h, no_iters)
    }; 
    println!("alpha = {}, beta = {}, gamma = {}, delta =  {}", params[0], params[1], params[2], params[3]);
    let mut datapoints = Vec::with_capacity(no_iters as usize);
    let mut x = x_0;
    let mut y = y_0;
    let mut timestamp = 0.0;
    datapoints.push((timestamp, x, y));
    for i in 0..no_iters {
        let x_new = x + h * (x * (params[0] - params[1] * y));
        let y_new = y + h * (y * (params[3] * x - params[2]));
        x = x_new;
        y = y_new;
        timestamp += h;
        datapoints.push((timestamp, x, y));
    }
    println!("time\t{}\t{}", &prey_name, &predator_name);
    for datapoint in datapoints {
        println!("{:.4}\t{:.4}\t{:.4}", datapoint.0, datapoint.1, datapoint.2);
    }
}
//lotka_volterra
//x_new = x + h * (x * (alpha - beta * y));
//y_new = y + h * (y * (delta * x - gamma));
//x = x_new;
//y = y_new;

//competive lotka_volterra
//x_new[i] = x[i] + h * (r[i] * x[i] * (1 - (1/K[i]) * (A.row(i).dot(x))));
//x[i] = x_new[i];

//general lotka_volterra
//x_new[i] = x[i] + h * (r[i] + A.row(i).dot(x));
//x[i] = x_new[i];

