extern crate rand;
extern crate image;

use rand::{distributions::WeightedIndex, prelude::*};
use image::GenericImageView;

#[derive(Clone)]
pub struct LAB {
    pub l: f32,
    pub a: f32,
    pub b: f32
}

#[derive(Clone)]
pub struct Lab(pastel::Lab);

impl Lab {
    pub fn distance(&self, color: &Lab) -> f64 {
        let xc1 = (self.0.a.powi(2) + self.0.b.powi(2)).sqrt();
        let xc2 = (color.0.a.powi(2) + color.0.b.powi(2)).sqrt();
        let xdl = color.0.l - self.0.l;
        let mut xdc = xc2 - xc1;
        let xde = ( (self.0.l - color.0.l).powi(2) + (self.0.a - color.0.a).powi(2) + (self.0.b - color.0.b).powi(2) ).sqrt();

        let mut xdh = xde.powi(2) - xdl.powi(2) - xdc.powi(2);
        if xdh > 0.0 {
            xdh = xdh.sqrt();
        } else {
            xdh = 0.0;
        }

        let xsc = 1.0 + 0.045 * xc1;
        let xsh = 1.0 + 0.015 * xc1;
        xdc /= xsc;
        xdh /= xsh;

        return ( xdl.powi(2) + xdc.powi(2) + xdh.powi(2) ).sqrt();
    }

    pub fn nearest(&self, colors: &Vec<Lab>) -> (usize, f64) {
        return colors
            .iter()
            .map(|c| self.distance(c))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(&b).expect("NaN encountered"))
            .unwrap();
    }
}


const KAPPA: f32 = 24389.0 / 27.0;
const EPSILON: f32 = 216.0 / 24389.0;

fn map_rgb_xyz(val: f32) -> f32 {
    return (val / 255.0).powf(2.19921875) * 100.0;
}

fn map_xyz_lab(val: f32) -> f32 {
    if val > EPSILON {
        return val.powf(1.0 / 3.0);
    } else {
        return (KAPPA * val + 16.0) / 116.0;
    }
}


impl LAB {
    // * Helper function to create a LAB color from RGB values without creating intermediate struct
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let var_r = map_rgb_xyz(r.into());
        let var_g = map_rgb_xyz(g.into());
        let var_b = map_rgb_xyz(b.into());

        let xyz : (f32, f32, f32) = (
            var_r*0.57667 + var_g*0.18555 + var_b*0.18819,
            var_r*0.29738 + var_g*0.62735 + var_b*0.07527,
            var_r*0.02703 + var_g*0.07069 + var_b*0.99110
        );

        let var_x = map_xyz_lab(xyz.0 / 95.047);
        let var_y = map_xyz_lab(xyz.1 / 100.0);
        let var_z = map_xyz_lab(xyz.2 / 108.883);

        return LAB {
            l: 116.0 * var_y - 16.0,
            a: 500.0 * (var_x - var_y),
            b: 200.0 * (var_y - var_z)
        };
    }

    // * Finds the index and distance from nearest color from a group of colors
    pub fn nearest(&self, colors: &Vec<LAB>) -> (usize, f32) {
        return colors
            .iter()
            .map(|c| self.distance(c))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(&b).expect("NaN encountered"))
            .unwrap();
    }

    // * Calculates Delta E(1994) between two colors
    pub fn distance(&self, color: &LAB) -> f32 {

        let xc1 = (self.a.powi(2) + self.b.powi(2)).sqrt();
        let xc2 = (color.a.powi(2) + color.b.powi(2)).sqrt();
        let xdl = color.l - self.l;
        let mut xdc = xc2 - xc1;
        let xde = ( (self.l - color.l).powi(2) + (self.a - color.a).powi(2) + (self.b - color.b).powi(2) ).sqrt();

        let mut xdh = xde.powi(2) - xdl.powi(2) - xdc.powi(2);
        if xdh > 0.0 {
            xdh = xdh.sqrt();
        } else {
            xdh = 0.0;
        }

        let xsc = 1.0 + 0.045 * xc1;
        let xsh = 1.0 + 0.015 * xc1;
        xdc /= xsc;
        xdh /= xsh;

        return ( xdl.powi(2) + xdc.powi(2) + xdh.powi(2) ).sqrt();
    }
}


fn recal_means_2(colors: &Vec<&Lab>) -> Lab {
    let mut new_color = pastel::Color::from_lab(0.0, 0.0, 0.0, 1.0).to_lab();
    let mut w_sum = 0.0;
    let w = 1.0;

    for col in colors.iter() {
        w_sum += w;
        new_color.l += w * col.0.l;
        new_color.a += w * col.0.a;
        new_color.b += w * col.0.b;
    }

    new_color.l /= w_sum;
    new_color.a /= w_sum;
    new_color.b /= w_sum;

    return Lab(new_color);
}

