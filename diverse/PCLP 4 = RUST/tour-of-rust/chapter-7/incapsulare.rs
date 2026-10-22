struct CreaturaMaritima {
    zgomot: String,
}

impl CreaturaMaritima {
    fn luare_zgomot(&self) -> &str {
        &self.zgomot
    }
}

fn main() {
    let creatura = CreaturaMaritima {
        zgomot: String::from("blub"),
    };
    println!("{}", creatura.luare_zgomot());
}

