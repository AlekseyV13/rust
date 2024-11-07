use rand::Rng;

// 1. Функція, яка генерує вектор із значеннями, що можуть бути рівномірно розподілені
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let target: u32 = rng.gen_range(10..100); // Базове значення для вантажу на кожному кораблі
    let mut shipments = vec![target; n]; // Генеруємо вектор із рівномірним вантажем

    // Рандомно змінюємо вантаж на деяких кораблях, щоб зберегти рівномірний розподіл
    for i in 0..n {
        if rng.gen_bool(0.5) {
            shipments[i] += 1;
            shipments[(i + 1) % n] -= 1;
        }
    }

    shipments
}

// 2. Функція, яка рахує мінімальну кількість перенесень для вирівнювання вантажу
fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    // Перевіряємо, чи можна розподілити вантаж рівномірно
    if total % n != 0 {
        return None;
    }

    let target = total / n;
    let mut moves = 0;

    // Рахуємо кількість переміщень для досягнення однакового вантажу на всіх кораблях
    for &shipment in shipments {
        if shipment > target {
            moves += (shipment - target) as usize;
        }
    }

    Some(moves)
}

// 3. Функція, яка виводить вектор у зрозумілому вигляді
fn print_vector(data: &[u32]) {
    println!("Вектор:");
    data.chunks(10).for_each(|chunk| {
        println!("{}", chunk.iter().map(|x| format!("{:3}", x)).collect::<Vec<_>>().join(" "));
    });
    println!();
}
#[test]
fn test13() {

    let shipments = gen_shipments(20);

    print_vector(&shipments);

    match count_permutation(&shipments) {
        Some(moves) => println!("Мінімальна кількість перенесень: {}", moves),
        None => println!("Неможливо вирівняти вантажі між кораблями."),
    }
}
