use crate::common::HasId;

pub fn calculate_new_id<T: HasId>(items: &[T]) -> u32 {
    items
        .iter()
        .map(|item| item.get_id())
        .max()
        .map_or(1, |max_id| max_id + 1)
}
