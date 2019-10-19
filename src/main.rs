use mtpng;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;
use std::thread;

mod camera;
mod hittable;
mod materials;
mod ray;
mod vec3;
use camera::*;
use hittable::*;
use materials::*;
use ray::*;
use vec3::*;

fn color(world: &HittableList, ray: &Ray, rng: &mut ThreadRng, depth: u8) -> Vec3 {
    if depth > 50 {
        return Vec3(0.0, 0.0, 0.0);
    }
    let hit_record = world.hit(&ray, 0.001, std::f32::MAX);
    if hit_record.t > 0.0 {
        let (scattered_ray, attenuation) =
            hit_record.material.unwrap().scatter(ray, &hit_record, rng);
        let refl = color(world, &scattered_ray, rng, depth + 1);
        return refl * attenuation;
    }

    // hit nothing. paint the sky:
    let unit_dir = ray.dir.normalized();
    let a = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - a) * Vec3(1.0, 1.0, 1.0) + (a) * Vec3(0.5, 0.7, 1.0)
}

fn clamp_to_u8(val: f32) -> u8 {
    let mut result = val.sqrt();
    if result < 0.0 {
        result = 0.0;
    }
    if result > 1.0 {
        result = 1.0;
    }
    (result * 255.99) as u8
}

fn build_world(rng: &mut ThreadRng) -> HittableList {
    let mut world = HittableList::new();
    world.push(Sphere {
        center: Vec3(-1.0, 0.5, 0.0),
        radius: 0.5,

        material: Box::new(DiffuseMaterial {
            albedo: Vec3(0.5, 0.7, 0.5),
        }),
    });
    world.push(Sphere {
        center: Vec3(0.0, 0.5, 0.0),
        radius: 0.5,
        material: Box::new(MetalMaterial {
            albedo: Vec3(0.8, 0.8, 0.8),
            fuzz: 0.1,
        }),
    });
    world.push(Sphere {
        center: Vec3(1.0, 0.5, 0.0),
        radius: 0.5,
        material: Box::new(GlassMaterial {
            albedo: Vec3(0.9, 0.9, 0.9),
            ref_idx: 1.5,
        }),
    });
    world.push(Sphere {
        center: Vec3(0.0, -500.0, 0.0),
        radius: 500.0,
        material: Box::new(DiffuseMaterial {
            albedo: Vec3(0.3, 0.35, 0.4),
        }),
    });

    for x in -11..5 {
        for z in -11..5 {
            let center = Vec3(
                0.5 * (x as f32 + 0.8 * rng.gen::<f32>()),
                0.1,
                0.5 * (z as f32 + 0.8 * rng.gen::<f32>()),
            );
            if center.squared_length() < 25.0
                && (center - Vec3(-1.0, 0.0, 0.0)).squared_length() > 0.25
                && (center - Vec3(0.0, 0.0, 0.0)).squared_length() > 0.25
                && (center - Vec3(1.0, 0.0, 0.0)).squared_length() > 0.25
            {
                let material: Box<dyn Material + Send + Sync> = if rng.gen::<f32>() > 0.5 {
                    Box::new(MetalMaterial {
                        albedo: Vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()),
                        fuzz: rng.gen::<f32>(),
                    })
                } else {
                    Box::new(DiffuseMaterial {
                        albedo: Vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()),
                    })
                };
                world.push(Sphere {
                    center,
                    radius: 0.1,
                    material,
                })
            }
        }
    }
    world
}

fn main() {
    let width: usize = 2560;
    let height: usize = 1600;
    let rays_per_pixel: usize = 1000;
    let n_threads: usize = 100;
    let mut threads = vec![];
    let n_pixels_per_thread = width * height / n_threads;
    let mut rng = rand::thread_rng();

    let camera = Arc::new(Camera::new(
        Vec3(6.0, 1.2, 3.0),
        Vec3(0.0, 0.5, 0.0),
        Vec3(0.0, 1.0, 0.0),
        25.0,
        width as f32 / height as f32,
        0.1,
        0.9,
    ));

    println!("camera: {:?}", camera);
    let world = Arc::new(build_world(&mut rng));
    for t_i in 0..n_threads {
        let camera = camera.clone();
        let world = world.clone();
        let from = t_i * n_pixels_per_thread;
        let to = from + n_pixels_per_thread;
        threads.push(thread::spawn(move || {
            let mut data: Vec<u8> = Vec::with_capacity(n_pixels_per_thread * 4);
            data.resize(n_pixels_per_thread * 4, 0);

            let mut t_rng = rand::thread_rng();
            for i in from..to {
                let mut rgb = Vec3(0.0, 0.0, 0.0);
                for _k in 0..rays_per_pixel {
                    let u = ((i % width) as f32 + t_rng.gen::<f32>()) / width as f32;
                    let v = 1.0 - ((i / width) as f32 + t_rng.gen::<f32>()) / height as f32;
                    let ray = camera.get_ray(u, v, &mut t_rng);
                    rgb += color(&world, &ray, &mut t_rng, 1);
                }
                rgb /= rays_per_pixel as f32;
                data[4 * (i - from)] = clamp_to_u8(rgb.r()) as u8;
                data[4 * (i - from) + 1] = clamp_to_u8(rgb.g()) as u8;
                data[4 * (i - from) + 2] = clamp_to_u8(rgb.b()) as u8;
                data[4 * (i - from) + 3] = 255_u8;
            }
            (from, to, data)
        }));
    }
    let mut aggregated_data = Vec::with_capacity(width * height * 4);
    aggregated_data.resize(width * height * 4, 0);
    for t in threads {
        let (from, to, data) = t.join().unwrap();
        for i in from..to {
            aggregated_data[4 * i] = data[4 * (i - from)];
            aggregated_data[4 * i + 1] = data[4 * (i - from) + 1];
            aggregated_data[4 * i + 2] = data[4 * (i - from) + 2];
            aggregated_data[4 * i + 3] = data[4 * (i - from) + 3];
        }
        save_to_file("out_image.png", &aggregated_data, width, height).unwrap();
    }
    // let d2 = t2.join().unwrap();
    // save_to_file("out_image.png", &d2, width, height)
}

fn save_to_file(fname: &str, data: &Vec<u8>, width: usize, height: usize) -> std::io::Result<()> {
    let mut header = mtpng::Header::new();
    header.set_size(width as u32, height as u32).unwrap();
    header
        .set_color(mtpng::ColorType::TruecolorAlpha, 8)
        .unwrap();
    let options = mtpng::encoder::Options::new();
    let path = Path::new(fname);
    let file_writer = BufWriter::new(File::create(path)?);
    let mut encoder = mtpng::encoder::Encoder::new(file_writer, &options);

    encoder.write_header(&header).unwrap();
    encoder.write_image_rows(&data).unwrap();
    encoder.finish().unwrap();
    Ok(())
}
