use std::{rc::Rc, time::Duration};

use par_stream::ParStreamExt;
use tokio::{
    task::{self, LocalSet},
    time::{self, Instant},
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    println!(
        "Task 1 (is_odd = false): {:?}",
        task1(5, false).collect::<Vec<u8>>()
    );
    println!(
        "Task 1 (is_odd = true): {:?}",
        task1(5, true).collect::<Vec<u8>>()
    );

    let vals: Vec<f64> = (0..10).map(f64::from).collect();
    let start_time = Instant::now();
    let result = task2(vals).await;
    let end_time = start_time.elapsed();
    println!("Task 2 took {end_time:?} to complete. Result: {result:?}");

    println!("Task 3: {}", task3().await);
}

/*
 * Напиши функцию, которая принимает на вход параметр n типа u8 и параметр is_odd типа bool и делает следующее:
 * если is_odd = false, то возвращает итератор, который производит все четные числа в возрастающем порядке в диапазоне от 0 до n включительно
 * если is_odd = true, то возвращает итератор, который производит все нечетные числа в убывающем порядке в том же диапазоне
 * Так же напиши тестирующую функцию для проверки
 */
fn task1(n: u8, is_odd: bool) -> impl Iterator<Item = u8> {
    (0..=n)
        .map(move |x| if is_odd { n - x } else { x })
        .filter(move |&x| (x % 2 == 0) ^ is_odd)
}

/*
 * Есть функция async fn f(a: f64) -> f64 { Box::pin(async { a + 1.2 }).await } и вектор values: Vec<f64>.
 * Функция f выполняется очень медленно, а вектор values может содержать очень много значений.
 * Напиши код, который вычисляет функцию f для каждого из значений values.
 * Нужно сделать упор на скорость выполнения.
 */
async fn task2(vals: Vec<f64>) -> Vec<f64> {
    tokio_stream::iter(vals)
        .par_then_unordered(None, f)
        .collect()
        .await
}

/*
 * Вызвать не Sync Future.
 */
async fn task3() -> Rc<f64> {
    LocalSet::new()
        .run_until(async move { task::spawn_local(f2(1.2)).await.unwrap() })
        .await
}

async fn f(a: f64) -> f64 {
    time::sleep(Duration::from_secs(1)).await;
    Box::pin(async { a + 1.2 }).await
}

async fn f2(a: f64) -> Rc<f64> {
    Box::pin(async { Rc::new(a + 1.2) }).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_even1() {
        assert_eq!(task1(5, false).collect::<Vec<u8>>(), vec![0, 2, 4])
    }

    #[test]
    fn task1_even2() {
        assert_eq!(task1(6, false).collect::<Vec<u8>>(), vec![0, 2, 4, 6])
    }

    #[test]
    fn task1_odd1() {
        assert_eq!(task1(5, true).collect::<Vec<u8>>(), vec![5, 3, 1])
    }

    #[test]
    fn task1_odd2() {
        assert_eq!(task1(6, true).collect::<Vec<u8>>(), vec![5, 3, 1])
    }
}
