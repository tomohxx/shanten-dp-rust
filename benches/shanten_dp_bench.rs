use std::fs::File;
use std::hint::black_box;
use std::io::{BufRead, BufReader};
use std::path::Path;

use criterion::{Criterion, criterion_group, criterion_main};
use shanten_dp::make_tile_limits;

type Dataset = Vec<([u8; 34], i8, i8, i8)>;

fn read(path: &Path) -> Dataset {
    let reader = BufReader::new(File::open(path).unwrap());
    let mut dataset: Dataset = Vec::with_capacity(10000);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_ascii_whitespace();
        let mut hand = [0u8; 34];

        for _ in 0..14 {
            let tid = iter.next().unwrap().parse::<usize>().unwrap();
            hand[tid] += 1;
        }

        let standard = iter.next().unwrap().parse::<i8>().unwrap();
        let thirteen_orphans = iter.next().unwrap().parse::<i8>().unwrap();
        let seven_pairs = iter.next().unwrap().parse::<i8>().unwrap();

        dataset.push((hand, standard, seven_pairs, thirteen_orphans));
    }

    dataset
}

fn verify(dataset: &Dataset) {
    let tile_limits = make_tile_limits(false);

    for (hand, standard_shanten, seven_pairs_shanten, thirteen_orphans_shanten) in dataset {
        assert_eq!(
            shanten_dp::calc_shanten::<i8>(hand, &tile_limits, 4, 1, false).unwrap().unwrap(),
            *standard_shanten,
            "standard::calc_shanten validation failed"
        );
        assert_eq!(
            shanten_dp::calc_shanten::<i8>(hand, &tile_limits, 4, 2, false).unwrap().unwrap(),
            *seven_pairs_shanten,
            "seven_pairs::calc_shanten validation failed"
        );
        assert_eq!(
            shanten_dp::calc_shanten::<i8>(hand, &tile_limits, 4, 4, false).unwrap().unwrap(),
            *thirteen_orphans_shanten,
            "thirteen_orphans::calc_shanten validation failed"
        );
    }
}

fn load_datasets() -> Vec<(&'static str, Dataset)> {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("benches/data");

    let datasets = vec![
        ("normal", read(&data_dir.join("p_normal_10000.txt"))),
        ("tinitu", read(&data_dir.join("p_tin_10000.txt"))),
        ("honitu", read(&data_dir.join("p_hon_10000.txt"))),
        ("kokusi", read(&data_dir.join("p_koku_10000.txt"))),
    ];

    for (_, dataset) in &datasets {
        verify(&dataset);
    }

    datasets
}

fn bm_calc_shanten(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_shanten");

    group.sample_size(10);

    let tile_limits = make_tile_limits(false);
    let datasets = load_datasets();

    for (name, dataset) in &datasets {
        group.bench_with_input(*name, dataset, |b, dataset| {
            b.iter(|| {
                for (hand, _, _, _) in dataset {
                    black_box(
                        shanten_dp::calc_shanten::<i8>(hand, &tile_limits, 4, 1, false).unwrap(),
                    );
                    black_box(
                        shanten_dp::calc_shanten::<i8>(hand, &tile_limits, 4, 2, false).unwrap(),
                    );
                    black_box(
                        shanten_dp::calc_shanten::<i8>(hand, &tile_limits, 4, 4, false).unwrap(),
                    );
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bm_calc_shanten);
criterion_main!(benches);
