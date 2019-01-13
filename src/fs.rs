use context::Context;
use core::unit::UnitType;
use mesh::Mesh;
use mesh_manager;
use obj;
use obj::Model;
use std::fs as std_fs;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use texture::{load_texture, Texture};

pub fn load_as_string<P: AsRef<Path>>(path: P) -> String {
    String::from_utf8(load(path).into_inner()).unwrap()
}

pub fn load_all_units(context: &mut Context) -> (mesh_manager::MeshManager, Vec<UnitType>) {
    let mut mm = mesh_manager::MeshManager::new();
    let mut units: Vec<UnitType> = Vec::new();

    //let dir = Path::new("./assets/units/");
    let dir = PathBuf::from("./assets/units/");
    assert!(dir.is_dir());

    for entry in std_fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            // If true, this is the defenition of a new unit
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let mesh = load_object_mesh(context, &path);
            let unit = load_unit_data(context, &path, name);
            if mesh.is_some() && unit.is_some() {
                mm.add(mesh.unwrap());
                units.push(unit.unwrap());
            }
        }
    }
    (mm, units)
}

/// Loads any file. Path starts from the assets folder
pub fn load<P: AsRef<Path>>(path: P) -> Cursor<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;

    let mut buf = Vec::new();
    let fullpathwithassets = &Path::new("assets").join(&path);
    let mut fullpath = &Path::new("").join(&path);
    println!("Fullpath - Befor: {}", fullpath.to_string_lossy());
    if !(fullpath.starts_with("assets")
        || fullpath.starts_with("/assets")
        || fullpath.starts_with("./assets"))
    {
        fullpath = fullpathwithassets;
    }
    println!("Fullpath - After: {}", fullpath.to_string_lossy());
    let mut file = match File::open(&fullpath) {
        Ok(file) => file,
        Err(err) => {
            panic!("Can`t open file '{}' ({})", fullpath.display(), err);
        }
    };
    match file.read_to_end(&mut buf) {
        Ok(_) => Cursor::new(buf),
        Err(err) => {
            panic!("Can`t read file '{}' ({})", fullpath.display(), err);
        }
    }
}

/// Recieves a folder and makes a Mesh from the .obj and .png files in that folder.
/// If no such files can be found, returns None.
pub fn load_object_mesh<P: AsRef<Path>>(context: &mut Context, path: &P) -> Option<Mesh> {
    let mut model: Option<Model> = None;
    let mut texture: Option<Texture> = None;
    for file in std_fs::read_dir(path).unwrap() {
        let file = file.unwrap().path();
        let ext = file.extension()?;
        if ext == "obj" {
            model = Some(obj::Model::new(&file));
        } else if ext == "png" {
            let texture_data = load(&file).into_inner();
            texture = Some(load_texture(context, &texture_data));
        }
    }
    if model.is_none() || texture.is_none() {
        return None;
    }
    let model = model.unwrap();
    let texture = texture.unwrap();

    let (vertices, indices) = obj::build(&model);
    Some(Mesh::new(context, &vertices, &indices, texture))
}

/// Load the data (not the meshes) of a single unit.
pub fn load_unit_data<P: AsRef<Path>>(
    context: &mut Context,
    path: &P,
    name: String,
) -> Option<UnitType> {
    Some(UnitType {
        name: name,
        count: 120,
        size: 2,
        hp: 3,
        defence_skill: 8,
        armor: 8,
        shield: 8,
        attack_skill: 8,
        // pub weapon_type: WeaponType,     // make enum weaopn type
        speed: 8,
        cost_recruit: 240.0,
        cost_upkeep: 50.0,
    })
}
