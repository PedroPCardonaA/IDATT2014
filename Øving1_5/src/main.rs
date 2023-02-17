extern crate random_number;
use std::convert::TryInto;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;
use random_number::random;

fn main() {
    let size = 1000;
    let no_threads = 10;
    let matrix1: Vec<Vec<i32>> = generate_matrix(size);

    let matrix2: Vec<Vec<i32>> = generate_matrix(size);

    let m1 = matrix1.clone();
    let m2 = matrix2.clone();

    println!("Without parallelism");
    multiple_matrix(matrix1, matrix2);
    println!("With parallelism");
    no_locks(m1, m2,no_threads);

}

fn multiple_matrix_threads_bad(a:Vec<Vec<i32>>, b:Vec<Vec<i32>>, no:i32){
    let start = Instant::now();
    if a[0].len() != b.len() {
        println!(" The number of columns in the 1st matrix equals is not the number of rows in the 2nd matrix");
        return;
    }

    
    let mat:Vec<Vec<i32>> = vec![vec![0;a.len()];b[0].len()];
    let a_lock = Arc::new(RwLock::new(a));
    let b_lock = Arc::new(RwLock::new(b));
    let result = Arc::new(RwLock::new(mat));
    let mut threads = Vec::new();
    let copy_a = a_lock.clone();
    let copy_b = b_lock.clone();
    let copy_result = result.clone();
    
    
    for x in 0..a_lock.read().unwrap().len(){
        let copy_a = copy_a.clone();
        let copy_b = copy_b.clone();
        let copy_result = copy_result.clone();


        threads.push(thread::spawn(move||{
            let copy_a = copy_a.clone();
            let copy_b = copy_b.clone();
            let copy_result = copy_result.clone();


            for i in 0..copy_b.read().unwrap().len(){
                let mut sum = 0;
                let copy_a = copy_a.clone();
                let copy_b = copy_b.clone();

                for j in 0..copy_b.read().unwrap()[0].len(){
                    sum += copy_a.read().unwrap()[x][i]*copy_b.read().unwrap()[i][j];
                }

            copy_result.write().unwrap()[x][i] = sum;
            }
        }));
    }

    for t in threads{
        let _ = t.join();
    }

    // for row in result.read().unwrap().iter() {
    //     for elem in row {
    //         print!("{} ", elem);
    //     }
    //     println!();   
    // }

    let duration = start.elapsed();
    println!("Duration: {:?} ms", duration.as_millis());

}

fn multiple_matrix(a:Vec<Vec<i32>>, b:Vec<Vec<i32>>){

    let start = Instant::now();

    if a[0].len() != b.len() {
        println!(" The number of columns in the 1st matrix equals is not the number of rows in the 2nd matrix");
        return;
    }

    
    let mut mat:Vec<Vec<i32>> = vec![vec![0;a.len()];b[0].len()];

    for x in 0..a.len(){
        for i in 0..b.len(){
            let mut sum = 0;
            for j in 0..b[i].len(){
                sum +=a[x][i]*b[i][j];
            }
            mat[x][i] = sum;
        }
    }
    // for row in mat {
    //     for elem in row {
    //         print!("{} ", elem);
    //     }
    //     println!();
    // }

    let duration = start.elapsed();
    println!("Duration: {:?} ms", duration.as_millis());

}

fn generate_matrix(size: i32) ->Vec<Vec<i32>>{

    let a = size.try_into().unwrap();
    // Create a vector of vectors (matrix)
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; a]; a];

    // Fill the matrix with random numbers between 0 and 1
    for i in 0..a {
        for j in 0..a {
            matrix[i][j] = random!(0,100);
        }
    }

    return matrix;
}

fn multiple_matrix_threads(a:Vec<Vec<i32>>, b:Vec<Vec<i32>>, no:i32){
    let start = Instant::now();
    if a[0].len() != b.len() {
        println!(" The number of columns in the 1st matrix equals is not the number of rows in the 2nd matrix");
        return;
    }

    let size =a.len();
    let num:i32 = (size/no as usize).try_into().unwrap();
    let mat:Vec<Vec<i32>> = vec![vec![0;a.len()];b[0].len()];
    let a_lock = Arc::new(RwLock::new(a));
    let b_lock = Arc::new(RwLock::new(b));
    let result = Arc::new(RwLock::new(mat));
    let mut threads = Vec::new();
    let copy_a = a_lock.clone();
    let copy_b = b_lock.clone();
    let copy_result = result.clone();
    
    
    for x in 0..no{
        let copy_a = copy_a.clone();
        let copy_b = copy_b.clone();
        let copy_result = copy_result.clone();


        threads.push(thread::spawn(move||{
            let copy_a = copy_a.clone();
            let copy_b = copy_b.clone();
            let copy_result = copy_result.clone();
            

            for l in (num * x)..(num + num * x){

                let copy_a = copy_a.clone();
                let copy_b = copy_b.clone();
                let copy_result = copy_result.clone();
                for i in 0..copy_b.read().unwrap().len(){
                    let mut sum = 0;
                    let copy_a = copy_a.clone();
                    let copy_b = copy_b.clone();
    
                    for j in 0..copy_b.read().unwrap()[0].len(){
                        sum += copy_a.read().unwrap()[l as usize][i]*copy_b.read().unwrap()[i][j];
                    }
    
                copy_result.write().unwrap()[l as usize][i] = sum;
                }
            }
        }));
    }

    for t in threads{
        let _ = t.join();
    }

    // for row in result.read().unwrap().iter() {
    //     for elem in row {
    //         print!("{} ", elem);
    //     }
    //     println!();   
    // }

    let duration = start.elapsed();
    println!("Duration: {:?} ms", duration.as_millis());

}

fn no_locks(a:Vec<Vec<i32>>, b:Vec<Vec<i32>>, no:i32){
    let start = Instant::now();
    if a[0].len() != b.len() {
        println!(" The number of columns in the 1st matrix equals is not the number of rows in the 2nd matrix");
        return;
    }

    
    let mat:Vec<Vec<i32>> = vec![vec![0;a.len()];b[0].len()];
    let mut threads = Vec::new();
    // let copy_a = a.clone();
    // let copy_b = b.clone();
    // let mut copy_result = mat.clone();
    
    
    for x in 0..a.len(){
        let copy_a = a.clone();
        let copy_b = b.clone();
        let mut copy_result = mat.clone();


        threads.push(thread::spawn(move||{
            // let copy_a = copy_a.clone();
            // let copy_b = copy_b.clone();
            // let mut copy_result = copy_result.clone();


            for i in 0..copy_b.len(){
                let mut sum = 0;
                // let copy_a = copy_a.clone();
                // let copy_b = copy_b.clone();

                for j in 0..copy_b[0].len(){
                    sum += copy_a[x][i]*copy_b[i][j];
                }

            copy_result[x][i] = sum;
            }
        }));
    }

    for t in threads{
        let _ = t.join();
    }

    // for row in result.read().unwrap().iter() {
    //     for elem in row {
    //         print!("{} ", elem);
    //     }
    //     println!();   
    // }

    let duration = start.elapsed();
    println!("Duration: {:?} ms", duration.as_millis());
}