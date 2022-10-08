use std::io::*;
use std::env::*;
use ndarray::prelude::*;
use std::fmt::Write;
fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "competitive" {
        competitive_lotka_volterra();
    } else {
        lotka_volterra();
    }
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
        let h = get_f32(&mut stdin, &mut line);
        println!("Enter the duration of the simulation");
        let no_iters : usize = {
            let dur = get_f32(&mut stdin, &mut line);
            (dur / h) as usize
        }; 
        //let no_iters = get_u32(&mut stdin, &mut line);
        (array, x, y, h, no_iters)
    }; 
    println!("alpha = {}, beta = {}, gamma = {}, delta =  {}", params[0], params[1], params[2], params[3]);
    let mut datapoints = Vec::with_capacity((no_iters as usize) + 1);
    let mut x = x_0;
    let mut y = y_0;
    let mut timestamp = 0.0;
    datapoints.push((timestamp, x, y));
    for _i in 0..no_iters {
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
fn write_table_line_species(buffer : &mut String, names : &[String]) {
    for name in names {
        write!(buffer, "{}\t", &name);
    }
}
fn write_row(buffer : &mut String, row : ArrayView1<f32>){
    for entry in row.iter() {
        write!(buffer, "{:.4}\t", entry);
    }
}
fn competitive_lotka_volterra() {
    println!("This is a Competitive Lotka-Volterra Simulator");
    let mut stdin = std::io::stdin();
    let mut line = String::new(); 
    println!("Enter the number of species");
    let no_species = get_u32(&mut stdin, &mut line) as usize;
    println!("Enter the name of each species");
    let species_names : Vec<String> = {
        let mut names = Vec::with_capacity(no_species);
        for _i in 0..no_species {
            names.push(get_string(&mut stdin));
        }
        names
    }; 
    let (r, k_inv, mut x) = {
        //TODO: Optimize
        let mut r_vec : Vec<f32> = Vec::with_capacity(no_species);
        let mut k_vec : Vec<f32> = Vec::with_capacity(no_species);
        let mut x_vec : Vec<f32> = Vec::with_capacity(no_species);
        for name in &species_names {
            println!("Enter the reproduction rate for {}", name);
            r_vec.push(get_f32(&mut stdin, &mut line));
            println!("Enter the carrying capacity for {}", name);
            k_vec.push(get_f32(&mut stdin, &mut line));
            println!("Enter the initial population for {}", &name);
            x_vec.push(get_f32(&mut stdin, &mut line));
        }
        let r = Array1::from(r_vec);
        let k_inv = Array1::from(k_vec.iter().map(|&z| 1.0/z).collect::<Vec<_>>());
        let x = Array1::from(x_vec);
        (r, k_inv, x)
    };
    let (community_matrix, h, no_iters) = {
        //let no_interactions = no_species * no_species;
        let mut matrix : Array2<f32> = Array2::zeros((no_species, no_species));
        for i in 0..no_species{
            let mut current_row = matrix.slice_mut(s![i as usize, ..]);
            let current_species = &species_names[i];
            for j in 0..no_species{
                if i == j {
                    current_row[j] = 1.0;                 
                } else {
                    println!("Enter the coefficent between {} and {}", current_species, &species_names[j]);
                    current_row[j] = get_f32(&mut stdin, &mut line);
                }
            }
            current_row *= k_inv[i];
        }
        //let mut interactions : Vec<f32> = Vec::with_capacity(no_interactions);
        /* println!("Enter the community matrix in row major order");
        for _i in 0..no_interactions {
            interactions.push(get_f32(&mut stdin, &mut line));
        } */
        //let matrix = Array2::from_shape_vec((no_species, no_species), interactions).unwrap();
        println!("Enter the timestep");
        let h =  get_f32(&mut stdin, &mut line);
        println!("Enter the duration of the simulation");
        let no_iters : usize = {
            let dur = get_f32(&mut stdin, &mut line);
            (dur / h) as usize
        }; 
        (matrix, h, no_iters)
    };
    let mut datapoints : Array2<f32> = Array::zeros((no_iters + 1, no_species));
    {
        let mut current_row = datapoints.slice_mut(s![0 as usize, ..]);
        current_row.assign(&x);
    }
    let end = (no_iters as f32) * h;
    let timevec = Array1::<f32>::linspace(0.0, end, no_iters + 1);
    let ones = Array1::<f32>::ones(no_species);
    //TODO: Optimize?
    for i in 0..no_iters {
        let row_index = i as usize;
        x.assign(&datapoints.slice(s![row_index, ..]));
        let mut current_row = datapoints.slice_mut(s![row_index + 1, ..]);
        //let x_new = &x + &(h * (((&r * &x) * &k_inv) * community_matrix.dot(&x)));
        //let x_new = &x + &(h * (((&r * &x) * (&ones - &(community_matrix.dot(&x) * &k_inv)))));
        let x_new = &x + &(h * ((&r * &x) * (&ones - &(community_matrix.dot(&x)))));
        current_row.assign(&x_new);
    }
    write!(&mut line, "time\t");
    write_table_line_species(&mut line, &species_names);
    println!("{}", &line);
    line.clear();
    for i in 0..(no_iters + 1) {
        write!(&mut line, "{:.4}\t", timevec[i]);
        write_row(&mut line, datapoints.slice(s![i, ..]));
        println!("{}", &line);
        line.clear();
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

fn generalized_lotka_volterra() {
    println!("This is a General Lotka-Volterra Simulator");
    let mut stdin = std::io::stdin();
    let mut line = String::new(); 
    println!("Enter the number of species");
    let no_species = get_u32(&mut stdin, &mut line) as usize;
    println!("Enter the name of each species");
    let species_names : Vec<String> = {
        let mut names = Vec::with_capacity(no_species);
        for _i in 0..no_species {
            names.push(get_string(&mut stdin));
        }
        names
    }; 
    let (r, mut x) = {
        //TODO: Optimize
        let mut r_vec : Vec<f32> = Vec::with_capacity(no_species);
        let mut x_vec : Vec<f32> = Vec::with_capacity(no_species);
        for name in &species_names {
            println!("Enter the reproduction rate for {}", name);
            r_vec.push(get_f32(&mut stdin, &mut line));
            println!("Enter the carrying capacity for {}", name);
            //k_vec.push(get_f32(&mut stdin, &mut line));
            println!("Enter the initial population for {}", &name);
            x_vec.push(get_f32(&mut stdin, &mut line));
        }
        let r = Array1::from(r_vec);
        let x = Array1::from(x_vec);
        (r, x)
    };
    let (community_matrix, h, no_iters) = {
        let no_interactions = no_species * no_species;
        let mut interactions : Vec<f32> = Vec::with_capacity(no_interactions);
        println!("Enter the community matrix in row major order");
        for _i in 0..no_interactions {
            interactions.push(get_f32(&mut stdin, &mut line));
        }
        let matrix = Array2::from_shape_vec((no_species, no_species), interactions).unwrap();
        println!("Enter the timestep");
        let h =  get_f32(&mut stdin, &mut line);
        println!("Enter the duration of the simulation");
        let no_iters : usize = {
            let dur = get_f32(&mut stdin, &mut line);
            (dur / h) as usize
        }; 
        (matrix, h, no_iters)
    };
    let mut datapoints : Array2<f32> = Array::zeros((no_iters + 1, no_species));
    {
        let mut current_row = datapoints.slice_mut(s![0 as usize, ..]);
        current_row.assign(&x);
    }
    let end = (no_iters as f32) * h;
    let timevec = Array1::<f32>::linspace(0.0, end, no_iters + 1);
    let ones = Array1::<f32>::ones(no_species);
    let mut temp1 : Array1<f32> = Array1::zeros(no_species);
    let mut temp2 : Array1<f32> = Array1::zeros(no_species);
    //TODO: Optimize?
    for i in 0..no_iters {
        let row_index = i as usize;
        x.assign(&datapoints.slice(s![row_index, ..]));
        let mut current_row = datapoints.slice_mut(s![row_index + 1, ..]);
        //let x_new = &x + &(h * (((&r * &x) * &k_inv) * community_matrix.dot(&x)));
        let x_new = &x + &((&x * &r + &(community_matrix.dot(&x))) * h);
        current_row.assign(&x_new);
    }
    write!(&mut line, "time\t");
    write_table_line_species(&mut line, &species_names);
    println!("{}", &line);
    line.clear();
    for i in 0..(no_iters + 1) {
        write!(&mut line, "{:.4}\t", timevec[i]);
        write_row(&mut line, datapoints.slice(s![i, ..]));
        println!("{}", &line);
        line.clear();
    }
}