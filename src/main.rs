#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate rand;
extern crate rayon;
extern crate serde;
extern crate serde_json;

use clap::App;
use rand::distributions::Bernoulli;
use rand::distributions::Distribution;
use rayon::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Resultat {
    f_max : i16,
    nb_parties : u32,
    proba : f64,
    seq : bool,
    valeurs:  Vec<f64>
}

impl Resultat {
    fn new(f_max: i16, nb_parties: u32, proba: f64, seq: bool, valeurs: Vec<f64>) -> Resultat {
        Resultat {
            f_max : f_max,
            nb_parties : nb_parties,
            proba : proba,
            seq : seq,
            valeurs : valeurs
        }
    }
}

fn main() {
    let matches = App::new("Ruine du joueur")
                            .version("0.1")
                            .author("Vincent Perrollaz <vincent.perrollaz@laposte.net>")
                            .about("Calcul des probabilités de gains dans un jeu de pile ou face répétés")
                            .args_from_usage(
                                "-p, --proba=[proba] 'Probabilité de gain du pile ou face'
                                 -n, --nb_parties=[nb_parties] 'Nombre de parties jouées pour estimer une probabilité'
                                 -f, --f_max=[f_max] 'Fortune visée par le joueur pour s'arrêter'
                                 -s, --seq 'Calculs sequentiels, par défaut en parallèle'")
                            .get_matches();

    let proba: f64 = matches.value_of("proba").unwrap_or("0.5").parse().unwrap();
    let nb_parties: u32 = matches.value_of("nb_parties").unwrap_or("100").parse().unwrap();
    let f_max: i16 = matches.value_of("f_max").unwrap_or("10").parse().unwrap();

    if matches.is_present("seq") {
        let res = Resultat::new(f_max, nb_parties, proba, true, sequentiel(f_max, nb_parties, proba));
        println!("{}", serde_json::to_string(&res).unwrap());
    }
    else {
        let res = Resultat::new(f_max, nb_parties, proba, false, parallele(f_max, nb_parties, proba));
        println!("{}", serde_json::to_string(&res).unwrap());
    }
}

fn sequentiel(f_max: i16, nb_parties: u32, proba: f64) -> Vec<f64> {
    (1..f_max as usize).into_iter()
              .map( |f_ini| {estimation_proba(f_ini as i16, f_max, nb_parties, proba)} )
              .collect()
}

fn parallele(f_max: i16, nb_parties: u32, proba: f64) -> Vec<f64> {
    (1..f_max as usize).into_par_iter()
              .map( |f_ini| {estimation_proba(f_ini as i16, f_max, nb_parties, proba)} )
              .collect()
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
    let mut nb_victoires = 0u32;
    for _ in 0..nb_parties {
        if une_partie(f_ini, f_max, &d, &mut gen) {
            nb_victoires += 1;
        }
    }
    (nb_victoires as f64) / (nb_parties as f64)
}
