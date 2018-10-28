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

    let resultats = parallele(100i16, 1000u32, proba);

    affichage(&resultats);
}

fn affichage(tableau: &[f64]) {
    for val in tableau.iter() {
        print!("{}, ", val);
    }
}

fn parallele(f_max: i16, nb_parties: u32, proba: f64) -> Vec<f64> {
    let mut children = vec![];
    for depart in 0..4 {
        children.push(std::thread::spawn(move || { let mut resultat = vec![];
            for i in 0..25 {
                resultat.push(estimation_proba((i+25*depart) as i16,
                                               f_max,
                                               nb_parties,
                                               proba));
            }
            resultat
        }));
    }

    let mut resultats = vec![0.0f64; f_max as usize];
    for (i, child) in children.into_iter().enumerate() {
        for (j, val) in child.join().unwrap().iter().enumerate() {
            resultats[25*i+j] = *val;
        }
    }

    resultats
}

fn une_partie(f_ini: i16, f_max:i16, d: &Bernoulli, gen: &mut rand::ThreadRng) -> bool {
    let mut etat = f_ini;
    while (0 < etat) & (etat < f_max) {
        etat += 2 * (d.sample(gen) as i16) - 1;
    }
        etat==f_max
}

fn estimation_proba(f_ini: i16, f_max: i16, nb_parties: u32, proba: f64) -> f64 {
    let d = Bernoulli::new(proba);
    let mut gen = rand::thread_rng();
    let mut nb_victoires = 0u16;
    for _ in 0..nb_parties {
        if une_partie(f_ini, f_max, &d, &mut gen) {
            nb_victoires += 1;
        }
    }
    (nb_victoires as f64) / (nb_parties as f64)
}
