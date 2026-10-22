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

trait FacatorDeZgomotGalagios: FacatorDeZgomot {
    fn fa_mult_zgomot(&self) {
        self.fa_zgomot();
        self.fa_zgomot();
        self.fa_zgomot();
    }
}

impl FacatorDeZgomot for CreaturaMaritima {
    fn fa_zgomot(&self) {
        println!("{}", &self.luare_zgomot());
    }
}

impl FacatorDeZgomotGalagios for CreaturaMaritima {}

fn main() {
    let creatura = CreaturaMaritima {
        nume: String::from("Ferris"),
        zgomot: String::from("blub"),
    };
    creatura.fa_mult_zgomot();
}
