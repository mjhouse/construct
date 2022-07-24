use crate::geometry::*;
use std::ops::Mul;

type Data = [f64;16];

#[derive(Default,Debug,Copy,Clone)]
pub struct Matrix {
    data: Data,
}

#[derive(Debug,Copy,Clone)]
pub enum MatrixType {
    Scale,
    Rotate,
    Translate,
}

impl Matrix {

    pub fn new<T: Into<f64> + Copy>(v: [T;16]) -> Self {
        Self {
            data: [
                 v[0].into(),  v[1].into(),  v[2].into(),  v[3].into(),
                 v[4].into(),  v[5].into(),  v[6].into(),  v[7].into(),
                 v[8].into(),  v[9].into(), v[10].into(), v[11].into(),
                v[12].into(), v[13].into(), v[14].into(), v[15].into(),
            ]
        }
    }

    pub fn unpack(&self) -> [f64;16] {
        self.data
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        Self::new([
            x,   0.0, 0.0, 0.0,
            0.0,   y, 0.0, 0.0,
            0.0, 0.0,   z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn translate(x: f64, y: f64, z: f64) -> Self {
        Self::new([
            1.0, 0.0, 0.0,   x,
            0.0, 1.0, 0.0,   y,
            0.0, 0.0, 1.0,   z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn rotate_x(x: f64) -> Self {
        let cosx = x.cos();
        let sinx = x.sin();
        Self::new([
            1.0,  0.0,   0.0, 0.0,
            0.0, cosx, -sinx, 0.0,
            0.0, sinx,  cosx, 0.0,
            0.0,  0.0,   0.0, 1.0,
        ])
    }

    pub fn rotate_y(y: f64) -> Self {
        let cosy = y.cos();
        let siny = y.sin();
        Self::new([
            cosy, 0.0, siny, 0.0,
             0.0, 1.0,  0.0, 0.0,
           -siny, 0.0, cosy, 0.0,
             0.0, 0.0,  0.0, 1.0,
        ])
    }

    pub fn rotate_z(z: f64) -> Self {
        let cosz = z.cos();
        let sinz = z.sin();
        Self::new([
            cosz, -sinz, 0.0, 0.0,
            sinz,  cosz, 0.0, 0.0,
             0.0,   0.0, 1.0, 0.0,
             0.0,   0.0, 0.0, 1.0,
        ])
    }

    pub fn rotate(x: f64, y: f64, z: f64) -> Self {
        Self::rotate_x(x) *
        Self::rotate_y(y) * 
        Self::rotate_z(z)
    }

    pub fn matching(v: MatrixType, x: f64, y: f64, z: f64) -> Self {
        match v {
            MatrixType::Scale => Self::scale(x,y,z),
            MatrixType::Rotate => Self::rotate(x,y,z),
            MatrixType::Translate => Self::translate(x,y,z),
        }
    }

}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let [
            a11, a12, a13, a14,
            a21, a22, a23, a24,
            a31, a32, a33, a34,
            a41, a42, a43, a44
        ] = self.unpack();

        let [
            b11, b12, b13, b14,
            b21, b22, b23, b24,
            b31, b32, b33, b34,
            b41, b42, b43, b44
        ] = rhs.unpack();

        let m11 = a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41;
        let m12 = a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42;
        let m13 = a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43;
        let m14 = a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44;

        let m21 = a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41;
        let m22 = a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42;
        let m23 = a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43;
        let m24 = a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44;

        let m31 = a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41;
        let m32 = a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42;
        let m33 = a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43;
        let m34 = a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44;

        let m41 = a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41;
        let m42 = a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42;
        let m43 = a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43;
        let m44 = a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44;

        Self::new([
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44
        ])
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::geometry::*;

    macro_rules! fassert_eq {
        ( $v: expr, $e: expr ) => {
            assert_relative_eq!($v,$e, epsilon = f64::EPSILON);
        }
    }

    #[test]
    fn test_rotation_matrix() {
        let a = Matrix::rotate(2.0,4.0,6.0);

        let [
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44
        ] = a.unpack();

        // need to double check this result in a 
        // third party tool
        fassert_eq!(m11, -0.6276091828117975);
        fassert_eq!(m12, -0.18263815796815594);
        fassert_eq!(m13, -0.7568024953079282);
        fassert_eq!(m14, 0.0);
        fassert_eq!(m21, -0.5444715276934526);
        fassert_eq!(m22, -0.5918539946650849);
        fassert_eq!(m23, 0.5943564625123038);
        fassert_eq!(m24, 0.0);
        fassert_eq!(m31, -0.556468749510218);
        fassert_eq!(m32, 0.7850809845187823);
        fassert_eq!(m33, 0.27201172505161186);
        fassert_eq!(m34, 0.0);
        fassert_eq!(m41, 0.0);
        fassert_eq!(m42, 0.0);
        fassert_eq!(m43, 0.0);
        fassert_eq!(m44, 1.0);

    }

    #[test]
    fn test_mul_matrices() {
        let a = Matrix::new([
             1,  2,  3,  4,
             5,  6,  7,  8,
             9, 10, 11, 12,
            13, 14, 15, 16,
        ]);

        let b = Matrix::new([
            16, 15, 14, 13,
            12, 11, 10,  9,
             8,  7,  6,  5,
             4,  3,  2,  1,
        ]);

        let c = a * b;

        let [
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44
        ] = c.unpack();

        fassert_eq!(m11,  80.0);
        fassert_eq!(m12,  70.0);
        fassert_eq!(m13,  60.0);
        fassert_eq!(m14,  50.0);
        fassert_eq!(m21, 240.0);
        fassert_eq!(m22, 214.0);
        fassert_eq!(m23, 188.0);
        fassert_eq!(m24, 162.0);
        fassert_eq!(m31, 400.0);
        fassert_eq!(m32, 358.0);
        fassert_eq!(m33, 316.0);
        fassert_eq!(m34, 274.0);
        fassert_eq!(m41, 560.0);
        fassert_eq!(m42, 502.0);
        fassert_eq!(m43, 444.0);
        fassert_eq!(m44, 386.0);
    }

    #[test]
    fn test_square_matrices() {
        let a = Matrix::new([
             1,  2,  3,  4,
             5,  6,  7,  8,
             9, 10, 11, 12,
            13, 14, 15, 16,
        ]);

        let c = a * a;

        let [
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44
        ] = c.unpack();

        fassert_eq!(m11,  90.0);
        fassert_eq!(m12, 100.0);
        fassert_eq!(m13, 110.0);
        fassert_eq!(m14, 120.0);
        fassert_eq!(m21, 202.0);
        fassert_eq!(m22, 228.0);
        fassert_eq!(m23, 254.0);
        fassert_eq!(m24, 280.0);
        fassert_eq!(m31, 314.0);
        fassert_eq!(m32, 356.0);
        fassert_eq!(m33, 398.0);
        fassert_eq!(m34, 440.0);
        fassert_eq!(m41, 426.0);
        fassert_eq!(m42, 484.0);
        fassert_eq!(m43, 542.0);
        fassert_eq!(m44, 600.0);
    }

    #[test]
    fn test_scaling_matrix() {
        // vertex data to transform
        let vertices = vec![
            Vertex::new(0.0,0.0,0.0),
            Vertex::new(3.0,0.0,0.0),
            Vertex::new(0.0,2.0,0.0),
        ];

        // a face that references the data
        let mut face = Face::new(1,2,3);

        // x, y, and z scaling factors
        let x = 1.123;
        let y = 1.123;
        let z = 1.123;

        // a scaling matrix
        let m = Matrix::scale(x,y,z);

        let mut triangle = face.triangle(&vertices);
        triangle.transform(&m);

        fassert_eq!(triangle.p1.x,0.0);
        fassert_eq!(triangle.p1.y,0.0);
        fassert_eq!(triangle.p1.z,0.0);
        fassert_eq!(triangle.p2.x,3.3689999999999998);
        fassert_eq!(triangle.p2.y,0.0);
        fassert_eq!(triangle.p2.z,0.0);
        fassert_eq!(triangle.p3.x,0.0);
        fassert_eq!(triangle.p3.y,2.246);
        fassert_eq!(triangle.p3.z,0.0);
    }

    #[test]
    fn test_rotate_x() {
        // vertex data to transform
        let mut vertex = Vertex::new(1.0,2.0,3.0);

        // a rotation matrix with angle in radians
        let matrix = Matrix::rotate_x(1.0);

        vertex.transform(&matrix);

        // using: https://keisan.casio.com/exec/system/15362817755710
        fassert_eq!(vertex.y,-1.4438083426874098);
        fassert_eq!(vertex.z,3.3038488872202123);
    }

    #[test]
    fn test_rotate_y() {
        // vertex data to transform
        let mut vertex = Vertex::new(1.0,2.0,3.0);

        // a rotation matrix with angle in radians
        let matrix = Matrix::rotate_y(1.0);

        vertex.transform(&matrix);

        // using: https://keisan.casio.com/exec/system/15362817755710
        fassert_eq!(vertex.x,3.064715260291829);
        fassert_eq!(vertex.z,0.7794359327965228);
    }

    #[test]
    fn test_rotate_z() {
        // vertex data to transform
        let mut vertex = Vertex::new(1.0,2.0,3.0);

        // a rotation matrix with angle in radians
        let matrix = Matrix::rotate_z(1.0);

        vertex.transform(&matrix);

        // using: https://keisan.casio.com/exec/system/15362817755710
        fassert_eq!(vertex.x,-1.1426396637476532);
        fassert_eq!(vertex.y,1.922075596544176);
    }

    #[test]
    fn test_rotate_series() {
        // vertex data to transform
        let mut vertex = Vertex::new(1.0,2.0,3.0);

        // a rotation matrix with angle in radians
        let a = Matrix::rotate_x(1.0);
        let b = Matrix::rotate_y(1.0);
        let c = Matrix::rotate_z(1.0);

        vertex.transform(&a);
        vertex.transform(&b);
        vertex.transform(&c);

        // using: https://keisan.casio.com/exec/system/15362817755710
        fassert_eq!(vertex.x,3.008940055606576);
        fassert_eq!(vertex.y,2.013923311660526);
        fassert_eq!(vertex.z,0.9436061871970718);
    }

    #[test]
    fn test_rotate_combined() {
        // vertex data to transform
        let mut vertex = Vertex::new(1.0,2.0,3.0);

        // a rotation matrix with angle in radians
        let a = Matrix::rotate_x(1.0);
        let b = Matrix::rotate_y(1.0);
        let c = Matrix::rotate_z(1.0);

        let m = a * b * c;

        vertex.transform(&m);

        // using: https://keisan.casio.com/exec/system/15362817755710
        fassert_eq!(vertex.x,1.9070421093244363);
        fassert_eq!(vertex.y,-1.134517035937589);
        fassert_eq!(vertex.z,3.0126502432958917);
    }

    #[test]
    fn test_rotate_combined_shortcut() {
        // vertex data to transform
        let mut vertex = Vertex::new(1.0,2.0,3.0);

        // a rotation matrix with angle in radians
        let m = Matrix::rotate(1.0,1.0,1.0);

        vertex.transform(&m);

        // using: https://keisan.casio.com/exec/system/15362817755710
        fassert_eq!(vertex.x,1.9070421093244363);
        fassert_eq!(vertex.y,-1.134517035937589);
        fassert_eq!(vertex.z,3.0126502432958917);
    }

}