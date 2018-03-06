use world::{Actor, World};
use engine::{GameObject, Material, Mesh, RenderQueue};

pub struct SkyBox {}

impl Actor for SkyBox {
    fn new() -> Box<Actor> {
        Box::new(SkyBox {})
    }

    fn start(&mut self, go: &mut GameObject, world: &mut World) {
        let db = &mut world.asset_system();

        let mut material = Material::new(db.new_program("skybox"));
        material.set("uSkybox", db.new_texture("skybox/sky_cubemap.png"));
        material.render_queue = RenderQueue::Skybox;

        let mut mesh = Mesh::new();
        mesh.add_surface(db.new_mesh_buffer("skybox"), material);
        go.add_component(mesh);
    }
}