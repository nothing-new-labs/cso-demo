use cso_core::metadata::{MdCache, MdId, Metadata};
use cso_demo::datum::Datum;
use cso_demo::statistics::{Bucket, ColumnMetadata, ColumnStats, Histogram, RelationMetadata, RelationStats};

#[test]
fn test_serialize_md_cache() {
    // mdids
    let relation_stats_id = Box::new(1u64) as Box<dyn MdId>;
    let relation_md_id = Box::new(2u64) as Box<dyn MdId>;
    let column_stats_id = Box::new(3u64) as Box<dyn MdId>;

    let json = serde_json::to_string(&relation_stats_id).unwrap();
    let new_relation_stats_id: Box<dyn MdId> = serde_json::from_str(json.as_str()).unwrap();
    debug_assert_eq!(relation_stats_id.to_string(), new_relation_stats_id.to_string());
    debug_assert_eq!("1", new_relation_stats_id.to_string());

    // relation stats
    let relation_stats = RelationStats::new("x".to_string(), 9011, false, vec![column_stats_id.clone()]);
    let boxed_relation_stats = Box::new(relation_stats.clone()) as Box<dyn Metadata>;
    let json = serde_json::to_string(&boxed_relation_stats).unwrap();
    let new_relation_stats: Box<dyn Metadata> = serde_json::from_str(json.as_str()).unwrap();

    // relation metadata
    let column_md = vec![
        ColumnMetadata::new("i".to_string(), 1, true, 4, Datum::I32(0)),
        ColumnMetadata::new("j".to_string(), 2, true, 4, Datum::I32(0)),
        ColumnMetadata::new("ctid".to_string(), 3, false, 4, Datum::I32(0)),
        ColumnMetadata::new("xmin".to_string(), 4, false, 4, Datum::I32(0)),
        ColumnMetadata::new("cmin".to_string(), 5, false, 4, Datum::I32(0)),
        ColumnMetadata::new("xmax".to_string(), 6, false, 4, Datum::I32(0)),
    ];
    let relation_md = RelationMetadata::new("x".to_string(), column_md, relation_stats_id.clone());
    let boxed_relation_md = Box::new(relation_md.clone()) as Box<dyn Metadata>;
    let json = serde_json::to_string(&boxed_relation_md).unwrap();
    let new_relation_md: Box<dyn Metadata> = serde_json::from_str(json.as_str()).unwrap();

    // column stats
    let buckets = vec![
        Bucket::new(Datum::I32(0), Datum::I32(1), 1, 2),
        Bucket::new(Datum::I32(1), Datum::I32(3), 3, 3),
    ];
    let histogram = Histogram::new(buckets);
    let column_stats = ColumnStats::new(1, 'x'.to_string(), Datum::I32(0), Datum::I32(1), 0, Some(histogram));
    let boxed_column_stats = Box::new(column_stats.clone()) as Box<dyn Metadata>;
    let json = serde_json::to_string(&boxed_column_stats).unwrap();
    let new_column_stats: Box<dyn Metadata> = serde_json::from_str(json.as_str()).unwrap();

    // metadata cache
    let mut md_cache = MdCache::new();
    md_cache.insert(relation_stats_id.clone(), new_relation_stats);
    md_cache.insert(relation_md_id.clone(), new_relation_md);
    md_cache.insert(column_stats_id.clone(), new_column_stats);

    let md_string = serde_json::to_string_pretty(&md_cache).unwrap();
    println!("{md_string}");

    let new_md_cache: MdCache = serde_json::from_str(md_string.as_str()).unwrap();

    let new_relation_stats = new_md_cache.get(&*relation_stats_id).unwrap();
    let new_relation_stats = new_relation_stats
        .downcast_ref::<RelationStats>()
        .expect("RelationStats expected");
    debug_assert_eq!(new_relation_stats.rows(), relation_stats.rows());

    let new_relation_md = new_md_cache.get(relation_md_id.as_ref()).unwrap();
    let new_relation_md = new_relation_md
        .downcast_ref::<RelationMetadata>()
        .expect("RelationMetadata expected");
    debug_assert_eq!(new_relation_md.name(), relation_md.name());

    let new_column_stats = new_md_cache.get(column_stats_id.as_ref()).unwrap();
    let new_column_stats = new_column_stats
        .downcast_ref::<ColumnStats>()
        .expect("ColumnStats expected");
    debug_assert_eq!(new_column_stats.null_count(), column_stats.null_count());
}
