struct CreaturaMaritima {
    pub nume: String,
    zgomot: String,
}

impl CreaturaMaritima {
    pub fn luare_sunet(&self) -> &str {
        // &self -> imrpumut structura, dar nu o s-o modific
        return &self.zgomot;
    }

    pub fn setare_sunter(&mut self, zgomot: &str) {
        self.zgomot = String::from(zgomot);
    }
}

fn main() {
    let mut creatura = CreaturaMaritima {
        nume: String::from("Ferris"),
        zgomot: String::from("blub"),
    };
    // ca sa putem seta campurile structurile (le modificam), structura trebuie sa fie mutable

    println!("{}", creatura.luare_sunet());

    let zgomot2 = String::from("gugugaga");
    creatura.setare_sunter(zgomot2.as_str());
    println!("{}", creatura.luare_sunet());
}
