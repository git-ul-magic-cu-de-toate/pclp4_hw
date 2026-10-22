struct CreaturaMaritima {
    pub nume: String,
    zgomot: String,
}

impl CreaturaMaritima {
    pub fn luare_zgomot(&self) -> &str {
        &self.zgomot
    }
}

trait FacatorDeZgomot {
    fn fa_zgomot(&self);
}

impl FacatorDeZgomot for CreaturaMaritima {
    fn fa_zgomot(&self) {
        println!("{}", &self.luare_zgomot());
    }
}

struct Ocean {
    animale: Vec<Box<dyn FacatorDeZgomot>>,
}

fn main() {
    let ferris = CreaturaMaritima {
        nume: String::from("Ferris"),
        zgomot: String::from("blub"),
    };
    let sarah = CreaturaMaritima {
        nume: String::from("Sarah"),
        zgomot: String::from("swish"),
    };
    let ocean = Ocean {
        animale: vec![Box::new(ferris), Box::new(sarah)],
    };
    for a in ocean.animale.iter() {
        a.fa_zgomot();
    }
}
