use crate::csf::particle::Particle;
use indicatif::ProgressStyle;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use las::Point;
use ndarray::Array2;
use std::io::prelude::*;

pub struct ClothSurface {
    pub particles: Array2<Particle>,
    pub simulation_threshold: f64,
    pub classification_threshold: f64,
    pub rigidness: f64,
    pub displacement: f64,
    bounds: ((f64, f64), (f64, f64)),
    cell_resolution: f64,
}

impl ClothSurface {
    pub fn initialize<'a>(
        lower_left: (f64, f64),
        upper_right: (f64, f64),
        cell_resolution: f64,
        simulation_threshold: f64,
        classification_threshold: f64,
        rigidness: f64,
        top_z: f64,
    ) -> ClothSurface {
        let rows = ((upper_right.1 - lower_left.1) / cell_resolution).ceil() as usize;
        let columns = ((upper_right.0 - lower_left.0) / cell_resolution).ceil() as usize;
        let mut particles: Array2<Particle> = Array2::default((rows, columns));
        for i in 0..rows {
            for j in 0..columns {
                let x = lower_left.0 + cell_resolution * j as f64;
                let y = lower_left.1 + cell_resolution * i as f64;
                particles[[i, j]].x = x;
                particles[[i, j]].y = y;
                particles[[i, j]].z.set(top_z);
            }
        }
        let upper_right_corrected = (
            lower_left.0 + cell_resolution * (columns - 1) as f64,
            lower_left.1 + cell_resolution * (rows - 1) as f64,
        );
        ClothSurface {
            particles,
            simulation_threshold,
            classification_threshold,
            rigidness,
            displacement: 0.05,
            bounds: (lower_left, upper_right_corrected),
            cell_resolution,
        }
    }

    pub fn is_ground_point(&self, point: &Point) -> bool {
        let particle = match self.get_closest_cell(point) {
            Some((row, column)) => &self.particles[[row, column]],
            None => return false,
        };
        let distance = (point.z - particle.z.get()).abs();
        distance < self.classification_threshold
    }

    fn iterate(&mut self) -> f64 {
        for i in 0..self.particles.nrows() {
            for j in 0..self.particles.ncols() {
                let neighbours = self.get_neighbours(i, j);
                self.particles[[i, j]].apply_force(self.rigidness, neighbours, self.displacement);
            }
        }
        let mut max_distance = 0.0;
        for i in 0..self.particles.nrows() {
            for j in 0..self.particles.ncols() {
                let mut particle = &mut self.particles[[i, j]];
                let distance = (particle.z.get() - particle.prev_z).abs();
                if distance > max_distance {
                    max_distance = distance;
                }
                particle.prev_z = particle.z.get();
            }
        }
        max_distance
    }

    pub fn simulate(&mut self) {
        let mut iteration = 0;
        let mut max_distance = f64::INFINITY;
        let spinner = indicatif::ProgressBar::new_spinner();
        while max_distance > self.simulation_threshold {
            spinner.set_message(format!(
                "Simulation threshold {} (meters) not reached, currently at {:.3}",
                self.simulation_threshold, max_distance
            ));
            spinner.inc(1);
            max_distance = self.iterate();
            iteration += 1;
        }
        spinner.finish_with_message(format!("Simulation finished with {} iterations", iteration));
    }

    fn get_neighbours(&self, i: usize, j: usize) -> Vec<&Particle> {
        let mut neighbours = Vec::new();
        let rows = self.particles.nrows();
        let columns = self.particles.ncols();
        if i > 0 {
            neighbours.push(&self.particles[[i - 1, j]]);
        }
        if i < rows - 1 {
            neighbours.push(&self.particles[[i + 1, j]]);
        }
        if j > 0 {
            neighbours.push(&self.particles[[i, j - 1]]);
        }
        if j < columns - 1 {
            neighbours.push(&self.particles[[i, j + 1]]);
        }

        neighbours
    }

    pub fn to_asc(&self, filename: &str) {
        let mut file = std::fs::File::create(filename).unwrap();
        let mut header = String::new();
        header.push_str("ncols ");
        header.push_str(&self.particles.ncols().to_string());
        header.push('\n');
        header.push_str("nrows ");
        header.push_str(&self.particles.nrows().to_string());
        header.push('\n');
        header.push_str("xllcorner ");
        header.push_str(&self.particles[[0, 0]].x.to_string());
        header.push('\n');
        header.push_str("yllcorner ");
        header.push_str(&self.particles[[0, 0]].y.to_string());
        header.push('\n');
        header.push_str("cellsize ");
        header.push_str(&((self.particles[[0, 1]].x - self.particles[[0, 0]].x).to_string()));
        header.push('\n');
        header.push_str("NODATA_value ");
        header.push_str(&(-9999.0).to_string());
        header.push('\n');
        file.write_all(header.as_bytes()).unwrap();
        for i in 0..self.particles.nrows() {
            let mut line = String::new();
            for j in 0..self.particles.ncols() {
                line.push_str(&self.particles[[i, j]].z.get().to_string());
                line.push_str(" ");
            }
            file.write_all(line.as_bytes()).unwrap();
        }
    }

    pub fn set_max_z_if_closest_to_particle(&mut self, point: &Point) {
        let particle = match self.get_closest_cell(point) {
            Some((row, col)) => &mut self.particles[[row, col]],
            None => return,
        };

        let distance = (particle.x - point.x).powi(2) + (particle.y - point.y).powi(2);
        if distance < particle.closest_pt_distance {
            particle.closest_pt_distance = distance;
            particle.max_z = point.z;
        }
    }

    fn get_closest_cell(&self, point: &Point) -> Option<(usize, usize)> {
        let ll = self.bounds.0;
        let ur = self.bounds.1;
        let x = point.x;
        let y = point.y;
        let cell_resolution = self.cell_resolution;
        let col = ((x - ll.0) / cell_resolution).floor() as usize;
        let row = ((ur.1 - y) / cell_resolution).ceil() as usize;
        if row >= self.particles.nrows() || col >= self.particles.ncols() {
            println!(
                "WARNING: Point ({}, {}) outside of cloth surface with ll: ({}, {}), ur ({}, {})",
                point.x, point.y, ll.0, ll.1, ur.0, ur.1
            );
            return None;
        }
        Some((row, col))
    }

    pub fn fix_zero_max_heights(&mut self) {
        // Assign max_z of closest particle that has non-zero max_z to all particles with max_z = 0
        let mut kd = KdTree::new(2);
        for particle in &self.particles {
            if particle.max_z != 0.0 {
                kd.add([particle.x.clone(), particle.y.clone()], particle.max_z)
                    .unwrap();
            }
        }
        for particle in &mut self.particles {
            if particle.max_z == 0.0 {
                let closest = kd
                    .nearest(&[particle.x, particle.y], 1, &squared_euclidean)
                    .unwrap();
                particle.max_z = *closest[0].1;
            }
        }
    }
}
