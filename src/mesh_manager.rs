use mesh::{Mesh, MeshId};

#[derive(Clone, Debug)]
pub struct MeshManager {
    meshes: Vec<Mesh>,
}

impl MeshManager {
    pub fn new() -> MeshManager {
        MeshManager { meshes: Vec::new() }
    }

    pub fn add(&mut self, mesh: Mesh) -> MeshId {
        self.meshes.push(mesh);
        MeshId {
            id: (self.meshes.len() as u16) - 1,
        }
    }

    pub fn set(&mut self, id: MeshId, mesh: Mesh) {
        let index = id.id as usize;
        self.meshes[index] = mesh;
    }

    pub fn get(&self, id: MeshId) -> &Mesh {
        let index = id.id as usize;
        &self.meshes[index]
    }
}
