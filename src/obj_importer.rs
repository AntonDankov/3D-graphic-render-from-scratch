use crate::types::{Entity, IntVec3, Mesh, Vec2, Vec3};
use native_dialog::FileDialog;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn import_entity_from_obj(file_path: &str) -> Entity {
    let mut entity = Entity {
        mesh: Mesh {
            vertices: vec![],
            indexes: vec![],
            triangles: vec![],
        },
        rotation: Vec3::default(),
    };
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file: {} | with error: {}", file_path, err);
            return entity;
        }
    };

    let reader = BufReader::new(file);

    for line_res in reader.lines() {
        let line = match line_res {
            Ok(line) => line,
            Err(err) => {
                println!("Error reading line: {}", err);
                continue; // Skip this line
            }
        };

        if line.starts_with("v ") {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 4 {
                let x = match parts[1].parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let y = match parts[2].parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let z = match parts[3].parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                entity.mesh.vertices.push(Vec3 { x, y, z });
            }
        } else if line.starts_with("f ") {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 4 {
                let x = match parts[1].split('/').collect::<Vec<&str>>()[0].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let y = match parts[2].split('/').collect::<Vec<&str>>()[0].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let z = match parts[3].split('/').collect::<Vec<&str>>()[0].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                entity.mesh.indexes.push(IntVec3 { x: x, y: y, z: z });
            }
        }
    }

    entity.rotation.x = 18.422432;
    entity.rotation.y = 14.808098;
    entity.rotation.z = 0.13040066;

    entity
}

pub fn open_model_path() -> Option<PathBuf> {
    FileDialog::new()
        .set_location("~")
        .add_filter("OBJ Models", &["obj"])
        .show_open_single_file()
        .unwrap_or(None)
}
