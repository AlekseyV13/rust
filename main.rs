fn main() {
    println!("Hello, world!");
}


#[test]
fn romb() {
    const SIZE: usize = 5; // Задаємо розмір ромба
    let mut diamond = String::new();

    // Верхня частина ромба
    for i in 0..SIZE {
        diamond.push_str(&" ".repeat(SIZE - i - 1));
        diamond.push_str(&"* ".repeat(i + 1));
        diamond.push('\n');
    }

    // Нижня частина ромба
    for i in (0..SIZE - 1).rev() {
        diamond.push_str(&" ".repeat(SIZE - i - 1));
        diamond.push_str(&"* ".repeat(i + 1));
        diamond.push('\n');
    }

    print!("{}", diamond);
}

#[test]
fn romb1() {
    const SIZE: usize = 5;

    for i in 0..(2 * SIZE - 1) {
        let stars = if i < SIZE { i + 1 } else { 2 * SIZE - i - 1 };
        println!("{}{}", " ".repeat(SIZE - stars), "* ".repeat(stars));
    }
}
#[test]
fn konvert() {
    const WIDTH: usize = 11;
    const HEIGHT: usize = 11;
    let mut envelope = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == y || x + y == WIDTH - 1 || y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {
                envelope.push('*');
            } else {
                envelope.push(' ');
            }
            envelope.push(' ');
        }
        envelope.push('\n');
    }
    print!("{}", envelope);
}

#[test]
fn yalinka() {
    let triangles = 5; // Кількість трикутників (рівнів) ялинки

    let mut tree = String::new();
    (0..triangles).for_each(|level| {
        (0..=level + 2).for_each(|row| {
            let spaces = triangles + 2 - row;
            let stars = 2 * row + 1;
            tree.push_str(&" ".repeat(spaces));
            tree.push_str(&"*".repeat(stars));
            tree.push('\n');
        });
    });

    print!("{}", tree);
}