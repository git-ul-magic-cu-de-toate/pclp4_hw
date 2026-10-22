struct CreaturaMaritima {
    pub nume: String,
    zgomot: String,
}

impl CreaturaMaritima {
    pub fn luare_zgomot(&self) -> &str {
        &self.zgomot
    }

    pub fn setare_zgomot(&mut self, zgomot: &str) {
        self.zgomot = String::from(zgomot);
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

fn main() {
    let mut creatura = CreaturaMaritima {
        nume: String::from("Ferris"),
        zgomot: String::from("blub"),
    };

    creatura.fa_zgomot();

    let mut new_zgomot = String::from("bau");
    creatura.setare_zgomot(new_zgomot.as_str());    
    creatura.fa_zgomot();

    creatura.nume = String::from("fish");
    creatura.zgomot = String::from("...");
    creatura.fa_zgomot();


    return ();
}
