use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fake::{Fake, Faker};
use surrealdb_bench::{init_to_file_based, Large, Medium, Small, DB};

fn parse_id_into_resource(id: &str) -> (String, String) {
    let (tb, id) = id.split_once(':').unwrap();
    (tb.to_string(), id.to_string())
}

fn benchmark_small(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async { init_to_file_based().await });

    let mut size_group = c.benchmark_group("Small, Medium, to Large on File");

    let small: Small = Faker.fake();
    let medium: Medium = Faker.fake();
    let large: Large = Faker.fake();
    size_group.bench_with_input(
        BenchmarkId::new("Small with No Define Statements", "Sample Small"),
        &small,
        |b, small| {
            b.to_async(&rt).iter(|| async {
                let small: Small = Faker.fake();
                let id = small.id.as_ref().unwrap();
                let (tb, id) = parse_id_into_resource(id);
                let _: Option<Small> = DB.update((tb, id)).content(black_box(small)).await.unwrap();
            })
        },
    );

    size_group.bench_with_input(
        BenchmarkId::new("Medium with no Define Statements", "Sample Medium"),
        &medium,
        |b, medium| {
            b.to_async(&rt).iter(|| async {
                let medium: Medium = Faker.fake();
                let id = medium.id.as_ref().unwrap();
                let (tb, id) = parse_id_into_resource(id);
                let _: Option<Medium> = DB
                    .update((tb, id))
                    .content(black_box(medium))
                    .await
                    .unwrap();
            })
        },
    );
    size_group.bench_with_input(
        BenchmarkId::new("Large with no Define Statements", "Sample Large"),
        &large,
        |b, large| {
            b.to_async(&rt).iter(|| async {
                let large: Large = Faker.fake();
                let id = large.id.as_ref().unwrap();
                let (tb, id) = parse_id_into_resource(id);
                let _: Option<Large> = DB.update((tb, id)).content(black_box(large)).await.unwrap();
            })
        },
    );

    rt.block_on(async {
        let _ = Small::run_define_statements().await;
        let _ = Medium::run_define_statements().await;
        let _ = Large::run_define_statements().await;
    });

    size_group.bench_with_input(
        BenchmarkId::new("Small with Define Statements", "Sample Small"),
        &small,
        |b, small| {
            b.to_async(&rt).iter(|| async {
                let small: Small = Faker.fake();
                let id = small.id.as_ref().unwrap();
                let (tb, id) = parse_id_into_resource(id);
                let _: Option<Small> = DB.update((tb, id)).content(black_box(small)).await.unwrap();
            })
        },
    );

    size_group.bench_with_input(
        BenchmarkId::new("Medium with Define Statements", "Sample Medium"),
        &medium,
        |b, medium| {
            b.to_async(&rt).iter(|| async {
                let medium: Medium = Faker.fake();
                let id = medium.id.as_ref().unwrap();
                let (tb, id) = parse_id_into_resource(id);
                let _: Option<Medium> = DB
                    .update((tb, id))
                    .content(black_box(medium))
                    .await
                    .unwrap();
            })
        },
    );
    size_group.bench_with_input(
        BenchmarkId::new("Large with Define Statements", "Sample Large"),
        &large,
        |b, large| {
            b.to_async(&rt).iter(|| async {
                let large: Large = Faker.fake();
                let id = large.id.as_ref().unwrap();
                let (tb, id) = parse_id_into_resource(id);
                let _: Option<Large> = DB.update((tb, id)).content(black_box(large)).await.unwrap();
            })
        },
    );

    size_group.finish();
}

criterion_group!(benches, benchmark_small);
criterion_main!(benches);
