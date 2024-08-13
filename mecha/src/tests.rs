use super::*;

/// Intersection test threshold
const TOLERANCE: f64 = 0.001;

/// Blender cube mesh exported to stl format (normals stripped)
fn cube_mesh() -> Load {
    vec![
        1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0,
        1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0,
        -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
        1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0,
        1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0,
    ]
    .load()
}

/// Blender cube mesh with Go/No-Go attribute interweaved in with XYZ pos attrib
fn cube_mesh_with_validation() -> Load {
    vec![
        1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0,
        -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
        1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
        -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
        1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, -1.0, 1.0,
    ]
    .load()
}

fn cube_import() -> Node {
    let path = TEST_DATA_PATH.to_owned() + "/cube.stl";
    Import::new().path(path).node()
}

fn import_mesh_and_export_validation_mesh(name: &str) -> Result<(), Error> {
    let path_in = TEST_DATA_PATH.to_owned() + "/" + name + ".stl";
    let mesh_in = Import::new().path(path_in).node();
    let path_out = TEST_OUT_PATH.to_owned() + "/" + name + ".obj";
    let mesh_out = Validate::new().mesh(mesh_in).tolerance(TOLERANCE).node();
    ValidationExport::new()
        .path(path_out)
        .mesh(mesh_out)
        .node()
        .solve(Task::Main)?;
    Ok(())
}

#[test]
fn import_cube_stl() -> Result<(), Error> {
    assert_eq!(cube_import().load()?, cube_mesh());
    Ok(())
}

#[test]
fn cube_validation() -> Result<(), Error> {
    let cube = cube_import();
    let validate = Validate::new().mesh(cube).tolerance(TOLERANCE).node();
    assert_eq!(validate.load()?, cube_mesh_with_validation());
    Ok(())
}

#[test]
fn test_cube() -> Result<(), Error> {
    import_mesh_and_export_validation_mesh("cube")
}

#[test]
fn test_cube_overlap() -> Result<(), Error> {
    import_mesh_and_export_validation_mesh("cube_overlap")
}

#[test]
fn test_cube_tilted() -> Result<(), Error> {
    import_mesh_and_export_validation_mesh("cube_tilted")
}

#[test]
fn test_cube_overlap_tilted() -> Result<(), Error> {
    import_mesh_and_export_validation_mesh("cube_overlap_tilted")
}

#[test]
fn test_monkey() -> Result<(), Error> {
    import_mesh_and_export_validation_mesh("monkey")
}

#[test]
fn test_monkey_with_hole() -> Result<(), Error> {
    import_mesh_and_export_validation_mesh("monkey_with_hole")
}
