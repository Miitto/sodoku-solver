mod grid;

use grid::Grid;

fn main() {
    #[rustfmt::skip]
    let start = Some(vec![
        Some(4), Some(8), None, Some(6), None, None, Some(5), None, None,
        Some(5), None, Some(2), None, Some(9), None, Some(4), Some(6), None,
        None, Some(7), None, Some(4), Some(5), None, Some(2), None, None,
        Some(1), None, Some(5), Some(9), None, Some(4), None, Some(2), Some(6),
        None, Some(2), None, Some(5), None, None, Some(9), Some(4), None,
        None, Some(4), None, Some(2), None, None, Some(1), Some(5), Some(3),
        None, Some(6), Some(4), None, None, Some(5), None, Some(9), Some(2),
        None, None, None, None, Some(4), Some(2), Some(6), None, Some(5),
        Some(2), Some(5), None, Some(8), Some(6), Some(9), None, Some(1), Some(4),
    ]);

    let mut grid = Grid::new(&start);

    let start_time = std::time::Instant::now();

    grid.solve();

    let elapsed = start_time.elapsed();

    println!("{}", grid);
    println!(
        "Elapsed: {} {} {} {}",
        elapsed.as_secs(),
        elapsed.subsec_millis(),
        elapsed.subsec_micros(),
        elapsed.subsec_nanos()
    );
}
