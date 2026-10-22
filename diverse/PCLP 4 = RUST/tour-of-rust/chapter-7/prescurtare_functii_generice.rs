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

fn generic_fa_zgomot(creatura: &impl FacatorDeZgomot)
{
    // cunoastem tipul real in momentul compilarii
    creatura.fa_zgomot();
}

fn main() {
    let creatura = CreaturaMaritima {
        nume: String::from("Ferris"),
        zgomot: String::from("blub"),
    };
    generic_fa_zgomot(&creatura);
}

