use std::{collections::HashMap, fs};

use num::complex::ComplexFloat;
use serde_json::{json, Value};

pub type Complex = num::complex::Complex<f32>;

pub struct StateVector {
    pub bases: Vec<Complex>,
}

pub struct DensityMatrix {
    pub bases: Vec<Vec<Complex>>,
}

pub struct Statistics {
    pub memory: HashMap<String, usize>,
}

impl StateVector {
    pub fn as_complex_bases(&self) -> &[Complex] {
        &self.bases
    }
    pub fn probabilities(&self) -> Vec<f32> {
        self.bases.iter().map(|c| c.norm_sqr()).collect()
    }
}

impl DensityMatrix {
    pub fn as_complex_bases(&self) -> &[Vec<Complex>] {
        &self.bases
    }
    pub fn probabilities(&self) -> Vec<f32> {
        (0..self.bases.len())
            .collect::<Vec<usize>>()
            .iter()
            .map(|&index| self.bases[index][index].re())
            .collect()
    }
}

pub async fn read_state(sv: &mut StateVector, source: &str) {
    for line in fs::read_to_string(source.to_string() + ".state")
        .unwrap()
        .lines()
    {
        let complex = line
            .split(" ")
            .filter_map(|s| s.parse::<f32>().ok())
            .collect::<Vec<_>>();
        sv.bases.push(Complex::new(complex[0], complex[1]));
    }
}

pub async fn print_state(statevector: &StateVector, probabilities: &[f32]) -> Value {
    let mut json = json!({});

    let amplitudes_and_probabilities = statevector
        .as_complex_bases()
        .iter()
        .zip(probabilities)
        .enumerate();
    for (idx, (amplitude, probability)) in amplitudes_and_probabilities {
        json[format!("{}", idx)] = json!({});
        json[format!("{}", idx)]["Real"] = json!(format!("{:.6}", amplitude.re));
        json[format!("{}", idx)]["Imaginary"] = json!(format!("{:.6}", amplitude.im));
        json[format!("{}", idx)]["Probability"] = json!(format!("{:.6}", probability));
    }

    json!({
        "State": json
    })
}

pub async fn read_density(dm: &mut DensityMatrix, source: &str) {
    for line in fs::read_to_string(source.to_string() + ".state")
        .unwrap()
        .lines()
    {
        let row = line
            .split(" ")
            .filter_map(|s| s.parse::<f32>().ok())
            .collect::<Vec<_>>();
        dm.bases
            .push(row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect());
    }
}

pub async fn print_density_matrix(density_matrix: &DensityMatrix, probabilities: &[f32]) -> Value {
    let mut json = json!({});

    let amplitudes_and_probabilities = density_matrix
        .as_complex_bases()
        .iter()
        .zip(probabilities)
        .enumerate();
    for (idx, (amplitude, probability)) in amplitudes_and_probabilities {
        json[format!("{}", idx)] = json!({});
        json[format!("{}", idx)]["Row"] = json!(format!("{:?}", amplitude));
        json[format!("{}", idx)]["Probability"] = json!(format!("{:.6}", probability));
    }

    json!({
        "DensityMatrix": json
    })
}

pub async fn read_stats(stats: &mut Statistics, source: &str) {
    for line in fs::read_to_string(source.to_string() + ".stats")
        .unwrap()
        .lines()
    {
        let complex = line
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        stats.memory.insert(
            complex[..complex.len() - 1]
                .iter()
                .map(|c| c.to_string())
                .collect::<String>(),
            complex.last().unwrap().clone(),
        );
    }
}

pub async fn print_stats(stats: &Statistics) -> Value {
    let mut json = json!({});

    stats.memory.iter().for_each(|(key, value)| {
        json[key] = json!(value);
    });

    json!({
        "Memory": json
    })
}
