use core::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Neg,Sub,SubAssign};
use super::vector::*;
use super::trigonometry::{sin,cos,tan};

#[derive(Debug,Clone,Copy)]
pub struct Mat<const ROW:usize,const COL:usize>{pub data:[[f32;ROW];COL]}

pub type Mat3=Mat<3,3>;
pub type Mat4=Mat<4,4>;

impl<const ROW:usize,const COL:usize>Mat<ROW,COL>{
    /// Creates matrix from array.
    #[inline]
    pub fn new(data:[[f32;ROW];COL])->Self{
        Self{data}
    }

    ///Creates matrix of zeros.
    #[inline]
    pub fn zero()->Self{
        Self{data:[[0.0;ROW];COL]}
    }

    /// Creates a matrix filled with the given scalar value.
    #[inline]
    pub fn fill(val:f32)->Self{
        Self{data:[[val;ROW];COL]}
    }

    /// Retrieves element at the given index (row, col).
    #[inline]
    pub fn get(&self,row:usize,col:usize)->f32{
        debug_assert!(row<ROW&&col<COL);
        self.data[col][row]
    }

    /// Retrieves a column at the given index (row, col) as an array.
    #[inline]
    pub fn get_col(&self,col:usize)->[f32;ROW]{
        debug_assert!(col<COL);
        self.data[col]
    }

    /// Retrieves a row at the given index (row, col) as an array.
    #[inline]
    pub fn get_row(&self,row:usize)->[f32;COL]{
        debug_assert!(row<ROW);
        let mut res:[f32;COL]=[0.0;COL];
        for col in 0..COL{res[col]=self.data[col][row];}res
    }

    /// Retrieves the entire matrix as a generic nested array.
    #[inline]
    pub fn get_all(&self)->[[f32;ROW];COL]{
        self.data
    }

    /// Sets element at the given index (row, col) to the given value.
    #[inline]
    pub fn set(&mut self,row:usize,col:usize,x:f32){
        debug_assert!(row<ROW&&col<COL);
        self.data[col][row]=x
    }

    /// Sets a column at the given index (row, col) to the given array.
    #[inline]
    pub fn set_col(&mut self,col:usize,arr:[f32;ROW]){
        debug_assert!(col<COL);
        self.data[col]=arr;
    }

    /// Sets a column at the given index (row, col) to the given array.
    #[inline]
    pub fn set_row(&mut self,row:usize,arr:[f32;COL]){
        debug_assert!(row<ROW);
        for x in 0..COL{
            self.data[x][row]=arr[x];
        }
    }

    /// Shifts all elements in the matrix to the given value.
    #[inline]
    pub fn set_all(&mut self,x:f32){
        for col in 0..COL{
            for row in 0..ROW{
                self.data[col][row]=x;
            }
        }
    }

    /// Shifts a single element at the given index (row, col) by the given value.
    #[inline]
    pub fn shift(&mut self,row:usize,col:usize,val:f32){
        debug_assert!(row<ROW&&col<COL);
        self.data[col][row]+=val
    }

    /// Shifts a column at the given index (row, col) the given array.
    #[inline]
    pub fn shift_col(&mut self,col:usize,arr:[f32;ROW]){
        debug_assert!(col<COL);
        for row in 0..ROW{
            self.data[col][row]+=arr[row];
        }
    }

    /// Shifts a row at the given index (row, col) by the given array.
    #[inline]
    pub fn shift_row(&mut self,row:usize,arr:[f32;COL]){
        debug_assert!(row<ROW);
        for col in 0..COL{
            self.data[col][row]+=arr[col]
        }
    }

    /// Shifts all elements in the matrix by the given value.
    #[inline]
    pub fn shift_all(&mut self,x:f32){
        for col in 0..COL{
            for row in 0..ROW{
                self.data[col][row]+=x;
            }
        }
    }

