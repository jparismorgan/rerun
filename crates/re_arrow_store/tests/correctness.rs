//! Correctness tests.
//!
//! Bending and twisting the datastore APIs in all kinds of weird ways to try and break them.

use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

use arrow2::array::{Array, UInt64Array};

use polars_core::{
    prelude::{DataFrame, JoinType},
    series::Series,
};
use re_arrow_store::{
    polars_util, test_bundle, DataStore, LatestAtQuery, RangeQuery, TimeRange, WriteError,
};
use re_log_types::{
    datagen::{
        build_frame_nr, build_log_time, build_some_instances, build_some_point2d, build_some_rects,
    },
    external::arrow2_convert::serialize::TryIntoArrow,
    field_types::{Instance, Point2D, Rect2D},
    msg_bundle::{wrap_in_listarray, Component as _, ComponentBundle},
    Duration, ObjPath as EntityPath, Time, TimeType, Timeline,
};

// ---

#[test]
fn write_errors() {
    init_logs();

    let ent_path = EntityPath::from("this/that");

    {
        use arrow2::compute::concatenate::concatenate;

        let mut store = DataStore::new(Instance::name(), Default::default());
        let mut bundle = test_bundle!(ent_path @
            [build_frame_nr(32.into()), build_log_time(Time::now())] => [
                build_some_instances(10), build_some_point2d(10)
        ]);

        // make instances 2 rows long
        bundle.components[0].value =
            concatenate(&[&*bundle.components[0].value, &*bundle.components[0].value]).unwrap();

        // The first component of the bundle determines the number of rows for all other
        // components in there (since it has to match for all of them), so in this case we get a
        // `MoreThanOneRow` error as the first component is > 1 row.
        assert!(matches!(
            store.insert(&bundle),
            Err(WriteError::MoreThanOneRow(_)),
        ));
    }

    {
        use arrow2::compute::concatenate::concatenate;

        let mut store = DataStore::new(Instance::name(), Default::default());
        let mut bundle = test_bundle!(ent_path @
            [build_frame_nr(32.into()), build_log_time(Time::now())] => [
                build_some_instances(10), build_some_point2d(10)
        ]);

        // make component 2 rows long
        bundle.components[1].value =
            concatenate(&[&*bundle.components[1].value, &*bundle.components[1].value]).unwrap();

        // The first component of the bundle determines the number of rows for all other
        // components in there (since it has to match for all of them), so in this case we get a
        // `MismatchedRows` error as the first component is 1 row but the 2nd isn't.
        assert!(matches!(
            store.insert(&bundle),
            Err(WriteError::MismatchedRows(_)),
        ));
    }

    {
        pub fn build_sparse_instances() -> ComponentBundle {
            let ids = wrap_in_listarray(UInt64Array::from(vec![Some(1), None, Some(3)]).boxed());
            ComponentBundle {
                name: Instance::name(),
                value: ids.boxed(),
            }
        }

        let mut store = DataStore::new(Instance::name(), Default::default());
        let bundle = test_bundle!(ent_path @
            [build_frame_nr(32.into()), build_log_time(Time::now())] => [
                build_sparse_instances(), build_some_point2d(3)
        ]);
        assert!(matches!(
            store.insert(&bundle),
            Err(WriteError::SparseClusteringComponent(_)),
        ));
    }

    {
        pub fn build_unsorted_instances() -> ComponentBundle {
            let ids = wrap_in_listarray(UInt64Array::from_vec(vec![1, 3, 2]).boxed());
            ComponentBundle {
                name: Instance::name(),
                value: ids.boxed(),
            }
        }
        pub fn build_duped_instances() -> ComponentBundle {
            let ids = wrap_in_listarray(UInt64Array::from_vec(vec![1, 2, 2]).boxed());
            ComponentBundle {
                name: Instance::name(),
                value: ids.boxed(),
            }
        }

        let mut store = DataStore::new(Instance::name(), Default::default());
        {
            let bundle = test_bundle!(ent_path @
                [build_frame_nr(32.into()), build_log_time(Time::now())] => [
                    build_unsorted_instances(), build_some_point2d(3)
            ]);
            assert!(matches!(
                store.insert(&bundle),
                Err(WriteError::InvalidClusteringComponent(_)),
            ));
        }
        {
            let bundle = test_bundle!(ent_path @
                [build_frame_nr(32.into()), build_log_time(Time::now())] => [
                    build_duped_instances(), build_some_point2d(3)
            ]);
            assert!(matches!(
                store.insert(&bundle),
                Err(WriteError::InvalidClusteringComponent(_)),
            ));
        }
    }

    {
        let mut store = DataStore::new(Instance::name(), Default::default());
        let bundle = test_bundle!(ent_path @
            [build_frame_nr(32.into()), build_log_time(Time::now())] => [
                build_some_instances(4), build_some_point2d(3)
        ]);
        assert!(matches!(
            store.insert(&bundle),
            Err(WriteError::MismatchedInstances { .. }),
        ));
    }
}

// ---

