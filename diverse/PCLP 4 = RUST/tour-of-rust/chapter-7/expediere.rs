struct CreaturaMaritima {
    pub nume: String,
    zgomot: String,
}
Trăsăturile pot moșteni metode de la alte trăsături.
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

fn static_fa_zgomot(creatura: &CreaturaMaritima) {
    // cunoastem tipul real
    creatura.fa_zgomot();
}

fn dinamic_fa_zgomot(facator_de_zgomot: &dyn FacatorDeZgomot) {
    // nu cunoastem tipul real
