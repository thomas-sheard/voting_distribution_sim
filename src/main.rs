//use rand::Rng;
//use std::io;

//fn new_voter(dist: &[f32; usize]) -> [u8; 3] {
//    let a: [u8; 3] = [1, 2, 3]
//}

fn main() {

    // define voter preference distribution for candidates
    // all values should add to 1 to be a valid distribution
    // (but this isn't strictly necessary; <1 may represent abstaining voters)

    // TODO: add support for optional cli input and an arbitrary number of candidates
    //let dist = [0.5, 0.3, 0.2];

    // TODO: add support for optional cli input that defaults to a reasonable value
    const DEFAULT_SAMPLE_SIZE: usize = 10;

    // check for valid distribution 
    // (not strictly necessary)
//    if candidate_a + candidate_b + candidate_c != 1.0 {
//        panic!("Invalid voter distribution (does not sum to 1)");
//    }

    let mut population = [[0u8; 3]; DEFAULT_SAMPLE_SIZE];

    for i in 0..DEFAULT_SAMPLE_SIZE {
        //let voter = new_voter(&dist);
        let voter: [u8; 3] = [1, 2, 3];
        population[i] = voter;
        println!("{}: {:#?}{:#?}{:#?}", i, population[0], population[1], population[2]);
    }


}