#[test]
fn latest_at_emptiness_edge_cases() {
    init_logs();

    for config in re_arrow_store::test_util::all_configs() {
        let mut store = DataStore::new(Instance::name(), config.clone());
        latest_at_emptiness_edge_cases_impl(&mut store);
    }
}
fn latest_at_emptiness_edge_cases_impl(store: &mut DataStore) {
    let ent_path = EntityPath::from("this/that");
    let now = Time::now();
    let now_minus_1s = now - Duration::from_secs(1.0);
    let now_minus_1s_nanos = now_minus_1s.nanos_since_epoch().into();
    let frame39 = 39.into();
    let frame40 = 40.into();
    let nb_instances = 3;

    store
        .insert(
            &test_bundle!(ent_path @ [build_log_time(now), build_frame_nr(frame40)] => [
                build_some_instances(nb_instances),
            ]),
        )
        .unwrap();

    if let err @ Err(_) = store.sanity_check() {
        store.sort_indices_if_needed();
        eprintln!("{store}");
        err.unwrap();
    }

    let timeline_wrong_name = Timeline::new("lag_time", TimeType::Time);
    let timeline_wrong_kind = Timeline::new("log_time", TimeType::Sequence);
    let timeline_frame_nr = Timeline::new("frame_nr", TimeType::Sequence);
    let timeline_log_time = Timeline::new("log_time", TimeType::Time);

    // empty frame_nr
    {
        let row_indices = store.latest_at(
            &LatestAtQuery::new(timeline_frame_nr, frame39),
            &ent_path,
            Instance::name(),
            &[Instance::name()],
        );
        assert!(row_indices.is_none());
    }

    // empty log_time
    {
        let row_indices = store.latest_at(
            &LatestAtQuery::new(timeline_log_time, now_minus_1s_nanos),
            &ent_path,
            Instance::name(),
            &[Instance::name()],
        );
        assert!(row_indices.is_none());
    }

    // wrong entity path
    {
        let row_indices = store.latest_at(
            &LatestAtQuery::new(timeline_frame_nr, frame40),
            &EntityPath::from("does/not/exist"),
            Instance::name(),
            &[Instance::name()],
        );
        assert!(row_indices.is_none());
    }

    // bunch of non-existing components
    {
        let components = &["they".into(), "dont".into(), "exist".into()];
        let row_indices = store
            .latest_at(
                &LatestAtQuery::new(timeline_frame_nr, frame40),
                &ent_path,
                Instance::name(),
                components,
            )
            .unwrap();
        let rows = store.get(components, &row_indices);
        rows.iter().all(|row| row.is_none());
    }

    // empty component list
    {
        let row_indices = store
            .latest_at(
                &LatestAtQuery::new(timeline_frame_nr, frame40),
                &ent_path,
                Instance::name(),
                &[],
            )
            .unwrap();
        assert!(row_indices.is_empty());
    }

    // wrong timeline name
    {
        let row_indices = store.latest_at(
            &LatestAtQuery::new(timeline_wrong_name, frame40),
            &EntityPath::from("does/not/exist"),
            Instance::name(),
            &[Instance::name()],
        );
        assert!(row_indices.is_none());
    }

    // wrong timeline kind
    {
        let row_indices = store.latest_at(
            &LatestAtQuery::new(timeline_wrong_kind, frame40),
            &EntityPath::from("does/not/exist"),
            Instance::name(),
            &[Instance::name()],
        );
        assert!(row_indices.is_none());
    }
}

// ---

// This one demonstrates a nasty edge case when stream-joining multiple iterators that happen to
// share the same exact row of data at some point (because, for that specific entry, it turns out
// that those component where inserted together).
//
// When that happens, one must be very careful to not only compare time and index row numbers, but
// also make sure that, if all else if equal, the primary iterator comes last so that it gathers as
// much state as possible!

#[test]
fn range_join_across_single_row() {
    init_logs();

    for config in re_arrow_store::test_util::all_configs() {
        let mut store = DataStore::new(Instance::name(), config.clone());
        range_join_across_single_row_impl(&mut store);
    }
}
fn range_join_across_single_row_impl(store: &mut DataStore) {
    let ent_path = EntityPath::from("this/that");

    let points = build_some_point2d(3);
    let rects = build_some_rects(3);
    let bundle =
        test_bundle!(ent_path @ [build_frame_nr(42.into())] => [points.clone(), rects.clone()]);
    store.insert(&bundle).unwrap();

    let timeline_frame_nr = Timeline::new("frame_nr", TimeType::Sequence);
    let query = RangeQuery::new(
        timeline_frame_nr,
        TimeRange::new(i64::MIN.into(), i64::MAX.into()),
    );
    let components = [Instance::name(), Point2D::name(), Rect2D::name()];
    let dfs = polars_util::range_components(
        store,
        &query,
        &ent_path,
        Point2D::name(),
        components,
        &JoinType::Outer,
    )
    .collect::<Vec<_>>();

    let df_expected = {
        let instances: Box<dyn Array> = vec![Instance(0), Instance(1), Instance(2)]
            .try_into_arrow()
            .unwrap();
        let points: Box<dyn Array> = points.try_into_arrow().unwrap();
        let rects: Box<dyn Array> = rects.try_into_arrow().unwrap();

        DataFrame::new(vec![
            Series::try_from((Instance::name().as_str(), instances)).unwrap(),
            Series::try_from((Point2D::name().as_str(), points)).unwrap(),
            Series::try_from((Rect2D::name().as_str(), rects)).unwrap(),
        ])
        .unwrap()
    };

    assert_eq!(1, dfs.len());
    let (_, df) = dfs[0].clone().unwrap();

    assert_eq!(df_expected, df);
}

// ---

pub fn init_logs() {
    static INIT: AtomicBool = AtomicBool::new(false);

    if INIT.compare_exchange(false, true, SeqCst, SeqCst).is_ok() {
        re_log::set_default_rust_log_env();
        tracing_subscriber::fmt::init(); // log to stdout
    }
}