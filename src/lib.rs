pub mod vector {
    // use num::Float;
    use std::ops;
    #[derive(Clone, PartialEq, Debug)]
    pub struct Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        position: [T; 3],
    }

    #[allow(dead_code)]
    impl<T> Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
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

        pub fn length(&self) -> f64 {
            0.0
        }

        fn squared_length(&self) -> T {
            self.position[0] * self.position[0]
                + self.position[1] * self.position[1]
                + self.position[2] * self.position[2]
        }
    }

    pub trait VectorOperations<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        fn cross(&self, rhs: &Vector<T>) -> Vector<T>;
        fn dot(&self, rhs: &Vector<T>) -> Vector<T>;
        // fn unit(&self) -> &Vector) -> Vector;
        // fn length(&self) -> f32;
        // fn squared_length(&self) -> f32;
        // fn make_unit(&mut self) -> &mut Vector;
        // fn unit_vector(vec: &Vector) -> Vector;
    }

    impl<T> VectorOperations<T> for Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
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
        fn dot(&self, rhs: &Vector<T>) -> Vector<T> {
            Vector {
                position: [
                    self.position[0] * rhs.position[0],
                    self.position[1] * rhs.position[1],
                    self.position[2] * rhs.position[2],
                ],
            }
        }
        //     fn length(&self) -> f32 {
        //         self.squared_length().sqrt()
        //     }
        //     fn squared_length(&self) -> f32 {
        //         self.position[0].powi(2) + self.position[1].powi(2) + self.position[2].powi(2)
        //     }
        //     fn make_unit(&mut self) -> &mut Vector {
        //         let k = 1.0 / self.length();
        //         (*self).position[0] /= k;
        //         (*self).position[1] /= k;
        //         (*self).position[2] /= k;
        //         self
        //     }
        //     fn unit_vector(vec: &Vector) -> Vector {
        //         vec.clone().make_unit().clone()
        //     }
    }

    impl<T> ops::Add<&Vector<T>> for &Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
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
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn add(self, rhs: &Vector<T>) -> Vector<T> {
            return &self + rhs;
        }
    }

    impl<T> ops::Add<Vector<T>> for Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn add(self, rhs: Vector<T>) -> Vector<T> {
            return &self + &rhs;
        }
    }

    impl<T> ops::Sub<&Vector<T>> for &Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
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
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn sub(self, rhs: &Vector<T>) -> Vector<T> {
            return &self + rhs;
        }
    }

    impl<T> ops::Sub<Vector<T>> for Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn sub(self, rhs: Vector<T>) -> Vector<T> {
            return &self + &rhs;
        }
    }

    impl<T> ops::Div<&Vector<T>> for &Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
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
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn div(self, rhs: Vector<T>) -> Vector<T> {
            return &self / &rhs;
        }
    }

    impl<T> ops::Div<&Vector<T>> for Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn div(self, rhs: &Vector<T>) -> Vector<T> {
            return &self / rhs;
        }
    }

    impl<T> ops::Div<T> for &Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
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
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn div(self, rhs: T) -> Vector<T> {
            return &self / rhs;
        }
    }

    impl<T> ops::Mul<&Vector<T>> for &Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
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
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn mul(self, rhs: Vector<T>) -> Vector<T> {
            return &self * &rhs;
        }
    }

    impl<T> ops::Mul<&Vector<T>> for Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn mul(self, rhs: &Vector<T>) -> Vector<T> {
            return &self * rhs;
        }
    }

    impl<T> ops::Mul<T> for &Vector<T>
    where
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
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
        T: Copy
            + ops::Add<Output = T>
            + ops::Sub<Output = T>
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Neg<Output = T>,
    {
        type Output = Vector<T>;
        fn mul(self, rhs: T) -> Vector<T> {
            return &self * rhs;
        }
    }

    // impl ops::AddAssign for Vector {
    //     fn add_assign(&mut self, rhs: Self) {
    //         *self = self.clone() + rhs;
    //     }
    // }

    // impl ops::SubAssign for Vector {
    //     fn sub_assign(&mut self, rhs: Self) {
    //         *self = self.clone() - rhs;
    //     }
    // }

    // impl ops::MulAssign for Vector {
    //     fn mul_assign(&mut self, rhs: Self) {
    //         *self = self.clone() * rhs;
    //     }
    // }

    // impl ops::DivAssign for Vector {
    //     fn div_assign(&mut self, rhs: Self) {
    //         *self = self.clone() / rhs;
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::vector::{Vector, VectorOperations};

    #[test]
    fn f32_constructor() {
        let vec: Vector<f32> = Vector::<f32>::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);
    }

    #[test]
    fn eq() {
        let vec_1: Vector<f32> = Vector::<f32>::new(1.0, 2.0, 3.0);
        let vec_2: Vector<f32> = Vector::<f32>::new(1.0, 2.0, 3.0);
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
    fn sub() {
        let vec_1 = Vector::<f32>::new(1.0, 2.0, 3.0);
        let vec_2 = Vector::<f32>::new(1.0, 2.0, 3.0);

        let vec3 = vec_1 - vec_2;
        assert_eq!(vec3, Vector::<f32>::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn div_vec() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        let vec_2 = Vector::<f32>::new(1.0, 2.0, 3.0);

        let vec3 = vec_1 / vec_2;
        assert_eq!(vec3, Vector::<f32>::new(10.0, 10.0, 10.0));
    }

    #[test]
    fn div_t() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(vec_1 / 10.0, Vector::<f32>::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn mul_vec() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        let vec_2 = Vector::<f32>::new(2.0, 2.0, 2.0);
        assert_eq!(vec_1 * vec_2, Vector::<f32>::new(20.0, 40.0, 60.0));
    }

    #[test]
    fn mul_t() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        assert_eq!(vec_1 * 2.0, Vector::<f32>::new(20.0, 40.0, 60.0));
    }

    #[test]
    fn cross() {
        let vec_1 = Vector::<f32>::new(10.0, 20.0, 30.0);
        let vec_2 = Vector::<f32>::new(5.0, 23.0, 6.0);

        assert_eq!(vec_1.cross(&vec_2), Vector::<f32>::new(-570.0, 90.0, 130.0));
    }
}
