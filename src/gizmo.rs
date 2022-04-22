mod position;

use kiss3d::scene::SceneNode;
use na::Vector3;
use std::path::Path;

pub struct GizmoPathes<'a> {
    pub gizmo_obj: &'a Path,
    pub gizmo_mtl: &'a Path,
}

pub fn spawn_gizmo(object: &mut SceneNode) {
    let gizmo_model = GizmoPathes {
        gizmo_obj: Path::new("src/assets/gizmo.obj"),
        gizmo_mtl: Path::new("src/assets/gizmo.mtl"),
    };

    let gizmo_scale = Vector3::new(10.0, 10.0, 10.0);

    object.add_obj(gizmo_model.gizmo_obj, gizmo_model.gizmo_mtl, gizmo_scale);
}
