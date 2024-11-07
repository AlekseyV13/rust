use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    (0..n).map(|_| rand::thread_rng().gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32)> {
    data.windows(2).min_by_key(|w| w[0] + w[1]).map(|w| (w[0], w[1]))
}

fn print_vector(data: &[i32]) {
    data.chunks(10).for_each(|chunk| {
        println!("{}", chunk.iter().map(|x| format!("{:3}", x)).collect::<Vec<_>>().join(" "));
    });
}
#[test]
fn test11() {
    let random_vector = gen_random_vector(20);
    print_vector(&random_vector);

    if let Some((a, b)) = min_adjacent_sum(&random_vector) {
        println!("Мінімальна пара сусідніх чисел: ({}, {})", a, b);
    } else {
        println!("Вектор занадто малий для пошуку мінімальної пари.");
    }
}
