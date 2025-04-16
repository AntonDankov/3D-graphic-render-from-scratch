use crate::types::{get_vec3_identity, Entity, Mesh, Texture, TextureUV, Triangle, Vec3};
use image::GenericImageView;
use native_dialog::FileDialog;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn import_entity_from_obj(file_path: &str) -> Entity {
    let mut entity = Entity {
        mesh: Mesh {
            vertices: vec![],
            triangles: vec![],
        },
        rotation: Vec3::default(),
        scale: get_vec3_identity(),
        translation: Vec3::default(),
    };

    entity.translation.z = 5.0;
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file: {} | with error: {}", file_path, err);
            return entity;
        }
    };

    let mut texture_uvs: Vec<TextureUV> = vec![];
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
        } else if line.starts_with("vt ") {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 3 {
                let u = match parts[1].parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let v = match parts[2].parse::<f32>() {
                    Ok(val) => 1.0 - val,
                    Err(_) => continue,
                };
                texture_uvs.push(TextureUV { u: u, v: v });
            }
        } else if line.starts_with("f ") {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 4 {
                let a = match parts[1].split('/').collect::<Vec<&str>>()[0].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let b = match parts[2].split('/').collect::<Vec<&str>>()[0].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let c = match parts[3].split('/').collect::<Vec<&str>>()[0].parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let a_uv_index = match parts[1].split('/').collect::<Vec<&str>>()[1].parse::<i32>()
                {
                    Ok(val) => (val - 1) as usize,
                    Err(_) => continue,
                };
                let b_uv_index = match parts[2].split('/').collect::<Vec<&str>>()[1].parse::<i32>()
                {
                    Ok(val) => (val - 1) as usize,
                    Err(_) => continue,
                };
                let c_uv_index = match parts[3].split('/').collect::<Vec<&str>>()[1].parse::<i32>()
                {
                    Ok(val) => (val - 1) as usize,
                    Err(_) => continue,
                };

                let triangle = Triangle {
                    a: a,
                    b: b,
                    c: c,
                    a_uv: texture_uvs[a_uv_index],
                    b_uv: texture_uvs[b_uv_index],
                    c_uv: texture_uvs[c_uv_index],
                };
                entity.mesh.triangles.push(triangle);
            }
        }
    }

    // entity.rotation.x = 18.422432;
    // entity.rotation.y = 14.808098;
    // entity.rotation.z = 0.13040066;

    entity
}

pub fn import_texture(path: &str) -> Texture {
    let image = image::open(&Path::new(path)).unwrap();
    let (width, height) = image.dimensions();

    let mut data = Vec::with_capacity((width * height) as usize);
    let rgba = image.to_rgba8();
    dbg!(width, height);
    for pixel in rgba.pixels() {
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;
        let a = pixel[3] as u32;

        let argb = (a << 24) | (r << 16) | (g << 8) | b;
        data.push(argb);
    }
    dbg!(data.len());
    Texture {
        data: data,
        width: width,
        height: height,
    }
}

pub fn open_model_path() -> Option<PathBuf> {
    FileDialog::new()
        // .set_location("~")
        .add_filter("OBJ Models", &["obj"])
        .show_open_single_file()
        .unwrap_or(None)
}

pub fn open_texture_path() -> Option<PathBuf> {
    FileDialog::new()
        // .set_location("~")
        .add_filter("Texture files", &["png"])
        .show_open_single_file()
        .unwrap_or(None)
}
