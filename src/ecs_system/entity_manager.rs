struct EntityManager {
    health_components: Vec<Option<Health>>,
    positions_components: Vec<Option<Positions>>,
    character_components: Vec<Option<Character>>,
}


impl EntityManager {

    fn new() -> Self {
        Self {
            health_components: Vec::new(),
            name_components: Vec::new(),
        }
    }

    fn new_entity(&mut self, health: Option<Health>, name: Option<Name>) {
        self.health_components.push(health);
        self.name_components.push(name);
    }
}