// * K-means++ clustering to create the palette
pub fn pigments_pixels_2(pixels: &Vec<Lab>, k: u8, max_iter: Option<u16>) -> Vec<(Lab, f32)> {
    // Values referenced from https://scikit-learn.org/stable/modules/generated/sklearn.cluster.KMeans.html
    const TOLERANCE: f64 = 1e-4;
    const MAX_ITER: u16 = 300;
    let mut rng = rand::thread_rng();

    // Randomly pick the starting cluster center
    let i: usize = rng.gen_range(0..pixels.len());
    let mut means: Vec<Lab> = vec![pixels[i].clone()];

    // Pick the remaining (k-1) means
    for _ in 0..(k - 1) {
        // Calculate the (nearest_distance)^2 for every color in the image
        let distances: Vec<f64> = pixels
            .iter()
            .map(|color| (color.nearest(&means).1).powi(2))
            .collect();

        // Create a weighted distribution based on distance^2
        // If error occurs, return the means already found
        let dist = match WeightedIndex::new(&distances) {
            Ok(t) => t,
            Err(_) => {
                // Calculate the dominance of each color
                let mut palette: Vec<(Lab, f32)> = means.iter().map(|c| (c.clone(), 0.0)).collect();
                let len = pixels.len() as f32;
                for color in pixels.iter() {
                    let near = color.nearest(&means).0;
                    palette[near].1 += 1.0 / len;
                }
                return palette;
            }
        };
        // Using the distances^2 as weights, pick a color and use it as a cluster center
        means.push(pixels[dist.sample(&mut rng)].clone());
    }

    let mut clusters: Vec<Vec<&Lab>>;
    let mut iters_left = max_iter.unwrap_or(MAX_ITER);
    loop {
        clusters = vec![Vec::new(); k as usize];
        for color in pixels.iter() {
            clusters[color.nearest(&means).0].push(color);
        }
        let mut changed: bool = false;
        for i in 0..clusters.len() {
            let new_mean = recal_means_2(&clusters[i]);
            if means[i].distance(&new_mean) > TOLERANCE {
                changed = true;
            }
            means[i] = new_mean;
        }
        iters_left -= 1;
        if !changed || iters_left <= 0 {
            break;
        }
    }

    // The length of every cluster divided by total pixels gives the dominance of each mean
    // For every mean, the corresponding dominance is added as a tuple item
    return clusters
        .iter()
        .enumerate()
        .map(|(i, cluster)| {
            (
                means[i].clone(),
                cluster.len() as f32 / pixels.len() as f32,
            )
        })
        .collect();
}




fn recal_means(colors: &Vec<&LAB>) -> LAB {
    let mut new_color = LAB {
        l: 0.0,
        a: 0.0,
        b: 0.0
    };
    let mut w_sum = 0.0;
    let w = 1.0;

    for col in colors.iter() {
        w_sum += w;
        new_color.l += w * col.l;
        new_color.a += w * col.a;
        new_color.b += w * col.b;
    }

    new_color.l /= w_sum;
    new_color.a /= w_sum;
    new_color.b /= w_sum;

    return new_color;
}

// * K-means++ clustering to create the palette
pub fn pigments_pixels(pixels: &Vec<LAB>, k: u8, max_iter: Option<u16>) -> Vec<(LAB, f32)> {
    // Values referenced from https://scikit-learn.org/stable/modules/generated/sklearn.cluster.KMeans.html
    const TOLERANCE: f32 = 1e-4;
    const MAX_ITER: u16 = 300;

    let mut rng = rand::thread_rng();

    // Randomly pick the starting cluster center
    let i: usize = rng.gen_range(0..pixels.len());
    let mut means: Vec<LAB> = vec![pixels[i].clone()];

    // Pick the remaining (k-1) means
    for _ in 0..(k - 1) {
        // Calculate the (nearest_distance)^2 for every color in the image
        let distances: Vec<f32> = pixels
            .iter()
            .map(|color| (color.nearest(&means).1).powi(2))
            .collect();

        // Create a weighted distribution based on distance^2
        // If error occurs, return the means already found
        let dist = match WeightedIndex::new(&distances) {
            Ok(t) => t,
            Err(_) => {
                // Calculate the dominance of each color
                let mut palette: Vec<(LAB, f32)> = means.iter().map(|c| (c.clone(), 0.0)).collect();
                let len = pixels.len() as f32;
                for color in pixels.iter() {
                    let near = color.nearest(&means).0;
                    palette[near].1 += 1.0 / len;
                }
                return palette;
            }
        };

        // Using the distances^2 as weights, pick a color and use it as a cluster center
        means.push(pixels[dist.sample(&mut rng)].clone());
    }

    let mut clusters: Vec<Vec<&LAB>>;
    let mut iters_left = max_iter.unwrap_or(MAX_ITER);
    loop {
        clusters = vec![Vec::new(); k as usize];
        for color in pixels.iter() {
            clusters[color.nearest(&means).0].push(color);
        }
        let mut changed: bool = false;
        for i in 0..clusters.len() {
            let new_mean = recal_means(&clusters[i]);
            if means[i].distance(&new_mean) > TOLERANCE {
                changed = true;
            }
            means[i] = new_mean;
        }
        iters_left -= 1;
        if !changed || iters_left <= 0 {
            break;
        }
    }

    // The length of every cluster divided by total pixels gives the dominance of each mean
    // For every mean, the corresponding dominance is added as a tuple item
    return clusters
        .iter()
        .enumerate()
        .map(|(i, cluster)| {
            (
                means[i].clone(),
                cluster.len() as f32 / pixels.len() as f32,
            )
        })
        .collect();
}



pub fn pigments(image_path: &str, count: u8) -> Result<Vec<(LAB, f32)>, Box<dyn std::error::Error>> {
    let mut img;
    img = image::open(image_path)?;
    img = img.resize(512, 512, image::imageops::FilterType::CatmullRom);

    let pixels: Vec<LAB> = img
        .pixels()
        .map(|(_, _, pix)| LAB::from_rgb(pix[0], pix[1], pix[2]))
        .collect();
    
    let pixels2: Vec<Lab> = img
        .pixels()
        .map(|(_, _, pix)| Lab(pastel::Color::from_lab(pix[0].into(), pix[1].into(), pix[2].into(), 1.0).to_lab()))
        .collect();

    let mut output = pigments_pixels(&pixels, count, None);
    output.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    return Ok(output);
}
