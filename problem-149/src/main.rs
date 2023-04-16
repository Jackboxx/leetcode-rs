/// Represents an equation of the form y = x + ca
/// where 'a' is an unknown
struct LinearEquation {
    y: f64,
    x: f64,
    coefficient: f64,
}

impl LinearEquation {
    fn solve_sytem(&self, other: &Self) -> bool {
        if self.coefficient == 0.0 {
            return self.y == self.x;
        }

        if other.coefficient == 0.0 {
            return other.y == other.x;
        }

        let canceling_coefficient = -self.coefficient / other.coefficient;

        let left = self.y + other.y * canceling_coefficient;
        let right =
            (self.x + self.coefficient) + (other.x + other.coefficient) * canceling_coefficient;

        left == right
    }
}

fn create_parameter_vectors(points: Vec<Vec<i32>>) -> Vec<(LinearEquation, LinearEquation)> {
    let mut equations = Vec::with_capacity(points.len() * (points.len() - 1));
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }

            let x1 = points[i][0];
            let y1 = points[i][1];

            let x2 = points[j][0];
            let y2 = points[j][1];

            let l1 = LinearEquation {
                y: 0.0,
                x: x1 as f64,
                coefficient: (x2 - x1) as f64,
            };

            let l2 = LinearEquation {
                y: 0.0,
                x: y1 as f64,
                coefficient: (y2 - y1) as f64,
            };

            equations.push((l1, l2));
        }
    }

    equations
}

fn max_points(points: Vec<Vec<i32>>) -> i32 {
    if points.len() == 1 {
        return 1;
    }

    let vectors = create_parameter_vectors(points.clone());
    let mut max = 0;

    for vector in vectors {
        let mut count = 0;
        let mut l1 = vector.0;
        let mut l2 = vector.1;

        for point in &points {
            let x = point[0] as f64;
            let y = point[1] as f64;

            l1.y = x;
            l2.y = y;

            if l1.solve_sytem(&l2) {
                count += 1;
            }
        }

        if count > max {
            max = count;
        }
    }

    max
}

fn main() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    max_points(points);

    let points = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];

    max_points(points);
}