    /// Returns the transpose of the matrix.
    #[inline]
    pub fn transpose(&self)->Mat<COL,ROW>{
        let mut rcl:Mat<COL,ROW>=Mat::<COL,ROW>::zero();
        for col in 0..COL{for row in 0..ROW{rcl.data[row][col]=self.data[col][row]}}rcl
    }

}

impl<const N:usize>Mat<N,N>{
    /// Creates an identity matrix (1.0 on the diagonal, 0.0 everywhere else).
    #[inline]
    pub fn identity()->Self{
        let mut rcl:Mat<N,N>=Self::zero();
        for i in 0..N{rcl.data[i][i]=1.0;}rcl
    }

    /// Transposes the square matrix in place.
    #[inline]
    pub fn transposed(&mut self){
        for col in 0..N{
            for row in (col+1)..N{
                let rcl:f32=self.data[col][row];
                self.data[col][row]=self.data[row][col];
                self.data[row][col]=rcl;
            }
        }
    }

    /// Calculates the trace (sum of diagonal elements) of the matrix.
    #[inline]
    pub fn trace(&self)->f32{
        let mut sum:f32=0.0;
        for i in 0..N{sum+=self.data[i][i]}sum
    }
}

impl Mat<4,4>{
    /// Creates a 4x4 translation matrix.
    #[inline]
    pub fn translation(x:f32,y:f32,z:f32)->Self{
        let mut res=Self::identity();
        res.data[3][0]=x;
        res.data[3][1]=y;
        res.data[3][2]=z;
        res
    }

    /// Creates a 4x4 scaling matrix.
    #[inline]
    pub fn scale(x:f32,y:f32,z:f32)->Self{
        let mut res=Self::identity();
        res.data[0][0]=x;
        res.data[1][1]=y;
        res.data[2][2]=z;
        res
    }

    #[inline]
    pub fn rotate_x(angle:f32)->Self{
        let mut res:Mat<4,4>=Self::identity();
        let c:f32=cos(angle);
        let s:f32=sin(angle);
        res.data[1][1]=c;
        res.data[1][2]=s;
        res.data[2][1]=-s;
        res.data[2][2]=c;
        res
    }

    /// Creates a 4x4 rotation matrix around the Y axis.
    #[inline]
    pub fn rotate_y(angle:f32)->Self{
        let mut res:Mat<4,4>=Self::identity();
        let c:f32=cos(angle);
        let s:f32=sin(angle);
        res.data[0][0]=c;
        res.data[0][2]=s;
        res.data[2][0]=-s;
        res.data[2][2]=c;
        res
    }

    /// Creates a perspective projection matrix (standard OpenGL depth).
    #[inline]
    pub fn persp(fov_y:f32,aspect:f32,near:f32,far:f32)->Self{
        let mut res:Mat<4,4>=Self::zero();
        let fov_y:f32=1.0/tan(fov_y/2.0);
        res.data[0][0]=fov_y/aspect;
        res.data[1][1]=fov_y;
        res.data[2][2]=(far+near)/(near-far);
        res.data[2][3]=-1.0;
        res.data[3][2]=(2.0*far*near)/(near-far);
        res
    }

    /// Creates a view matrix that looks at the target from the camera position.
    #[inline]
    pub fn view(pos:Vec3,target:Vec3,global_up:Vec3)->Self{
        let forward:Vec3=(pos-target).normalized();
        let right:Vec3=global_up.cross(forward).normalized();
        let up:Vec3=forward.cross(right);
        let tx:f32=-right.dot(pos);
        let ty:f32=-up.dot(pos);
        let tz:f32=-forward.dot(pos);
        Self{data:[[right.x,up.x,forward.x,0.0],[right.y,up.y,forward.y,0.0],[right.z,up.z,forward.z,0.0],[tx,ty,tz,1.0]]}
    }
}

impl<const ROW:usize,const COL:usize>Add<Mat<ROW,COL>>for Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn add(self,rhs:Mat<ROW,COL>)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{for row in 0..ROW{res.data[col][row]=self.data[col][row]+rhs.data[col][row]}}res
    }
}

