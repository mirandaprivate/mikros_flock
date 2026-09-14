use crate::{
    field::{SmallField, from_raw},
    product_tree::{Options, ProductTree, Strategy},
};
use flock_field::F128;

#[test]
fn every_saved_layer_equals_independent_direct_block_products() {
    for width in [1, 2, 3, 8, 16, 31, 64, 127, 128] {
        let field = SmallField::for_width(width).unwrap();
        let values: Vec<_> = (0..259)
            .map(|i| ((i * 19 + 3) as u128) & field.mask())
            .collect();
        let z = from_raw(0x1827ab7198769871928778);
        for strategy in [Strategy::BitwidthAware, Strategy::FullExtension] {
            let tree = ProductTree::new(
                &values,
                width,
                z,
                Options {
                    strategy,
                    ..Default::default()
                },
            )
            .unwrap();
            for level in 0..=tree.log_len {
                let expected: Vec<_> = (0..(512 >> level))
                    .map(|block| {
                        (block * (1 << level)..(block + 1) * (1 << level))
                            .fold(F128::ONE, |p, i| {
                                p * (z + field.embed(values.get(i).copied().unwrap_or(0)))
                            })
                    })
                    .collect();
                assert_eq!(tree.layer(level), expected, "width={width},level={level}");
            }
        }
    }
}
