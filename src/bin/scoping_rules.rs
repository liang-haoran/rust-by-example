fn main() {
    println!("{:=^56}", " main ");

    raii();

    ownership_and_moves();
}

fn raii() {
    println!("{:=^56}", " raii ");

    // RAII：有一种读法叫资源获取即初始化，但是感觉反过来讲更好记：（变量）初始化即完成资源的获取。
    // 大意是，资源的有效期和与资源绑定的变量生命周期一致，即变量初始化的时候即完成了资源的获取，变量离开作用域调用析构时完成资源的释放
    // 好处是，只要正确的析构，就不会出现资源泄露。

}

fn ownership_and_moves() {
    println!("{:=^56}", " ownership and moves ");

    // 栈上分配资源的例子
    let x = 5i32;

    // i32 实现了 copy，所以没有发生 move
    let y = x;

    println!("{}, {}", x, y);

    // 堆上分配资源的例子
    let a = Box::new(1);

    // Box 没有实现 copy，所以发生了 move
    let b = a;

    // 发生了 move，原变量不能再访问
    //println!("moved {}", a);

    mutability();

    partical_moves();

    fn mutability() {
        println!("{:=^56}", " mutability ");

        // 😮数据，注意是数据的可变性可以通过 move 来改变
        let immutable_box = Box::new(0);

        let mut mutable_box = immutable_box;

        *mutable_box = 1;
        println!("box: {}", mutable_box);
    }

    fn partical_moves() {
        // 除了 tuple、array 之外，struct 竟然也可以部分引用
        // tuple、array 在部分引用之后，会导致整体失去 W、O 权限，关键是引用
        // struct 解构的时候也可以部分引用，同时应用所有权和借用规则，会导致整体失去 R、W、O 权限
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }

        let person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };

        let Person { name, ref age } = person;

        // 按照所有权规则，move 之后不能在访问
        //println!("{:?}", person);

        println!("{:?}", person.age);

        let mut foo = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };

        fn what(person: &Person) {}

        let Person { ref mut age, ..} = foo;
        **age = 2;
        println!("{:?}", foo.name);

        // 只要编译器可以根据声明来确定具体的元素，就可以细化借用规则
        // 如果 tuple 有2个或以上的元素类型是TypeB，然后返回&TypeB，编译器（不会去看函数的具体实现）就无法确定到底借用了哪个元素了
        fn get_first(name: &(Box<i32>, String)) -> &Box<i32> {
            &name.0
        }

        let mut name = (
            Box::new(1),
            String::from("bar")
        );

        let bos = get_first(&name);

        //let first = &name.0;

        // 读第一遍的时候，以为这种写法也不行
        name.1.push_str("what");

        println!("{:?}", name);
    }
}
