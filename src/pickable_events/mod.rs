pub enum PickableComponentState {
    Spawned(ComponentSpawned)
}

pub enum ComponentSpawned {
    ComponentSpawned,
    ComponentNotSpawned,
    Any
}
