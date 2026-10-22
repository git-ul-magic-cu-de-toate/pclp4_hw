trait MyTrait {
    fn method1(&self);
    fn method2(&self);
    fn method3(&self);
}

struct MyStruct;

impl MyTrait for MyStruct {
    fn method1(&self) {
        println!("Implemented method1");
    }
}

impl MyTrait for MyStruct {
    fn method2(&self) {
        println!("Implemented method2");
    }
}

impl MyTrait for MyStruct {
    fn method3(&self) {
        println!("Implemented method3");
    }
}

fn main() {
    let my_instance = MyStruct;
    my_instance.method1();
    my_instance.method2();
    my_instance.method3();
}
