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
    
    fn fa_mult_zgomot(&self){
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


struct Numbers {
    x: i32,
    y: i32,
}

trait Operatii {
    fn add(&self) -> i32;
    fn dec(&self) -> i32;
}


impl Operatii for Numbers {
    fn add(&self) -> i32 {
        return self.x + self.y;
    }

    fn dec(&self) -> i32 {
        return self.x - self.y;
    }
}

fn main() {
    let creatura = CreaturaMaritima {
        nume: String::from("Ferris"),
        zgomot: String::from("blub"),
    };
    creatura.fa_mult_zgomot();


    let nr = Numbers {x: 5, y: 10};
    println!("{} + {} = {}", nr.x, nr.y, nr.add());
}

