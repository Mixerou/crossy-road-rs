pub(super) mod forest;
pub(super) mod spawn_point;

#[derive(Debug)]
pub(super) enum Region {
    SpawnPoint,
    Forest,
}
