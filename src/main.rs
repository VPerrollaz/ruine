extern crate rand;

use rand::distributions::Bernoulli;
use rand::distributions::Distribution;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let proba: f64;
    if args.len()  >= 2 {
         match args[1].parse() {
             Ok(nombre) => proba = nombre,
             _ => proba = 0.5f64,
         }
    }
    else {
         proba = 0.5f64;
    }

    let mut children = vec![];
    for depart in 0..4 {
        children.push(std::thread::spawn(move || { let mut resultat = vec![];
            for i in 0..25 {
                resultat.push(estimation_proba(40000u32, (i+25*depart) as i8, proba));
            }
            resultat
        }));
    }

    let mut resultats = [0.0f64; 100];
    for (i, child) in children.into_iter().enumerate() {
        for (j, val) in child.join().unwrap().iter().enumerate() {
            resultats[25*i+j] = *val;
        }
    }
    affichage(resultats);
}

fn affichage(tableau: [f64; 100]) {
    for val in tableau.iter() {
        print!("{}, ", val);
    }
}

fn une_partie(x: i8, d: &Bernoulli, gen: &mut rand::ThreadRng) -> bool {
    let mut etat = x;
    while (0 < etat) & (etat < 100) {
        etat += 2 * (d.sample(gen) as i8) - 1;
    }
        etat==100
}

fn estimation_proba(nb_parties: u32, x: i8, proba: f64) -> f64 {
    let d = Bernoulli::new(proba);
    let mut gen = rand::thread_rng();
    let mut nb_victoires = 0u16;
    for _ in 0..nb_parties {
        if une_partie(x, &d, &mut gen) {
            nb_victoires += 1;
        }
    }
    (nb_victoires as f64) / (nb_parties as f64)
}
