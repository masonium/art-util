extern crate nalgebra as na;
use ndarray::prelude::*;
use rand::distributions::Uniform;
use rand::Rng;

pub struct PoissonSampling {
    radius: f32,
    rr_dist: Uniform<f32>,
    theta_dist: Uniform<f32>,
    cell_size: f32,

    w: f32,
    h: f32,

    uw: Uniform<f32>,
    uh: Uniform<f32>,

    pub points: Vec<na::Point2<f32>>,
    grid_accel: Array2<Option<usize>>,
    pub active_list: Vec<usize>,
}

impl PoissonSampling {
    pub fn new(r: f32, w: f32, h: f32) -> PoissonSampling {
        let d = r / (2.0 as f32).sqrt();

        let capac = (w * h / (std::f32::consts::PI * r * r)) as usize;
        let points = Vec::with_capacity(capac);

        let minwh = if w < h { w } else { h };
        let active_list: Vec<usize> =
            Vec::with_capacity((std::f32::consts::PI * minwh / r) as usize);
        let grid_accel =
            Array2::<Option<usize>>::from_elem(((w / d) as usize + 1, (h / d) as usize + 1), None);

        PoissonSampling {
            radius: r,
            cell_size: r / (2.0 as f32).sqrt(),
            rr_dist: Uniform::new(r * r, 4.0 * r * r),
            theta_dist: Uniform::new(0.0, 2.0 * std::f32::consts::PI),

            w,
            h,

            uw: Uniform::new(0.0, w),
            uh: Uniform::new(0.0, h),

            points,
            active_list,
            grid_accel,
        }
    }

    /// Return a value of 'r' that will yield roughly `n` total poisson samples.
    pub fn radius_from_n(w: f32, h: f32, n: usize) -> f32 {
        2.0 * (w * h * 1.12 / (n as f32 * std::f32::consts::PI)).sqrt()
    }

    fn random_annulus<R: Rng>(&self, r: &mut R, p: na::Point2<f32>) -> na::Point2<f32> {
        let u = r.sample(self.rr_dist).sqrt();
        let v = r.sample(self.theta_dist);

        na::Point2::new(p.x + u * v.cos(), p.y + u * v.sin())
    }

    fn add_point(&mut self, p: na::Point2<f32>) {
        let d = self.cell_size;
        self.grid_accel[((p.x / d) as usize, (p.y / d) as usize)] = Some(self.points.len());
        self.active_list.push(self.points.len());
        self.points.push(p);
    }

    pub fn samples<R: Rng>(&mut self, r: &mut R) -> &Vec<na::Point2<f32>> {
        while self.next_sample(r).is_some() {}
        &self.points
    }

    pub fn into_points<R: Rng>(mut self, r: &mut R) -> Vec<na::Point2<f32>> {
        self.samples(r);
        self.points
    }

    pub fn next_sample<R: Rng>(&mut self, r: &mut R) -> Option<na::Point2<f32>> {
        if self.points.is_empty() {
            let p = na::Point2::new(r.sample(&self.uw), r.sample(&self.uh));
            self.add_point(p);
            return Some(p);
        }

        while !self.active_list.is_empty() {
            // grab a random active point
            let i = r.sample(Uniform::new(0, self.active_list.len()));
            let p = self.points[self.active_list[i]];

            let k = 30;

            'point_loop: for _ in 0..k {
                // sample a point within
                let p_new = self.random_annulus(r, p);
                // println!("Distance {} from active point", (p_new-p).dot(&(p_new-p)).sqrt());

                if p_new.x < 0.0 || p_new.x >= self.w || p_new.y < 0.0 || p_new.y >= self.h {
                    continue 'point_loop;
                }

                let pxi = (p_new.x / self.cell_size) as usize;
                let pyi = (p_new.y / self.cell_size) as usize;

                for xi in &[-1, 0, 1] {
                    let pnxi = xi + pxi as isize;
                    if pnxi < 0 || pnxi >= self.grid_accel.dim().0 as isize {
                        continue;
                    }

                    for yi in &[-1, 0, 1] {
                        let pnyi = yi + pyi as isize;
                        if pnyi < 0 || pnyi >= self.grid_accel.dim().1 as isize {
                            continue;
                        }

                        if let Some(ref other_pi) = self.grid_accel[(pnxi as usize, pnyi as usize)]
                        {
                            let dr = p_new - self.points[*other_pi];
                            let r2 = dr.dot(&dr);
                            if r2 < self.radius * self.radius {
                                continue 'point_loop;
                            }
                        }
                    }
                }

                // if we get here, we've found a point
                self.add_point(p_new);
                return Some(p_new);
            }

            // if we get here, the point we had didn't work
            self.active_list.remove(i);
        }

        None
    }
}
