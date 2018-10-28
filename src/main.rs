extern crate clap;
extern crate rand;

use clap::App;
use rand::distributions::Bernoulli;
use rand::distributions::Distribution;

fn main() {
    let matches = App::new("Ruine du joueur")
                            .version("0.1")
                            .author("Vincent Perrollaz <vincent.perrollaz@laposte.net>")
                            .about("Calcul des probabilités de gains dans un jeu de pile ou face répété")
                            .args_from_usage(
                                "-p, --proba=[proba] 'Probabilité de gain du pile ou face'
                                 -n, --nb_parties=[nb_parties] 'Nombre de parties joués pour estimer une probabilité'
                                 -f, --f_max=[f_max] 'Fortune visée par le joueur pour s'arrêter'
                                 -m, --mode=[seq|par] 'Calculs sequentiels ou parallélisés'")
                            .get_matches();

    let proba: f64 = matches.value_of("proba").unwrap_or("0.5").parse().unwrap();
    let nb_parties: u32 = matches.value_of("nb_parties").unwrap_or("1000").parse().unwrap();
    let f_max: i16 = matches.value_of("f_max").unwrap_or("100").parse().unwrap();
    println!("Appel de la fonction avec 
    proba = {}
    nb_parties = {}
    f_max = {}", proba, nb_parties, f_max);
    
    // let resultats = parallele(100i16, 1000u32, proba);

    // affichage(&resultats);
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
