use super::PermissionEntity;

impl From<PermissionEntity> for Overrides {
    fn from(entity: PermissionEntity) -> Self {
        Overrides {
            allowed: vec![entity],
            denied: vec![],
        }
    }
}

impl From<Vec<PermissionEntity>> for Overrides {
    fn from(entities: Vec<PermissionEntity>) -> Self {
        Overrides {
            allowed: entities,
            denied: vec![],
        }
    }
}

#[derive(Default, Debug)]
pub struct Overrides {
    pub allowed: Vec<PermissionEntity>,
    pub denied: Vec<PermissionEntity>,
}

impl Overrides {
    pub fn empty() -> Self {
        Overrides {
            allowed: vec![],
            denied: vec![],
        }
    }

    pub fn allow(&mut self, entity: PermissionEntity) {
        self.allowed.push(entity);
    }

    pub fn deny(&mut self, entity: PermissionEntity) {
        self.denied.push(entity);
    }
}
