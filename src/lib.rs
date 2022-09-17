pub mod vector {
    use num::Float;
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
            self.squared_length().sqrt()
        }

        fn squared_length(&self) -> T {
            self.position[0].powi(2) + self.position[1].powi(2) + self.position[2].powi(2)
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
                    self.position[0] / k,
                    self.position[1] / k,
                    self.position[2] / k,
                ],
            }
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
                    self.position[0] * rhs.position[1],
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

    // TODO: Add support for T * Vector<T>
    impl<T> ops::Mul<T> for Vector<T>
    where
        T: Float,
    {
        type Output = Vector<T>;
        fn mul(self, rhs: T) -> Vector<T> {
            return &self * rhs;
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
        let vec_2 = Vector::<f32>::new(2.0, 3.0, 4.0);
        assert_eq!(vec_1.dot(&vec_2), 200.0);
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
            Vector::<f32>::new(374.16574, 748.3315, 1122.4972)
        );
    }

    #[test]
    fn unit_vector() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(
            vec_1.unit_vector(),
            Vector::<f32>::new(374.16574, 748.3315, 1122.4972)
        );
    }

    #[test]
    fn dot() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(
            vec_1.unit_vector(),
            Vector::<f32>::new(374.16574, 748.3315, 1122.4972)
        );
    }
}