impl<const ROW:usize,const COL:usize>Sub<Mat<ROW,COL>>for Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn sub(self,rhs:Mat<ROW,COL>)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{for row in 0..ROW{res.data[col][row]=self.data[col][row]-rhs.data[col][row]}}res
    }
}

impl<const ROW:usize,const COL:usize>Mul<f32>for Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn mul(self,rhs:f32)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{
            for row in 0..ROW{res.data[col][row]=self.data[col][row]*rhs}}res
    }
}

impl<const ROW1:usize,const COL1:usize,const COL2:usize>Mul<Mat<COL1,COL2>>for Mat<ROW1,COL1>{
    type Output=Mat<ROW1,COL2>;

    #[inline]
    fn mul(self,rhs:Mat<COL1,COL2>)->Self::Output{
        let mut res:Mat<ROW1,COL2>=Mat::<ROW1,COL2>::zero();
        for row in 0..ROW1{
            for col in 0..COL2{
                let mut sum:f32=0.0;
                for i in 0..COL1{sum+=self.data[i][row]*rhs.data[col][i];}res.data[col][row]=sum
            }
        }
        res
    }
}

impl<const ROW:usize,const COL:usize>Div<f32>for Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn div(self,rhs:f32)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{for row in 0..ROW{res.data[col][row]=self.data[col][row]/rhs}}res
    }
}

impl<const ROW:usize,const COL:usize>Neg for Mat<ROW,COL>{
    type Output=Mat<ROW,COL>;

    #[inline]
    fn neg(self)->Self::Output{
        let mut res:Mat<ROW,COL>=Mat::<ROW,COL>::zero();
        for col in 0..COL{for row in 0..ROW{res.data[col][row]=-self.data[col][row]}}res
    }
}

impl<const ROW:usize,const COL:usize>AddAssign<Mat<ROW,COL>>for Mat<ROW,COL>{
    #[inline]
    fn add_assign(&mut self,rhs:Mat<ROW,COL>){
        for col in 0..COL{for row in 0..ROW{self.data[col][row]+=rhs.data[col][row]}}
    }
}

impl<const ROW:usize,const COL:usize>SubAssign<Mat<ROW,COL>>for Mat<ROW,COL>{
    #[inline]
    fn sub_assign(&mut self,rhs:Mat<ROW,COL>){
        for col in 0..COL{for row in 0..ROW{self.data[col][row]-=rhs.data[col][row]}}
    }
}

impl<const ROW:usize,const COL:usize>MulAssign<f32>for Mat<ROW,COL>{
    #[inline]
    fn mul_assign(&mut self,rhs:f32){
        for col in 0..COL{for row in 0..ROW{self.data[col][row]*=rhs}}
    }
}

impl<const ROW:usize,const COL:usize>DivAssign<f32>for Mat<ROW,COL>{
    #[inline]
    fn div_assign(&mut self,rhs:f32){
        for col in 0..COL{for row in 0..ROW{self.data[col][row]/=rhs}}
    }
}

impl Mul<Vec4>for Mat<4,4>{
    type Output=Vec4;

    #[inline]
    fn mul(self,vec:Vec4)->Self::Output{
        Vec4{
            x:self.data[0][0]*vec.x+self.data[1][0]*vec.y+self.data[2][0]*vec.z+self.data[3][0]*vec.w,
            y:self.data[0][1]*vec.x+self.data[1][1]*vec.y+self.data[2][1]*vec.z+self.data[3][1]*vec.w,
            z:self.data[0][2]*vec.x+self.data[1][2]*vec.y+self.data[2][2]*vec.z+self.data[3][2]*vec.w,
            w:self.data[0][3]*vec.x+self.data[1][3]*vec.y+self.data[2][3]*vec.z+self.data[3][3]*vec.w,
        }
    }
}

