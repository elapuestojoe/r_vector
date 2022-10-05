pub mod vector {
    use r_float::Float;
    use std::ops;

    #[derive(Clone, PartialEq, Debug)]
    pub struct Vector<T>
    where
        T: Float,
    {
        position: [T; 3],
    }

    #[allow(dead_code)]
    impl<T> Vector<T>
    where
        T: Float,
    {
        pub fn new(a: T, b: T, c: T) -> Vector<T> {
            Vector {
                position: [a, b, c],
            }
        }

        pub fn r(&self) -> T {
            self.position[0]
        }
        pub fn g(&self) -> T {
            self.position[1]
        }
        pub fn b(&self) -> T {
            self.position[2]
        }
        pub fn x(&self) -> T {
            self.position[0]
        }
        pub fn y(&self) -> T {
            self.position[1]
        }
        pub fn z(&self) -> T {
            self.position[2]
        }

        pub fn length(&self) -> T {
            self.length_squared().sqrt()
        }

        pub fn length_squared(&self) -> T {
            self.position[0].powi(2) + self.position[1].powi(2) + self.position[2].powi(2)
        }

        pub fn random() -> Vector<T> {
            Vector {
                position: [T::random(), T::random(), T::random()],
            }
        }

        pub fn random_range(low: T, high: T) -> Vector<T> {
            Vector {
                position: [
                    T::random_range(low, high),
                    T::random_range(low, high),
                    T::random_range(low, high),
                ],
            }
        }
    }

    pub trait VectorOperations<T>
    where
        T: Float,
    {
        fn cross(&self, rhs: &Vector<T>) -> Vector<T>;
        fn dot(&self, rhs: &Vector<T>) -> T;
        fn make_unit(&mut self) -> &mut Vector<T>;
        fn unit_vector(&self) -> Vector<T>;
        fn reflect(&self, rhs: &Vector<T>) -> Vector<T>;
        fn refract(
            &self,
            rhs: &Vector<T>,
            refraction_index: T,
        ) -> Option<Vector<T>>;
    }

    impl<T> VectorOperations<T> for Vector<T>
    where
        T: Float,
    {
        fn cross(&self, rhs: &Vector<T>) -> Vector<T> {
            Vector {
                position: [
                    self.position[1] * rhs.position[2] - self.position[2] * rhs.position[1],
                    -(self.position[0] * rhs.position[2] - self.position[2] * rhs.position[0]),
                    self.position[0] * rhs.position[1] - self.position[1] * rhs.position[0],
                ],
            }
        }
        fn dot(&self, rhs: &Vector<T>) -> T {
            self.position[0] * rhs.position[0]
                + self.position[1] * rhs.position[1]
                + self.position[2] * rhs.position[2]
        }
        fn make_unit(&mut self) -> &mut Vector<T> {
            *self = self.unit_vector();
            self
        }
        fn unit_vector(&self) -> Vector<T> {
            let k = self.length().recip();
            Vector {
                position: [
                    self.position[0] * k,
                    self.position[1] * k,
                    self.position[2] * k,
                ],
            }
        }
        fn reflect(&self, rhs: &Vector<T>) -> Vector<T> {
            self - &(rhs * (self.dot(rhs) * T::from_i32(2)))
        }
        fn refract(
            &self,
            rhs: &Vector<T>,
            refraction_index: T,
        ) -> Option<Vector<T>> {
            let unit_vector = self.unit_vector();
            let determinant = self.dot(rhs);
            let discriminant =
                T::one() - refraction_index.powi(2) * (T::one() - determinant.powi(2));
            if discriminant <= T::zero() {
                return Option::None;
            }
            Some(
                (unit_vector - self * determinant) * refraction_index - (rhs * discriminant.sqrt()),
            )
        }
    }

    impl<T> ops::Add<&Vector<T>> for &Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn add(self, rhs: &Vector<T>) -> Vector<T> {
            Vector {
                position: [
                    self.position[0] + rhs.position[0],
                    self.position[1] + rhs.position[1],
                    self.position[2] + rhs.position[2],
                ],
            }
        }
    }

    impl<T> ops::Add<&Vector<T>> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn add(self, rhs: &Vector<T>) -> Vector<T> {
            return &self + rhs;
        }
    }

    impl<T> ops::Add<Vector<T>> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn add(self, rhs: Vector<T>) -> Vector<T> {
            return &self + &rhs;
        }
    }

    impl<T> ops::Sub<&Vector<T>> for &Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn sub(self, rhs: &Vector<T>) -> Vector<T> {
            Vector {
                position: [
                    self.position[0] - rhs.position[0],
                    self.position[1] - rhs.position[1],
                    self.position[2] - rhs.position[2],
                ],
            }
        }
    }

    impl<T> ops::Sub<&Vector<T>> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn sub(self, rhs: &Vector<T>) -> Vector<T> {
            return &self - rhs;
        }
    }

    impl<T> ops::Sub<Vector<T>> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn sub(self, rhs: Vector<T>) -> Vector<T> {
            return &self - &rhs;
        }
    }

    impl<T> ops::Div<&Vector<T>> for &Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn div(self, rhs: &Vector<T>) -> Vector<T> {
            Vector {
                position: [
                    self.position[0] / rhs.position[0],
                    self.position[1] / rhs.position[1],
                    self.position[2] / rhs.position[2],
                ],
            }
        }
    }

    impl<T> ops::Div<Vector<T>> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn div(self, rhs: Vector<T>) -> Vector<T> {
            return &self / &rhs;
        }
    }

    impl<T> ops::Div<&Vector<T>> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn div(self, rhs: &Vector<T>) -> Vector<T> {
            return &self / rhs;
        }
    }

    impl<T> ops::Div<T> for &Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn div(self, rhs: T) -> Vector<T> {
            Vector {
                position: [
                    self.position[0] / rhs,
                    self.position[1] / rhs,
                    self.position[2] / rhs,
                ],
            }
        }
    }

    impl<T> ops::Div<T> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn div(self, rhs: T) -> Vector<T> {
            return &self / rhs;
        }
    }

    impl<T> ops::Mul<&Vector<T>> for &Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn mul(self, rhs: &Vector<T>) -> Vector<T> {
            Vector {
                position: [
                    self.position[0] * rhs.position[0],
                    self.position[1] * rhs.position[1],
                    self.position[2] * rhs.position[2],
                ],
            }
        }
    }

    impl<T> ops::Mul<Vector<T>> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn mul(self, rhs: Vector<T>) -> Vector<T> {
            return &self * &rhs;
        }
    }

    impl<T> ops::Mul<&Vector<T>> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn mul(self, rhs: &Vector<T>) -> Vector<T> {
            return &self * rhs;
        }
    }

    impl<T> ops::Mul<T> for &Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn mul(self, rhs: T) -> Vector<T> {
            Vector {
                position: [
                    self.position[0] * rhs,
                    self.position[1] * rhs,
                    self.position[2] * rhs,
                ],
            }
        }
    }

    impl<T> ops::Mul<T> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn mul(self, rhs: T) -> Vector<T> {
            return &self * rhs;
        }
    }

    impl ops::Mul<Vector<f32>> for f32 {
        type Output = Vector<f32>;
        fn mul(self, rhs: Vector<f32>) -> Vector<f32> {
            return rhs * self;
        }
    }

    impl ops::Mul<&Vector<f32>> for f32 {
        type Output = Vector<f32>;
        fn mul(self, rhs: &Vector<f32>) -> Vector<f32> {
            return rhs * self;
        }
    }

    impl ops::Mul<Vector<f64>> for f64 {
        type Output = Vector<f64>;
        fn mul(self, rhs: Vector<f64>) -> Vector<f64> {
            return rhs * self;
        }
    }

    impl ops::Mul<&Vector<f64>> for f64 {
        type Output = Vector<f64>;
        fn mul(self, rhs: &Vector<f64>) -> Vector<f64> {
            return rhs * self;
        }
    }

    impl<T> ops::AddAssign<Vector<T>> for Vector<T>
    where
        T: Float,
    {
        fn add_assign(&mut self, rhs: Self) {
            *self = self.clone() + rhs;
        }
    }

    impl<T> ops::SubAssign<Vector<T>> for Vector<T>
    where
        T: Float,
    {
        fn sub_assign(&mut self, rhs: Self) {
            *self = self.clone() - rhs;
        }
    }

    impl<T> ops::MulAssign<Vector<T>> for Vector<T>
    where
        T: Float,
    {
        fn mul_assign(&mut self, rhs: Self) {
            *self = self.clone() * rhs;
        }
    }

    impl<T> ops::MulAssign<T> for Vector<T>
    where
        T: Float,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self = self.clone() * rhs;
        }
    }

    impl<T> ops::DivAssign<Vector<T>> for Vector<T>
    where
        T: Float,
    {
        fn div_assign(&mut self, rhs: Self) {
            *self = self.clone() / rhs;
        }
    }

    impl<T> ops::DivAssign<T> for Vector<T>
    where
        T: Float,
    {
        fn div_assign(&mut self, rhs: T) {
            *self = self.clone() / rhs;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::{Vector, VectorOperations};
    #[test]
    fn f32_constructor() {
        let vec = Vector::<f32>::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);
    }
    #[test]
    fn f64_constructor() {
        let vec = Vector::<f64>::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);
    }
    #[test]
    fn eq() {
        let vec_1 = Vector::<f32>::new(1.0, 2.0, 3.0);
        let vec_2 = Vector::<f32>::new(1.0, 2.0, 3.0);
        assert_eq!(&vec_1, &vec_2);
    }
    #[test]
    fn add() {
        let vec_1 = Vector::<f32>::new(1.0, 2.0, 3.0);
        let vec_2 = Vector::<f32>::new(1.0, 2.0, 3.0);

        let vec3 = vec_1 + vec_2;
        assert_eq!(vec3, Vector::<f32>::new(2.0, 4.0, 6.0));
    }
    #[test]
    fn add_assign() {
        let mut vec_1 = Vector::<f32>::new(1.0, 2.0, 3.0);
        vec_1 += Vector::<f32>::new(1.0, 2.0, 3.0);

        assert_eq!(vec_1, Vector::<f32>::new(2.0, 4.0, 6.0));
    }
    #[test]
    fn sub() {
        let vec_1 = Vector::<f32>::new(1.0, 2.0, 3.0);
        let vec_2 = Vector::<f32>::new(1.0, 2.0, 3.0);

        let vec3 = vec_1 - vec_2;
        assert_eq!(vec3, Vector::<f32>::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn sub_assign() {
        let mut vec_1 = Vector::<f32>::new(1.0, 2.0, 3.0);
        vec_1 -= Vector::<f32>::new(1.0, 2.0, 3.0);

        assert_eq!(vec_1, Vector::<f32>::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn div_vec() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        let vec_2 = Vector::<f32>::new(1.0, 2.0, 3.0);

        let vec3 = vec_1 / vec_2;
        assert_eq!(vec3, Vector::<f32>::new(10.0, 10.0, 10.0));
    }
    #[test]
    fn div_vec_assign() {
        let mut vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        vec_1 /= Vector::<f32>::new(1.0, 2.0, 3.0);

        assert_eq!(vec_1, Vector::<f32>::new(10.0, 10.0, 10.0));
    }
    #[test]
    fn div_t() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(vec_1 / 10.0, Vector::<f32>::new(1.0, 2.0, 3.0));
    }
    #[test]
    fn div_t_assign() {
        let mut vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        vec_1 /= 10.0;
        assert_eq!(vec_1, Vector::<f32>::new(1.0, 2.0, 3.0));
    }
    #[test]
    fn mul_vec() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        let vec_2 = Vector::<f32>::new(2.0, 2.0, 2.0);
        assert_eq!(vec_1 * vec_2, Vector::<f32>::new(20.0, 40.0, 60.0));
    }
    #[test]
    fn mul_vec_assign() {
        let mut vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        vec_1 *= Vector::<f32>::new(2.0, 2.0, 2.0);
        assert_eq!(vec_1, Vector::<f32>::new(20.0, 40.0, 60.0));
    }
    #[test]
    fn mul_t_rhs() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(vec_1 * 2.0, Vector::<f32>::new(20.0, 40.0, 60.0));
    }
    #[test]
    fn mul_t_lhs() {
        let vec_1 = Vector::new(10.0f32, 20.0f32, 30.0f32);
        assert_eq!(2.0 * vec_1, Vector::new(20.0, 40.0, 60.0));

        let vec_2 = Vector::new(10f64, 20f64, 30f64);
        assert_eq!(2.0 * vec_2, Vector::new(20.0, 40.0, 60.0));

        let vec_3 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(2.0 * &vec_3, Vector::new(20.0, 40.0, 60.0));

        let vec_4 = Vector::new(10.0f64, 20.0f64, 30.0f64);
        assert_eq!(2.0 * &vec_4, Vector::new(20.0, 40.0, 60.0));
    }
    #[test]
    fn mul_t_assign() {
        let mut vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        vec_1 *= 2.0;
        assert_eq!(vec_1, Vector::<f32>::new(20.0, 40.0, 60.0));
    }
    #[test]
    fn length() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(vec_1.length(), 37.416573);
    }
    #[test]
    fn length_squared() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(vec_1.length_squared(), 1400.0);
    }
    #[test]
    fn cross() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        let vec_2 = Vector::<f32>::new(5.0, 23.0, 6.0);

        assert_eq!(vec_1.cross(&vec_2), Vector::<f32>::new(-570.0, 90.0, 130.0));
    }
    #[test]
    fn make_unit() {
        let mut vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(
            *vec_1.make_unit(),
            Vector::<f32>::new(0.26726124, 0.5345225, 0.80178374)
        );
    }
    #[test]
    fn unit_vector() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(
            vec_1.unit_vector(),
            Vector::<f32>::new(0.26726124, 0.5345225, 0.80178374)
        );
    }
    #[test]
    fn dot() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        let vec_2 = Vector::<f32>::new(2.0, 2.0, 2.0);
        assert_eq!(vec_1.dot(&vec_2), 120.0);
    }
    #[test]
    fn random() {
        let vec_1 = Vector::<f32>::random();
        assert!(vec_1.x() > 0f32 && vec_1.x() < 1_f32);
        assert!(vec_1.y() > 0f32 && vec_1.y() < 1_f32);
        assert!(vec_1.z() > 0f32 && vec_1.z() < 1_f32);
    }
    #[test]
    fn random_range() {
        let vec_1 = Vector::random_range(-1f64, 1f64);
        assert!(vec_1.x() > -1f64 && vec_1.x() < 1_f64);
        assert!(vec_1.y() > -1f64 && vec_1.y() < 1_f64);
        assert!(vec_1.z() > -1f64 && vec_1.z() < 1_f64);
    }
    #[test]
    fn reflect() {
        let vec_1 = Vector::new(3.5, 7.0, 10.5);
        let vec_2 = Vector::new(2.0, 1.5, 6.0);
        assert_eq!(vec_1.reflect(&vec_2), Vector::new(-318.5, -234.5, -955.5));
    }
    #[test]
    fn refract() {
        let vec_1 = Vector::new(1.0, 2.0, 3.0);
        let vec_2 = Vector::new(2.0, 1.5, 1.0);
        assert_eq!(
            vec_1.refract(&vec_2, 1.0/1.5).unwrap(),
            Vector::new(-15.92548878632739, -18.38806555481852, -20.850642323309653)
        );
    }
}
